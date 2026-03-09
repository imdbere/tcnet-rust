//! TCNet packet types.
//!
//! This module contains parsers and builders for various TCNet packet types.

mod app_data;
mod error;
mod metadata;
mod metrics;
mod mixer;
mod opt_in;
mod request;
mod status;
mod time;

pub use app_data::AppDataPacket;
pub use app_data::{compute_auth_xor_key, RESOLUME_ARENA_TRAILER, RESOLUME_SIGNATURE};
pub use error::{ErrorCode, ErrorNotificationPacket, ERROR_NOTIFICATION_PACKET_SIZE};
pub use metadata::{MetadataPacket, TrackKey, METADATA_DATA_TYPE, METADATA_PACKET_SIZE};
pub use metrics::{MetricsDataPacket, METRICS_DATA_TYPE, METRICS_DATA_PACKET_SIZE};
pub use mixer::{
    MixerChannel, MixerCrossfader, MixerDataPacket, MixerEffects, MixerFilter, MixerMaster,
    MixerMonitor, MixerType, MIXER_DATA_TYPE,
};
pub use opt_in::{OptInBuilder, OptInPacket};
pub use request::{RequestDataType, RequestPacket, REQUEST_PACKET_SIZE};
pub use status::{LayerStatus, StatusPacket};
pub use time::{LayerTimeData, TimePacket};

use crate::error::{Result, TcNetError};
use crate::header::ManagementHeader;
use crate::types::{MessageType, MANAGEMENT_HEADER_SIZE};

/// A parsed TCNet packet.
#[derive(Debug, Clone)]
pub enum Packet {
    /// Opt-IN packet for network participation
    OptIn(OptInPacket),
    /// Opt-OUT packet for leaving network
    OptOut(ManagementHeader),
    /// Status packet with layer configuration
    Status(StatusPacket),
    /// Time packet with layer timing data
    Time(TimePacket),
    /// Error/notification packet (response to failed requests)
    ErrorNotification(ErrorNotificationPacket),
    /// Metrics data packet with playback info
    MetricsData(MetricsDataPacket),
    /// Metadata packet with track info
    Metadata(MetadataPacket),
    /// Mixer data packet with mixer state
    MixerData(MixerDataPacket),
    /// Unknown or unsupported packet type (header only)
    Unknown(ManagementHeader),
}

impl Packet {
    /// Parse a packet from raw bytes.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < MANAGEMENT_HEADER_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: MANAGEMENT_HEADER_SIZE,
                actual: data.len(),
            });
        }

        let header = ManagementHeader::parse(data)?;

        match header.message_type {
            MessageType::OptIn => Ok(Packet::OptIn(OptInPacket::parse(data)?)),
            MessageType::OptOut => Ok(Packet::OptOut(header)),
            MessageType::Status => Ok(Packet::Status(StatusPacket::parse(data)?)),
            MessageType::Time => Ok(Packet::Time(TimePacket::parse(data)?)),
            MessageType::ErrorNotification => {
                Ok(Packet::ErrorNotification(ErrorNotificationPacket::parse(data)?))
            }
            MessageType::Data => Self::parse_data_packet(data, header),
            _ => Ok(Packet::Unknown(header)),
        }
    }

    /// Parse a Data packet (type 200) based on its DataType field.
    fn parse_data_packet(data: &[u8], header: ManagementHeader) -> Result<Self> {
        // DataType is at byte 24
        if data.len() < 25 {
            return Ok(Packet::Unknown(header));
        }

        let data_type = data[24];

        match data_type {
            METRICS_DATA_TYPE => Ok(Packet::MetricsData(MetricsDataPacket::parse(data)?)),
            METADATA_DATA_TYPE => Ok(Packet::Metadata(MetadataPacket::parse(data)?)),
            MIXER_DATA_TYPE => Ok(Packet::MixerData(MixerDataPacket::parse(data)?)),
            _ => Ok(Packet::Unknown(header)),
        }
    }

    /// Get the header of this packet.
    pub fn header(&self) -> &ManagementHeader {
        match self {
            Packet::OptIn(p) => &p.header,
            Packet::OptOut(h) => h,
            Packet::Status(p) => &p.header,
            Packet::Time(p) => &p.header,
            Packet::ErrorNotification(p) => &p.header,
            Packet::MetricsData(p) => &p.header,
            Packet::Metadata(p) => &p.header,
            Packet::MixerData(p) => &p.header,
            Packet::Unknown(h) => h,
        }
    }
}
