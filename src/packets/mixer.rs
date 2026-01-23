//! Mixer Data Packet (Type 200, DataType 150) parsing.
//!
//! Mixer Data packets contain real-time mixer state including channel levels,
//! EQ settings, effects, and crossfader position.

use std::io::Cursor;

use binrw::BinRead;
use tracing::info;

use crate::error::{Result, TcNetError};
use crate::header::ManagementHeader;
use crate::wire::{RawMixerChannel, RawMixerDataPacket};

/// Data type identifier for Mixer Data packets.
pub const MIXER_DATA_TYPE: u8 = 150;

/// Minimum packet size for Mixer Data.
pub const MIXER_DATA_PACKET_SIZE: usize = 270;

/// Mixer type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MixerType {
    /// Standard mixer
    #[default]
    Standard = 0,
    /// Extended mixer
    Extended = 2,
}

impl MixerType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Standard,
            2 => Self::Extended,
            _ => Self::Standard,
        }
    }
}

/// Channel data from a mixer.
#[derive(Debug, Clone, Default)]
pub struct MixerChannel {
    /// Channel number (1-6)
    pub number: u8,
    /// Source select
    pub source_select: u8,
    /// Audio level (0-255)
    pub audio_level: u8,
    /// Fader level (0-255)
    pub fader_level: u8,
    /// Trim/gain level (0-255)
    pub trim_level: u8,
    /// Compressor level (0-255)
    pub comp_level: u8,
    /// EQ Hi level (0-255, 128 = center)
    pub eq_hi: u8,
    /// EQ Hi Mid level (0-255, 128 = center)
    pub eq_hi_mid: u8,
    /// EQ Low Mid level (0-255, 128 = center)
    pub eq_low_mid: u8,
    /// EQ Low level (0-255, 128 = center)
    pub eq_low: u8,
    /// Filter/Color knob (0-255, 128 = center)
    pub filter_color: u8,
    /// Send level to effects (0-255)
    pub send: u8,
    /// CUE A button state
    pub cue_a: u8,
    /// CUE B button state
    pub cue_b: u8,
    /// Crossfader assign (0=A, 1=THRU, 2=B typically)
    pub crossfader_assign: u8,
}

impl MixerChannel {
    fn from_raw(raw: &RawMixerChannel, number: u8) -> Self {
        Self {
            number,
            source_select: raw.source_select,
            audio_level: raw.audio_level,
            fader_level: raw.fader_level,
            trim_level: raw.trim_level,
            comp_level: raw.comp_level,
            eq_hi: raw.eq_hi,
            eq_hi_mid: raw.eq_hi_mid,
            eq_low_mid: raw.eq_low_mid,
            eq_low: raw.eq_low,
            filter_color: raw.filter_color,
            send: raw.send,
            cue_a: raw.cue_a,
            cue_b: raw.cue_b,
            crossfader_assign: raw.crossfader_assign,
        }
    }

    /// Check if the fader is up (above threshold).
    pub fn is_fader_up(&self, threshold: u8) -> bool {
        self.fader_level > threshold
    }

    /// Check if CUE is active (either A or B).
    pub fn is_cue_active(&self) -> bool {
        self.cue_a > 0 || self.cue_b > 0
    }
}

/// Master section data from a mixer.
#[derive(Debug, Clone, Default)]
pub struct MixerMaster {
    /// Master audio level (0-255)
    pub audio_level: u8,
    /// Master fader level (0-255)
    pub fader_level: u8,
    /// Master filter (0-255)
    pub filter: u8,
    /// Master CUE A
    pub cue_a: u8,
    /// Master CUE B
    pub cue_b: u8,
    /// Master isolator on/off
    pub isolator_on: bool,
    /// Master isolator Hi (0-255)
    pub isolator_hi: u8,
    /// Master isolator Mid (0-255)
    pub isolator_mid: u8,
    /// Master isolator Low (0-255)
    pub isolator_low: u8,
}

/// Effects section data from a mixer.
#[derive(Debug, Clone, Default)]
pub struct MixerEffects {
    /// Send FX effect type
    pub send_fx_effect: u8,
    /// Send FX level (0-255)
    pub send_fx_level: u8,
    /// Send FX time (0-255)
    pub send_fx_time: u8,
    /// Send FX HPF (0-255)
    pub send_fx_hpf: u8,
    /// Send FX size/feedback (0-255)
    pub send_fx_size_feedback: u8,
    /// Beat FX on/off
    pub beat_fx_on: bool,
    /// Beat FX level/depth (0-255)
    pub beat_fx_level: u8,
    /// Beat FX channel select
    pub beat_fx_channel: u8,
    /// Beat FX select
    pub beat_fx_select: u8,
    /// Beat FX frequency Hi (0-255)
    pub beat_fx_freq_hi: u8,
    /// Beat FX frequency Mid (0-255)
    pub beat_fx_freq_mid: u8,
    /// Beat FX frequency Low (0-255)
    pub beat_fx_freq_low: u8,
}

/// Filter section data from a mixer.
#[derive(Debug, Clone, Default)]
pub struct MixerFilter {
    /// High-pass filter (0-255)
    pub hpf: u8,
    /// Low-pass filter (0-255)
    pub lpf: u8,
    /// Resonance (0-255)
    pub resonance: u8,
}

/// Crossfader data from a mixer.
#[derive(Debug, Clone, Default)]
pub struct MixerCrossfader {
    /// Crossfader position (0-255, 0=A, 128=center, 255=B)
    pub position: u8,
    /// Crossfader curve (0-2 typically)
    pub curve: u8,
    /// Channel fader curve (0-2 typically)
    pub channel_curve: u8,
}

/// Headphones/booth data from a mixer.
#[derive(Debug, Clone, Default)]
pub struct MixerMonitor {
    /// Headphones pre-EQ
    pub headphones_pre_eq: u8,
    /// Headphones A level
    pub headphones_a_level: u8,
    /// Headphones A cue/master mix
    pub headphones_a_mix: u8,
    /// Headphones B level
    pub headphones_b_level: u8,
    /// Headphones B cue/master mix
    pub headphones_b_mix: u8,
    /// Booth level
    pub booth_level: u8,
    /// Booth EQ Hi
    pub booth_eq_hi: u8,
    /// Booth EQ Low
    pub booth_eq_low: u8,
}

/// Mixer Data Packet containing complete mixer state.
#[derive(Debug, Clone)]
pub struct MixerDataPacket {
    /// Common management header
    pub header: ManagementHeader,
    /// Mixer ID
    pub mixer_id: u8,
    /// Mixer type
    pub mixer_type: MixerType,
    /// Mixer name
    pub mixer_name: String,
    /// Master section
    pub master: MixerMaster,
    /// Effects section
    pub effects: MixerEffects,
    /// Filter section
    pub filter: MixerFilter,
    /// Crossfader section
    pub crossfader: MixerCrossfader,
    /// Monitor/headphones section
    pub monitor: MixerMonitor,
    /// Channel data (6 channels)
    pub channels: [MixerChannel; 6],
}

impl MixerDataPacket {
    /// Parse a mixer data packet from bytes.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < MIXER_DATA_PACKET_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: MIXER_DATA_PACKET_SIZE,
                actual: data.len(),
            });
        }

        // Debug: dump key byte ranges
        info!(
            "Mixer packet raw bytes len: {}:\n\
             {:02x?}",
             data.len(),
            &data
        );

        let mut cursor = Cursor::new(data);
        let raw = RawMixerDataPacket::read(&mut cursor)
            .map_err(|e| TcNetError::ParseError(e.to_string()))?;

        // Verify data type
        if raw.data_type != MIXER_DATA_TYPE {
            return Err(TcNetError::ParseError(format!(
                "expected data type {}, got {}",
                MIXER_DATA_TYPE, raw.data_type
            )));
        }

        let header = ManagementHeader::from_raw(raw.header)?;

        // Build channel array
        let mut channels: [MixerChannel; 6] = Default::default();
        for i in 0..6 {
            channels[i] = MixerChannel::from_raw(&raw.channels[i], (i + 1) as u8);
        }

        Ok(Self {
            header,
            mixer_id: raw.mixer_id,
            mixer_type: MixerType::from_u8(raw.mixer_type),
            mixer_name: raw.mixer_name.to_string_lossy(),
            master: MixerMaster {
                audio_level: raw.master_audio_level,
                fader_level: raw.master_fader_level,
                filter: raw.master_filter,
                cue_a: raw.master_cue_a,
                cue_b: raw.master_cue_b,
                isolator_on: raw.master_isolator_on != 0,
                isolator_hi: raw.master_isolator_hi,
                isolator_mid: raw.master_isolator_mid,
                isolator_low: raw.master_isolator_low,
            },
            effects: MixerEffects {
                send_fx_effect: raw.send_fx_effect,
                send_fx_level: raw.send_fx_level,
                send_fx_time: raw.send_fx_time,
                send_fx_hpf: raw.send_fx_hpf,
                send_fx_size_feedback: raw.send_fx_size_feedback,
                beat_fx_on: raw.beat_fx_on != 0,
                beat_fx_level: raw.beat_fx_level,
                beat_fx_channel: raw.beat_fx_channel,
                beat_fx_select: raw.beat_fx_select,
                beat_fx_freq_hi: raw.beat_fx_freq_hi,
                beat_fx_freq_mid: raw.beat_fx_freq_mid,
                beat_fx_freq_low: raw.beat_fx_freq_low,
            },
            filter: MixerFilter {
                hpf: raw.filter_hpf,
                lpf: raw.filter_lpf,
                resonance: raw.filter_resonance,
            },
            crossfader: MixerCrossfader {
                position: raw.crossfader,
                curve: raw.crossfader_curve,
                channel_curve: raw.channel_fader_curve,
            },
            monitor: MixerMonitor {
                headphones_pre_eq: raw.headphones_pre_eq,
                headphones_a_level: raw.headphones_a_level,
                headphones_a_mix: raw.headphones_a_mix,
                headphones_b_level: raw.headphones_b_level,
                headphones_b_mix: raw.headphones_b_mix,
                booth_level: raw.booth_level,
                booth_eq_hi: raw.booth_eq_hi,
                booth_eq_low: raw.booth_eq_low,
            },
            channels,
        })
    }

    /// Get a channel by number (1-6).
    pub fn channel(&self, num: u8) -> Option<&MixerChannel> {
        if num >= 1 && num <= 6 {
            Some(&self.channels[(num - 1) as usize])
        } else {
            None
        }
    }

    /// Get channels with faders up.
    pub fn active_channels(&self, threshold: u8) -> impl Iterator<Item = &MixerChannel> {
        self.channels.iter().filter(move |c| c.is_fader_up(threshold))
    }
}
