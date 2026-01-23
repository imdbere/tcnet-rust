//! Core types for the TCNet protocol.
//!
//! This module contains the fundamental enums and types used throughout
//! the TCNet protocol implementation.

use std::fmt;

/// TCNet protocol version
pub const PROTOCOL_VERSION_MAJOR: u8 = 3;
pub const PROTOCOL_VERSION_MINOR: u8 = 5;

/// TCNet header signature
pub const TCNET_HEADER: &[u8; 3] = b"TCN";

/// Broadcast ports used by TCNet
pub const PORT_BROADCAST_CONTROL: u16 = 60000;
pub const PORT_BROADCAST_TIME: u16 = 60001;

/// Default unicast port range
pub const PORT_UNICAST_DEFAULT: u16 = 65023;
pub const PORT_UNICAST_MAX: u16 = 65535;

/// Management header size in bytes
pub const MANAGEMENT_HEADER_SIZE: usize = 24;

/// Opt-IN packet total size
pub const OPT_IN_PACKET_SIZE: usize = 68;

/// Time packet minimum size (without on-air data, pre-V3.3.3)
pub const TIME_PACKET_MIN_SIZE: usize = 154;

/// Time packet full size (with on-air data, V3.3.3+)
pub const TIME_PACKET_SIZE: usize = 162;

/// Node type in the TCNet network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NodeType {
    /// Auto mode - can become master if needed
    Auto = 1,
    /// Master node - generates and sends timecode
    Master = 2,
    /// Slave node - receives timecode only
    Slave = 4,
    /// Repeater node - can receive and forward timecode
    Repeater = 8,
}

impl NodeType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            1 => Some(Self::Auto),
            2 => Some(Self::Master),
            4 => Some(Self::Slave),
            8 => Some(Self::Repeater),
            _ => None,
        }
    }
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Master => write!(f, "Master"),
            Self::Slave => write!(f, "Slave"),
            Self::Repeater => write!(f, "Repeater"),
        }
    }
}

/// Message types in TCNet protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MessageType {
    /// Opt-IN packet for joining network
    OptIn = 2,
    /// Opt-OUT packet for leaving network
    OptOut = 3,
    /// Status packet
    Status = 5,
    /// Time sync packet
    TimeSync = 10,
    /// Error notification
    ErrorNotification = 13,
    /// Request packet
    Request = 20,
    /// Application specific data
    ApplicationData = 30,
    /// Control messages
    Control = 101,
    /// Text data
    TextData = 128,
    /// Keyboard data
    KeyboardData = 132,
    /// Data packet (various subtypes)
    Data = 200,
    /// File data packet
    FileData = 204,
    /// Time packet
    Time = 254,
}

impl MessageType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            2 => Some(Self::OptIn),
            3 => Some(Self::OptOut),
            5 => Some(Self::Status),
            10 => Some(Self::TimeSync),
            13 => Some(Self::ErrorNotification),
            20 => Some(Self::Request),
            30 => Some(Self::ApplicationData),
            101 => Some(Self::Control),
            128 => Some(Self::TextData),
            132 => Some(Self::KeyboardData),
            200 => Some(Self::Data),
            204 => Some(Self::FileData),
            254 => Some(Self::Time),
            _ => None,
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OptIn => write!(f, "Opt-IN"),
            Self::OptOut => write!(f, "Opt-OUT"),
            Self::Status => write!(f, "Status"),
            Self::TimeSync => write!(f, "TimeSync"),
            Self::ErrorNotification => write!(f, "Error"),
            Self::Request => write!(f, "Request"),
            Self::ApplicationData => write!(f, "AppData"),
            Self::Control => write!(f, "Control"),
            Self::TextData => write!(f, "Text"),
            Self::KeyboardData => write!(f, "Keyboard"),
            Self::Data => write!(f, "Data"),
            Self::FileData => write!(f, "FileData"),
            Self::Time => write!(f, "Time"),
        }
    }
}

/// Layer state indicating playback status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum LayerState {
    #[default]
    Idle = 0,
    Playing = 3,
    Looping = 4,
    Paused = 5,
    Stopped = 6,
    CueButtonDown = 7,
    PlatterDown = 8,
    FastForward = 9,
    FastReverse = 10,
    Hold = 11,
}

impl LayerState {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Idle,
            3 => Self::Playing,
            4 => Self::Looping,
            5 => Self::Paused,
            6 => Self::Stopped,
            7 => Self::CueButtonDown,
            8 => Self::PlatterDown,
            9 => Self::FastForward,
            10 => Self::FastReverse,
            11 => Self::Hold,
            _ => Self::Idle,
        }
    }

    /// Returns true if this layer is actively producing timecode
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Playing | Self::Looping)
    }
}

impl fmt::Display for LayerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "IDLE"),
            Self::Playing => write!(f, "PLAYING"),
            Self::Looping => write!(f, "LOOPING"),
            Self::Paused => write!(f, "PAUSED"),
            Self::Stopped => write!(f, "STOPPED"),
            Self::CueButtonDown => write!(f, "CUE"),
            Self::PlatterDown => write!(f, "PLATTER"),
            Self::FastForward => write!(f, "FFWD"),
            Self::FastReverse => write!(f, "FFRV"),
            Self::Hold => write!(f, "HOLD"),
        }
    }
}

/// SMPTE timecode frame rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum SmpteMode {
    #[default]
    Fps24 = 24,
    Fps25 = 25,
    Fps29_97 = 29,
    Fps30 = 30,
}

impl SmpteMode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            24 => Self::Fps24,
            25 => Self::Fps25,
            29 => Self::Fps29_97,
            30 => Self::Fps30,
            _ => Self::Fps25, // Default to 25fps (common in EU)
        }
    }

    pub fn frames_per_second(&self) -> f64 {
        match self {
            Self::Fps24 => 24.0,
            Self::Fps25 => 25.0,
            Self::Fps29_97 => 29.97,
            Self::Fps30 => 30.0,
        }
    }
}

impl fmt::Display for SmpteMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fps24 => write!(f, "24fps"),
            Self::Fps25 => write!(f, "25fps"),
            Self::Fps29_97 => write!(f, "29.97fps"),
            Self::Fps30 => write!(f, "30fps"),
        }
    }
}

/// Timecode state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum TimecodeState {
    #[default]
    Stopped = 0,
    Running = 1,
    ForceResync = 2,
}

impl TimecodeState {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Stopped,
            1 => Self::Running,
            2 => Self::ForceResync,
            _ => Self::Stopped,
        }
    }
}

impl fmt::Display for TimecodeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stopped => write!(f, "STOP"),
            Self::Running => write!(f, "RUN"),
            Self::ForceResync => write!(f, "RESYNC"),
        }
    }
}

/// Node options flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NodeOptions(pub u16);

impl NodeOptions {
    pub const NEED_AUTHENTICATION: u16 = 1;
    pub const SUPPORTS_CONTROL_MESSAGES: u16 = 2;
    pub const SUPPORTS_APP_SPECIFIC_DATA: u16 = 4;
    pub const DO_NOT_DISTURB: u16 = 8;

    pub fn new() -> Self {
        Self(0)
    }

    pub fn with_control_messages(mut self) -> Self {
        self.0 |= Self::SUPPORTS_CONTROL_MESSAGES;
        self
    }

    pub fn with_app_specific_data(mut self) -> Self {
        self.0 |= Self::SUPPORTS_APP_SPECIFIC_DATA;
        self
    }

    pub fn needs_auth(&self) -> bool {
        self.0 & Self::NEED_AUTHENTICATION != 0
    }

    pub fn supports_control(&self) -> bool {
        self.0 & Self::SUPPORTS_CONTROL_MESSAGES != 0
    }

    pub fn supports_app_data(&self) -> bool {
        self.0 & Self::SUPPORTS_APP_SPECIFIC_DATA != 0
    }

    pub fn do_not_disturb(&self) -> bool {
        self.0 & Self::DO_NOT_DISTURB != 0
    }
}

/// Layer identifier for the 8 TCNet layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum Layer {
    #[default]
    Layer1 = 1,
    Layer2 = 2,
    Layer3 = 3,
    Layer4 = 4,
    LayerA = 5,
    LayerB = 6,
    LayerM = 7, // Master
    LayerC = 8,
}

impl Layer {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Layer1),
            1 => Some(Self::Layer2),
            2 => Some(Self::Layer3),
            3 => Some(Self::Layer4),
            4 => Some(Self::LayerA),
            5 => Some(Self::LayerB),
            6 => Some(Self::LayerM),
            7 => Some(Self::LayerC),
            _ => None,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::Layer1 => 0,
            Self::Layer2 => 1,
            Self::Layer3 => 2,
            Self::Layer4 => 3,
            Self::LayerA => 4,
            Self::LayerB => 5,
            Self::LayerM => 6,
            Self::LayerC => 7,
        }
    }

    pub fn all() -> [Layer; 8] {
        [
            Self::Layer1,
            Self::Layer2,
            Self::Layer3,
            Self::Layer4,
            Self::LayerA,
            Self::LayerB,
            Self::LayerM,
            Self::LayerC,
        ]
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Layer1 => write!(f, "L1"),
            Self::Layer2 => write!(f, "L2"),
            Self::Layer3 => write!(f, "L3"),
            Self::Layer4 => write!(f, "L4"),
            Self::LayerA => write!(f, "LA"),
            Self::LayerB => write!(f, "LB"),
            Self::LayerM => write!(f, "LM"),
            Self::LayerC => write!(f, "LC"),
        }
    }
}

/// SMPTE Timecode value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Timecode {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub frames: u8,
}

impl Timecode {
    pub fn new(hours: u8, minutes: u8, seconds: u8, frames: u8) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            frames,
        }
    }

    /// Create timecode from milliseconds and frame rate
    pub fn from_millis(millis: u32, mode: SmpteMode) -> Self {
        let total_seconds = millis / 1000;
        let remaining_millis = millis % 1000;

        let hours = (total_seconds / 3600) as u8;
        let minutes = ((total_seconds % 3600) / 60) as u8;
        let seconds = (total_seconds % 60) as u8;
        let frames = ((remaining_millis as f64 / 1000.0) * mode.frames_per_second()) as u8;

        Self {
            hours,
            minutes,
            seconds,
            frames,
        }
    }
}

impl fmt::Display for Timecode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}:{:02}",
            self.hours, self.minutes, self.seconds, self.frames
        )
    }
}
