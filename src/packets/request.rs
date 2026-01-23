//! Request Packet (Type 20) for requesting data from other nodes.
//!
//! Request packets are sent to master or repeater nodes to request specific data
//! such as waveforms, metadata, or mixer state.

use crate::header::ManagementHeader;
use crate::types::{MessageType, NodeOptions, NodeType};

/// Request packet size (26 bytes).
pub const REQUEST_PACKET_SIZE: usize = 26;

/// Data types that can be requested.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RequestDataType {
    /// Layer status data (type 2)
    LayerStatus = 2,
    /// Metadata (type 4)
    Metadata = 4,
    /// Beat grid info (type 8)
    BeatGrid = 8,
    /// Cue data (type 12)
    CueData = 12,
    /// Small waveform (type 16)
    SmallWaveform = 16,
    /// Big waveform (type 32)
    BigWaveform = 32,
    /// Mixer data (type 150)
    MixerData = 150,
}

impl RequestDataType {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Request packet for requesting data from other nodes.
#[derive(Debug, Clone)]
pub struct RequestPacket {
    /// Common management header
    pub header: ManagementHeader,
    /// Data type to request
    pub data_type: RequestDataType,
    /// Layer to request data for (0 for mixer data)
    pub layer: u8,
}

impl RequestPacket {
    /// Create a new request packet.
    pub fn new(
        node_id: u16,
        node_name: &str,
        sequence: u8,
        node_type: NodeType,
        node_options: NodeOptions,
        timestamp_us: u32,
        data_type: RequestDataType,
        layer: u8,
    ) -> Self {
        let header = ManagementHeader::new(
            node_id,
            MessageType::Request,
            node_name,
            sequence,
            node_type,
            node_options,
            timestamp_us,
        );

        Self {
            header,
            data_type,
            layer,
        }
    }

    /// Create a request for mixer data.
    pub fn mixer_data(
        node_id: u16,
        node_name: &str,
        sequence: u8,
        node_type: NodeType,
        node_options: NodeOptions,
        timestamp_us: u32,
    ) -> Self {
        Self::new(
            node_id,
            node_name,
            sequence,
            node_type,
            node_options,
            timestamp_us,
            RequestDataType::MixerData,
            0, // Layer doesn't matter for mixer data
        )
    }

    /// Serialize the packet to bytes.
    pub fn to_bytes(&self) -> [u8; REQUEST_PACKET_SIZE] {
        let mut buf = [0u8; REQUEST_PACKET_SIZE];

        // Management header (bytes 0-23)
        buf[..24].copy_from_slice(&self.header.to_bytes());

        // Data type (byte 24)
        buf[24] = self.data_type.as_u8();

        // Layer (byte 25)
        buf[25] = self.layer;

        buf
    }
}
