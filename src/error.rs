//! Error types for the TCNet protocol.

use thiserror::Error;

/// Errors that can occur when parsing or handling TCNet packets.
#[derive(Error, Debug)]
pub enum TcNetError {
    /// Packet is too short to contain required data
    #[error("packet too short: expected at least {expected} bytes, got {actual}")]
    PacketTooShort { expected: usize, actual: usize },

    /// Invalid TCNet header (should be "TCN")
    #[error("invalid header: expected 'TCN', got {0:?}")]
    InvalidHeader([u8; 3]),

    /// Unknown or unsupported message type
    #[error("unknown message type: {0}")]
    UnknownMessageType(u8),

    /// Unknown node type
    #[error("unknown node type: {0}")]
    UnknownNodeType(u8),

    /// Invalid node name (not valid ASCII)
    #[error("invalid node name: {0}")]
    InvalidNodeName(String),

    /// IO error during network operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to get MAC address
    #[error("failed to get MAC address: {0}")]
    MacAddress(String),

    /// Binary parsing error
    #[error("parse error: {0}")]
    ParseError(String),
}

/// Result type for TCNet operations.
pub type Result<T> = std::result::Result<T, TcNetError>;
