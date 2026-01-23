//! Wire-format structs for binary packet parsing using binrw.
//!
//! These structs map directly to the binary layout of TCNet packets.
//! They are converted to the higher-level types in `crate::header` and `crate::packets`.

use binrw::{binread, BinRead};

/// Fixed-size string helper for binrw.
/// Reads N bytes and stores them as a fixed array.
#[derive(Debug, Clone, Copy)]
pub struct FixedStr<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for FixedStr<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> BinRead for FixedStr<N> {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let mut buf = [0u8; N];
        reader.read_exact(&mut buf)?;
        Ok(Self(buf))
    }
}

impl<const N: usize> FixedStr<N> {
    /// Convert to a trimmed String.
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0)
            .trim_end_matches('\0')
            .trim()
            .to_string()
    }

    /// Get the raw bytes.
    pub fn as_bytes(&self) -> &[u8; N] {
        &self.0
    }
}

// ============================================================================
// Management Header (24 bytes)
// ============================================================================

/// Raw management header as it appears on the wire.
#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct RawManagementHeader {
    /// Node ID (bytes 0-1)
    pub node_id: u16,
    /// Protocol version major (byte 2)
    pub version_major: u8,
    /// Protocol version minor (byte 3)
    pub version_minor: u8,
    /// Header signature "TCN" (bytes 4-6)
    #[br(assert(header_sig == [b'T', b'C', b'N'], "invalid TCNet header signature"))]
    pub header_sig: [u8; 3],
    /// Message type (byte 7)
    pub message_type: u8,
    /// Node name (bytes 8-15)
    pub node_name: FixedStr<8>,
    /// Sequence number (byte 16)
    pub sequence: u8,
    /// Node type (byte 17)
    pub node_type: u8,
    /// Node options flags (bytes 18-19)
    pub node_options: u16,
    /// Timestamp in microseconds (bytes 20-23)
    pub timestamp_us: u32,
}

// ============================================================================
// Opt-IN Packet (68 bytes)
// ============================================================================

/// Raw Opt-IN packet as it appears on the wire.
#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct RawOptInPacket {
    /// Management header (bytes 0-23)
    pub header: RawManagementHeader,
    /// Number of nodes registered (bytes 24-25)
    pub node_count: u16,
    /// Listener port (bytes 26-27)
    pub listener_port: u16,
    /// Uptime in seconds (bytes 28-29)
    pub uptime_secs: u16,
    /// Reserved (bytes 30-31)
    #[br(pad_after = 0)]
    pub _reserved1: u16,
    /// Vendor name (bytes 32-47)
    pub vendor_name: FixedStr<16>,
    /// Application name (bytes 48-63)
    pub app_name: FixedStr<16>,
    /// App version major (byte 64)
    pub app_version_major: u8,
    /// App version minor (byte 65)
    pub app_version_minor: u8,
    /// App version bug (byte 66)
    pub app_version_bug: u8,
    /// Reserved (byte 67)
    pub _reserved2: u8,
}

// ============================================================================
// Status Packet (300 bytes)
// ============================================================================

/// Raw Status packet as it appears on the wire.
#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct RawStatusPacket {
    /// Management header (bytes 0-23)
    pub header: RawManagementHeader,
    /// Number of nodes registered (bytes 24-25)
    pub node_count: u16,
    /// Listener port (bytes 26-27)
    pub listener_port: u16,
    /// Reserved (bytes 28-33)
    #[br(count = 6)]
    pub _reserved1: Vec<u8>,
    /// Layer sources (bytes 34-41)
    pub layer_sources: [u8; 8],
    /// Layer statuses (bytes 42-49)
    pub layer_statuses: [u8; 8],
    /// Layer track IDs (bytes 50-81)
    pub layer_track_ids: [u32; 8],
    /// Reserved (byte 82)
    pub _reserved2: u8,
    /// SMPTE mode (byte 83)
    pub smpte_mode: u8,
    /// Auto master mode (byte 84)
    pub auto_master_mode: u8,
    /// Reserved (bytes 85-99)
    #[br(count = 15)]
    pub _reserved3: Vec<u8>,
    /// App-specific data (bytes 100-171)
    #[br(count = 72)]
    pub _app_specific: Vec<u8>,
    /// Layer names (bytes 172-299, 16 bytes each)
    pub layer_names: [FixedStr<16>; 8],
}

// ============================================================================
// Time Packet (154-162 bytes)
// ============================================================================

/// Per-layer SMPTE/timecode data (6 bytes per layer).
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RawLayerTimecode {
    /// SMPTE mode (0 = use general)
    pub smpte_mode: u8,
    /// Timecode state
    pub tc_state: u8,
    /// Hours
    pub hours: u8,
    /// Minutes
    pub minutes: u8,
    /// Seconds
    pub seconds: u8,
    /// Frames
    pub frames: u8,
}

/// Raw Time packet as it appears on the wire.
/// Note: on_air field is optional (added in V3.3.3).
#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct RawTimePacket {
    /// Management header (bytes 0-23)
    pub header: RawManagementHeader,
    /// Layer times in ms (bytes 24-55)
    pub layer_times: [u32; 8],
    /// Layer total times in ms (bytes 56-87)
    pub layer_total_times: [u32; 8],
    /// Beat markers (bytes 88-95)
    pub beat_markers: [u8; 8],
    /// Layer states (bytes 96-103)
    pub layer_states: [u8; 8],
    /// Reserved (byte 104)
    pub _reserved: u8,
    /// General SMPTE mode (byte 105)
    pub smpte_mode: u8,
    /// Per-layer timecode data (bytes 106-153)
    pub layer_timecodes: [RawLayerTimecode; 8],
    // Note: on_air (bytes 154-161) is optional and handled separately
}

/// Optional on-air data for V3.3.3+ time packets.
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RawOnAirData {
    pub on_air: [u8; 8],
}

// ============================================================================
// Mixer Data Packet (270 bytes) - Data Type 150
// ============================================================================

/// Raw mixer channel data (14 data bytes + 10 reserved = 24 bytes per channel).
#[binread]
#[br(little)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RawMixerChannel {
    /// Source select
    pub source_select: u8,
    /// Audio level (0-255)
    pub audio_level: u8,
    /// Fader level (0-255)
    pub fader_level: u8,
    /// Trim level (0-255)
    pub trim_level: u8,
    /// Compressor level (0-255)
    pub comp_level: u8,
    /// EQ Hi level (0-255)
    pub eq_hi: u8,
    /// EQ Hi Mid level (0-255)
    pub eq_hi_mid: u8,
    /// EQ Low Mid level (0-255)
    pub eq_low_mid: u8,
    /// EQ Low level (0-255)
    pub eq_low: u8,
    /// Filter/Color (0-255)
    pub filter_color: u8,
    /// Send level (0-255)
    pub send: u8,
    /// CUE A (0-255)
    pub cue_a: u8,
    /// CUE B (0-255)
    pub cue_b: u8,
    /// Crossfader assign (0-255)
    pub crossfader_assign: u8,
    /// Reserved (10 bytes)
    pub _reserved: [u8; 10],
}

// ============================================================================
// Request Packet (26 bytes)
// ============================================================================

/// Raw Request packet as it appears on the wire (26 bytes).
/// Used to request data from other nodes.
#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct RawRequestPacket {
    /// Management header (bytes 0-23)
    pub header: RawManagementHeader,
    /// Data type to request (byte 24)
    pub data_type: u8,
    /// Layer to request data for (byte 25)
    pub layer: u8,
}

// ============================================================================
// Mixer Data Packet (270 bytes) - Data Type 150
// ============================================================================

/// Raw Mixer Data packet as it appears on the wire (270 bytes).
/// This is a TCNet Data Packet (type 200) with DataType 150.
#[binread]
#[br(little)]
#[derive(Debug, Clone)]
pub struct RawMixerDataPacket {
    /// Management header (bytes 0-23)
    pub header: RawManagementHeader,
    /// Data type (byte 24) - should be 150 for Mixer Data
    pub data_type: u8,
    /// Mixer ID (byte 25)
    pub mixer_id: u8,
    /// Mixer type (byte 26) - 0=Standard, 2=Extended
    pub mixer_type: u8,
    /// Reserved (bytes 27-28)
    #[br(count = 2)]
    pub _reserved1: Vec<u8>,
    /// Mixer name (bytes 29-44)
    pub mixer_name: FixedStr<16>,
    /// Reserved (bytes 45-56)
    #[br(count = 12)]
    pub _reserved2: Vec<u8>,
    /// Reserved for Mic 1-2 Level (bytes 57-58)
    #[br(count = 2)]
    pub _reserved_mic: Vec<u8>,
    /// Mic EQ Hi (byte 59)
    pub mic_eq_hi: u8,
    /// Mic EQ Low (byte 60)
    pub mic_eq_low: u8,
    /// Master audio level (byte 61)
    pub master_audio_level: u8,
    /// Master fader level (byte 62)
    pub master_fader_level: u8,
    /// Reserved (bytes 63-66)
    #[br(count = 4)]
    pub _reserved3: Vec<u8>,
    /// Link CUE A (byte 67)
    pub link_cue_a: u8,
    /// Link CUE B (byte 68)
    pub link_cue_b: u8,
    /// Master filter (byte 69)
    pub master_filter: u8,
    /// Reserved (byte 70)
    pub _reserved4: u8,
    /// Master CUE A (byte 71)
    pub master_cue_a: u8,
    /// Master CUE B (byte 72)
    pub master_cue_b: u8,
    /// Reserved (byte 73)
    pub _reserved5: u8,
    /// Master isolator on/off (byte 74)
    pub master_isolator_on: u8,
    /// Master isolator Hi (byte 75)
    pub master_isolator_hi: u8,
    /// Master isolator Mid (byte 76)
    pub master_isolator_mid: u8,
    /// Master isolator Low (byte 77)
    pub master_isolator_low: u8,
    /// Reserved (byte 78)
    pub _reserved6: u8,
    /// Filter HPF (byte 79)
    pub filter_hpf: u8,
    /// Filter LPF (byte 80)
    pub filter_lpf: u8,
    /// Filter resonance (byte 81)
    pub filter_resonance: u8,
    /// Reserved (bytes 82-83)
    #[br(count = 2)]
    pub _reserved7: Vec<u8>,
    /// Send FX effect (byte 84)
    pub send_fx_effect: u8,
    /// Send FX Ext 1 (byte 85)
    pub send_fx_ext1: u8,
    /// Send FX Ext 2 (byte 86)
    pub send_fx_ext2: u8,
    /// Send FX master mix (byte 87)
    pub send_fx_master_mix: u8,
    /// Send FX size feedback (byte 88)
    pub send_fx_size_feedback: u8,
    /// Send FX time (byte 89)
    pub send_fx_time: u8,
    /// Send FX HPF (byte 90)
    pub send_fx_hpf: u8,
    /// Send FX level (byte 91)
    pub send_fx_level: u8,
    /// Send return 3 source select (byte 92)
    pub send_return3_source: u8,
    /// Send return 3 type (byte 93)
    pub send_return3_type: u8,
    /// Send return 3 on/off (byte 94)
    pub send_return3_on: u8,
    /// Send return 3 level (byte 95)
    pub send_return3_level: u8,
    /// Reserved (byte 96)
    pub _reserved8: u8,
    /// Channel fader curve (byte 97)
    pub channel_fader_curve: u8,
    /// Cross fader curve (byte 98)
    pub crossfader_curve: u8,
    /// Cross fader position (byte 99)
    pub crossfader: u8,
    /// Beat FX on/off (byte 100)
    pub beat_fx_on: u8,
    /// Beat FX level/depth (byte 101)
    pub beat_fx_level: u8,
    /// Beat FX channel select (byte 102)
    pub beat_fx_channel: u8,
    /// Beat FX select (byte 103)
    pub beat_fx_select: u8,
    /// Beat FX freq Hi (byte 104)
    pub beat_fx_freq_hi: u8,
    /// Beat FX freq Mid (byte 105)
    pub beat_fx_freq_mid: u8,
    /// Beat FX freq Low (byte 106)
    pub beat_fx_freq_low: u8,
    /// Headphones pre EQ (byte 107)
    pub headphones_pre_eq: u8,
    /// Headphones A level (byte 108)
    pub headphones_a_level: u8,
    /// Headphones A mix (byte 109)
    pub headphones_a_mix: u8,
    /// Headphones B level (byte 110)
    pub headphones_b_level: u8,
    /// Headphones B mix (byte 111)
    pub headphones_b_mix: u8,
    /// Booth level (byte 112)
    pub booth_level: u8,
    /// Booth EQ Hi (byte 113)
    pub booth_eq_hi: u8,
    /// Booth EQ Low (byte 114)
    pub booth_eq_low: u8,
    /// Reserved (bytes 115-124)
    #[br(count = 10)]
    pub _reserved9: Vec<u8>,
    /// Channel 1-6 data (bytes 125-268, 24 bytes each)
    pub channels: [RawMixerChannel; 6],
    /// Final reserved byte (byte 269) - packet is 270 bytes total
    pub _reserved_final: u8,
}
