//! Status Packet (Type 5) parsing.
//!
//! Status packets contain layer configuration and state information.

use std::io::Cursor;

use binrw::BinRead;

use crate::error::{Result, TcNetError};
use crate::header::ManagementHeader;
use crate::types::{Layer, LayerState, SmpteMode};
use crate::wire::RawStatusPacket;

/// Layer status information from a Status packet.
#[derive(Debug, Clone, Default)]
pub struct LayerStatus {
    /// Layer identifier
    pub layer: Layer,
    /// Source layer number
    pub source: u8,
    /// Layer playback state
    pub status: LayerState,
    /// Track ID loaded on this layer
    pub track_id: u32,
    /// Layer name (16 chars max)
    pub name: String,
}

/// Status packet containing layer configuration.
#[derive(Debug, Clone)]
pub struct StatusPacket {
    /// Common management header
    pub header: ManagementHeader,
    /// Number of nodes registered
    pub node_count: u16,
    /// Listener port for unicast
    pub listener_port: u16,
    /// SMPTE mode
    pub smpte_mode: SmpteMode,
    /// Auto master mode (0=Disabled, 1=HTP Master, 2=Link Master)
    pub auto_master_mode: u8,
    /// Layer status for all 8 layers
    pub layers: [LayerStatus; 8],
}

impl StatusPacket {
    /// Minimum size for a status packet.
    const MIN_SIZE: usize = 300;

    /// Parse a status packet from bytes using binrw.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < Self::MIN_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: Self::MIN_SIZE,
                actual: data.len(),
            });
        }

        let mut cursor = Cursor::new(data);
        let raw = RawStatusPacket::read(&mut cursor)
            .map_err(|e| TcNetError::ParseError(e.to_string()))?;

        let header = ManagementHeader::from_raw(raw.header)?;

        // Build layer status array
        let mut layers: [LayerStatus; 8] = Default::default();
        for i in 0..8 {
            layers[i] = LayerStatus {
                layer: Layer::from_index(i).unwrap_or(Layer::Layer1),
                source: raw.layer_sources[i],
                status: LayerState::from_u8(raw.layer_statuses[i]),
                track_id: raw.layer_track_ids[i],
                name: raw.layer_names[i].to_string_lossy(),
            };
        }

        Ok(Self {
            header,
            node_count: raw.node_count,
            listener_port: raw.listener_port,
            smpte_mode: SmpteMode::from_u8(raw.smpte_mode),
            auto_master_mode: raw.auto_master_mode,
            layers,
        })
    }

    /// Get auto master mode as a descriptive string.
    pub fn auto_master_mode_str(&self) -> &'static str {
        match self.auto_master_mode {
            0 => "Disabled",
            1 => "HTP Master",
            2 => "Link Master",
            _ => "Unknown",
        }
    }

    /// Get active layers (non-idle).
    pub fn active_layers(&self) -> impl Iterator<Item = &LayerStatus> {
        self.layers.iter().filter(|l| l.status != LayerState::Idle)
    }
}
