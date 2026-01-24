//! Metrics Data Packet (Data Type 2).
//!
//! Contains real-time playback metrics for a layer including position, BPM,
//! beat information, and speed data.

use std::io::Cursor;

use binrw::BinRead;

use crate::error::{Result, TcNetError};
use crate::header::ManagementHeader;
use crate::types::{Layer, LayerState};
use crate::wire::RawMetricsDataPacket;

/// Data type identifier for Metrics Data packets.
pub const METRICS_DATA_TYPE: u8 = 2;

/// Minimum packet size for Metrics Data.
pub const METRICS_DATA_PACKET_SIZE: usize = 122;

/// Metrics Data packet containing real-time playback information for a layer.
#[derive(Debug, Clone)]
pub struct MetricsDataPacket {
    /// Common management header
    pub header: ManagementHeader,
    /// Layer this data is for
    pub layer: Layer,
    /// Current layer state (playing, paused, etc.)
    pub layer_state: LayerState,
    /// Whether this layer is the sync master
    pub is_sync_master: bool,
    /// Current beat marker (1-4)
    pub beat_marker: u8,
    /// Total track length in milliseconds
    pub track_length_ms: u32,
    /// Current playback position in milliseconds
    pub current_position_ms: u32,
    /// Playback speed (0-65536, where 32768 = 100%)
    pub speed: u32,
    /// Current beat number in the track
    pub beat_number: u32,
    /// BPM value (stored as hundredths, e.g., 12000 = 120.00 BPM)
    pub bpm_raw: u32,
    /// Pitch bend value (0-65536, where 32768 = 100%)
    pub pitch_bend: u16,
    /// Track ID (database ID of loaded track)
    pub track_id: u32,
}

impl MetricsDataPacket {
    /// Parse a Metrics Data packet from raw bytes.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < METRICS_DATA_PACKET_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: METRICS_DATA_PACKET_SIZE,
                actual: data.len(),
            });
        }

        let mut cursor = Cursor::new(data);
        let raw = RawMetricsDataPacket::read(&mut cursor)
            .map_err(|e| TcNetError::ParseError(e.to_string()))?;

        let header = ManagementHeader::from_raw(raw.header)?;

        // Layer ID is 1-based in the protocol
        let layer = Layer::from_index((raw.layer_id.saturating_sub(1)) as usize)
            .unwrap_or(Layer::Layer1);

        Ok(Self {
            header,
            layer,
            layer_state: LayerState::from_u8(raw.layer_state),
            is_sync_master: raw.sync_master != 0,
            beat_marker: raw.beat_marker,
            track_length_ms: raw.track_length_ms,
            current_position_ms: raw.current_position_ms,
            speed: raw.speed,
            beat_number: raw.beat_number,
            bpm_raw: raw.bpm,
            pitch_bend: raw.pitch_bend,
            track_id: raw.track_id,
        })
    }

    /// Get the BPM as a floating-point value.
    pub fn bpm(&self) -> f64 {
        self.bpm_raw as f64 / 100.0
    }

    /// Get the playback speed as a percentage (100.0 = normal speed).
    pub fn speed_percent(&self) -> f64 {
        (self.speed as f64 / 32768.0) * 100.0
    }

    /// Get the pitch bend as a percentage (100.0 = no bend).
    pub fn pitch_bend_percent(&self) -> f64 {
        (self.pitch_bend as f64 / 32768.0) * 100.0
    }

    /// Get the current position as a formatted time string (MM:SS.mmm).
    pub fn position_string(&self) -> String {
        let total_secs = self.current_position_ms / 1000;
        let millis = self.current_position_ms % 1000;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}.{:03}", mins, secs, millis)
    }

    /// Get the track length as a formatted time string (MM:SS).
    pub fn track_length_string(&self) -> String {
        let total_secs = self.track_length_ms / 1000;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    /// Get the remaining time in milliseconds.
    pub fn remaining_ms(&self) -> u32 {
        self.track_length_ms.saturating_sub(self.current_position_ms)
    }

    /// Get progress through the track as a percentage (0.0 - 100.0).
    pub fn progress_percent(&self) -> f64 {
        if self.track_length_ms == 0 {
            return 0.0;
        }
        (self.current_position_ms as f64 / self.track_length_ms as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bpm_conversion() {
        let packet = MetricsDataPacket {
            header: ManagementHeader::new(
                1,
                crate::types::MessageType::Data,
                "TEST",
                0,
                crate::types::NodeType::Master,
                crate::types::NodeOptions::new(),
                0,
            ),
            layer: Layer::Layer1,
            layer_state: LayerState::Playing,
            is_sync_master: true,
            beat_marker: 1,
            track_length_ms: 180000,
            current_position_ms: 60000,
            speed: 32768,
            beat_number: 100,
            bpm_raw: 12800, // 128.00 BPM
            pitch_bend: 32768,
            track_id: 42,
        };

        assert!((packet.bpm() - 128.0).abs() < 0.01);
        assert!((packet.speed_percent() - 100.0).abs() < 0.01);
        assert!((packet.progress_percent() - 33.33).abs() < 0.1);
    }
}
