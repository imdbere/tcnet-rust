//! Time Packet (Type 254) parsing.
//!
//! Time packets contain real-time timing data for up to 8 layers.

use std::io::Cursor;

use binrw::BinRead;

use crate::error::{Result, TcNetError};
use crate::header::ManagementHeader;
use crate::types::{Layer, LayerState, SmpteMode, Timecode, TimecodeState, TIME_PACKET_MIN_SIZE, TIME_PACKET_SIZE};
use crate::wire::{RawOnAirData, RawTimePacket};

/// Timing data for a single layer.
#[derive(Debug, Clone, Default)]
pub struct LayerTimeData {
    /// Layer identifier
    pub layer: Layer,
    /// Current time in milliseconds
    pub time_ms: u32,
    /// Total track time in milliseconds
    pub total_time_ms: u32,
    /// Beat marker position (0=unknown, 1-4=position)
    pub beat_marker: u8,
    /// Layer playback state
    pub state: LayerState,
    /// SMPTE mode for this layer (0 = use general)
    pub smpte_mode: SmpteMode,
    /// Timecode state
    pub tc_state: TimecodeState,
    /// SMPTE timecode
    pub timecode: Timecode,
    /// On-air state / fader position (0-255)
    pub on_air: u8,
}

impl LayerTimeData {
    /// Check if this layer is actively producing timecode.
    pub fn is_active(&self) -> bool {
        self.state.is_active() || self.tc_state == TimecodeState::Running
    }

    /// Get remaining time in milliseconds.
    pub fn remaining_ms(&self) -> u32 {
        self.total_time_ms.saturating_sub(self.time_ms)
    }

    /// Get progress as a percentage (0.0 - 100.0).
    pub fn progress_percent(&self) -> f64 {
        if self.total_time_ms == 0 {
            0.0
        } else {
            (self.time_ms as f64 / self.total_time_ms as f64) * 100.0
        }
    }
}

/// Time Packet containing timing data for all layers.
#[derive(Debug, Clone)]
pub struct TimePacket {
    /// Common management header
    pub header: ManagementHeader,
    /// General SMPTE mode
    pub smpte_mode: SmpteMode,
    /// Layer timing data (8 layers)
    pub layers: [LayerTimeData; 8],
}

impl TimePacket {
    /// Parse a time packet from bytes using binrw.
    /// Supports both V3.3.3+ (162 bytes with on-air) and older (154 bytes without on-air).
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < TIME_PACKET_MIN_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: TIME_PACKET_MIN_SIZE,
                actual: data.len(),
            });
        }

        let mut cursor = Cursor::new(data);
        let raw = RawTimePacket::read(&mut cursor)
            .map_err(|e| TcNetError::ParseError(e.to_string()))?;

        // Try to read optional on-air data (V3.3.3+)
        let on_air = if data.len() >= TIME_PACKET_SIZE {
            RawOnAirData::read(&mut cursor)
                .map(|d| d.on_air)
                .unwrap_or([0u8; 8])
        } else {
            [0u8; 8]
        };

        let header = ManagementHeader::from_raw(raw.header)?;
        let general_smpte = SmpteMode::from_u8(raw.smpte_mode);

        // Build layer data array
        let mut layers: [LayerTimeData; 8] = Default::default();
        for i in 0..8 {
            let tc = &raw.layer_timecodes[i];
            let layer_smpte = if tc.smpte_mode == 0 {
                general_smpte
            } else {
                SmpteMode::from_u8(tc.smpte_mode)
            };

            layers[i] = LayerTimeData {
                layer: Layer::from_index(i).unwrap_or(Layer::Layer1),
                time_ms: raw.layer_times[i],
                total_time_ms: raw.layer_total_times[i],
                beat_marker: raw.beat_markers[i],
                state: LayerState::from_u8(raw.layer_states[i]),
                smpte_mode: layer_smpte,
                tc_state: TimecodeState::from_u8(tc.tc_state),
                timecode: Timecode::new(tc.hours, tc.minutes, tc.seconds, tc.frames),
                on_air: on_air[i],
            };
        }

        Ok(Self {
            header,
            smpte_mode: general_smpte,
            layers,
        })
    }

    /// Get only the active layers (playing or running timecode).
    pub fn active_layers(&self) -> impl Iterator<Item = &LayerTimeData> {
        self.layers.iter().filter(|l| l.is_active())
    }

    /// Get a specific layer by identifier.
    pub fn layer(&self, layer: Layer) -> &LayerTimeData {
        &self.layers[layer.index()]
    }
}
