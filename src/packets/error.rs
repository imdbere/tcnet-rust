//! Error/Notification Packet (Message Type 13).
//!
//! Sent in response to requests that failed or couldn't be handled.

use std::fmt;
use std::io::Cursor;

use binrw::BinRead;

use crate::error::{Result, TcNetError};
use crate::header::ManagementHeader;
use crate::types::Layer;
use crate::wire::RawErrorNotificationPacket;

/// Packet size for Error/Notification packets.
pub const ERROR_NOTIFICATION_PACKET_SIZE: usize = 30;

/// Error/notification codes returned by TCNet nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ErrorCode {
    /// Request was not recognized
    RequestUnknown = 1,
    /// Request was recognized but cannot be handled by the node
    RequestNotPossible = 13,
    /// Request was for data that is empty (nothing to send)
    RequestDataEmpty = 14,
    /// Request was handled successfully (OK response)
    RequestOk = 255,
    /// Unknown error code
    Unknown(u16),
}

impl ErrorCode {
    /// Parse an error code from the raw u16 value.
    pub fn from_u16(value: u16) -> Self {
        match value {
            1 => Self::RequestUnknown,
            13 => Self::RequestNotPossible,
            14 => Self::RequestDataEmpty,
            255 => Self::RequestOk,
            other => Self::Unknown(other),
        }
    }

    /// Get the raw u16 value.
    pub fn as_u16(&self) -> u16 {
        match self {
            Self::RequestUnknown => 1,
            Self::RequestNotPossible => 13,
            Self::RequestDataEmpty => 14,
            Self::RequestOk => 255,
            Self::Unknown(v) => *v,
        }
    }

    /// Check if this is an error (not OK).
    pub fn is_error(&self) -> bool {
        !matches!(self, Self::RequestOk)
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestUnknown => write!(f, "Request Unknown"),
            Self::RequestNotPossible => write!(f, "Request Not Possible"),
            Self::RequestDataEmpty => write!(f, "Data Empty"),
            Self::RequestOk => write!(f, "OK"),
            Self::Unknown(code) => write!(f, "Unknown({})", code),
        }
    }
}

/// Error/Notification packet sent in response to failed requests.
#[derive(Debug, Clone)]
pub struct ErrorNotificationPacket {
    /// Common management header
    pub header: ManagementHeader,
    /// Data type from the original request
    pub data_type: u8,
    /// Layer from the original request (if applicable)
    pub layer: Option<Layer>,
    /// Raw layer ID (0 means not layer-specific)
    pub layer_id: u8,
    /// Error/notification code
    pub code: ErrorCode,
    /// Message type from the original request
    pub request_message_type: u16,
}

impl ErrorNotificationPacket {
    /// Parse an Error/Notification packet from raw bytes.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < ERROR_NOTIFICATION_PACKET_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: ERROR_NOTIFICATION_PACKET_SIZE,
                actual: data.len(),
            });
        }

        let mut cursor = Cursor::new(data);
        let raw = RawErrorNotificationPacket::read(&mut cursor)
            .map_err(|e| TcNetError::ParseError(e.to_string()))?;

        let header = ManagementHeader::from_raw(raw.header)?;

        // Layer ID 0 means not layer-specific
        let layer = if raw.layer_id > 0 {
            Layer::from_index((raw.layer_id.saturating_sub(1)) as usize)
        } else {
            None
        };

        Ok(Self {
            header,
            data_type: raw.data_type,
            layer,
            layer_id: raw.layer_id,
            code: ErrorCode::from_u16(raw.code),
            request_message_type: raw.request_message_type,
        })
    }

    /// Get a description of what was requested.
    pub fn request_description(&self) -> String {
        let data_type_name = match self.data_type {
            2 => "Metrics",
            4 => "Metadata",
            8 => "BeatGrid",
            12 => "CueData",
            16 => "SmallWaveform",
            32 => "BigWaveform",
            150 => "MixerData",
            _ => "Unknown",
        };

        if let Some(layer) = self.layer {
            format!("{} for {}", data_type_name, layer)
        } else {
            data_type_name.to_string()
        }
    }
}

impl fmt::Display for ErrorNotificationPacket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error from {}: {} - {} (request: {})",
            self.header.node_name_str(),
            self.code,
            self.request_description(),
            self.request_message_type
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_parsing() {
        assert_eq!(ErrorCode::from_u16(1), ErrorCode::RequestUnknown);
        assert_eq!(ErrorCode::from_u16(13), ErrorCode::RequestNotPossible);
        assert_eq!(ErrorCode::from_u16(14), ErrorCode::RequestDataEmpty);
        assert_eq!(ErrorCode::from_u16(255), ErrorCode::RequestOk);
        assert!(matches!(ErrorCode::from_u16(999), ErrorCode::Unknown(999)));
    }

    #[test]
    fn test_error_code_is_error() {
        assert!(ErrorCode::RequestUnknown.is_error());
        assert!(ErrorCode::RequestNotPossible.is_error());
        assert!(ErrorCode::RequestDataEmpty.is_error());
        assert!(!ErrorCode::RequestOk.is_error());
    }
}
