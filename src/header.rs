//! Management header parsing and building.
//!
//! All TCNet packets share a common 24-byte management header.

use std::io::Cursor;

use binrw::BinRead;

use crate::error::{Result, TcNetError};
use crate::types::{
    MessageType, NodeOptions, NodeType, MANAGEMENT_HEADER_SIZE, PROTOCOL_VERSION_MAJOR,
    PROTOCOL_VERSION_MINOR, TCNET_HEADER,
};
use crate::wire::RawManagementHeader;

/// The management header present in all TCNet packets.
#[derive(Debug, Clone)]
pub struct ManagementHeader {
    /// Unique node ID (for multiple apps on same IP)
    pub node_id: u16,
    /// Protocol version major
    pub version_major: u8,
    /// Protocol version minor
    pub version_minor: u8,
    /// Message type
    pub message_type: MessageType,
    /// Node name (8 ASCII characters)
    pub node_name: [u8; 8],
    /// Sequence number (0-255, wraps)
    pub sequence: u8,
    /// Node type
    pub node_type: NodeType,
    /// Node options flags
    pub node_options: NodeOptions,
    /// Timestamp in microseconds (0-999999)
    pub timestamp_us: u32,
}

impl ManagementHeader {
    /// Create a new management header with the given parameters.
    pub fn new(
        node_id: u16,
        message_type: MessageType,
        node_name: &str,
        sequence: u8,
        node_type: NodeType,
        node_options: NodeOptions,
        timestamp_us: u32,
    ) -> Self {
        let mut name_bytes = [b' '; 8];
        let name_slice = node_name.as_bytes();
        let len = name_slice.len().min(8);
        name_bytes[..len].copy_from_slice(&name_slice[..len]);

        Self {
            node_id,
            version_major: PROTOCOL_VERSION_MAJOR,
            version_minor: PROTOCOL_VERSION_MINOR,
            message_type,
            node_name: name_bytes,
            sequence,
            node_type,
            node_options,
            timestamp_us,
        }
    }

    /// Parse a management header from bytes using binrw.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < MANAGEMENT_HEADER_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: MANAGEMENT_HEADER_SIZE,
                actual: data.len(),
            });
        }

        let mut cursor = Cursor::new(data);
        let raw = RawManagementHeader::read(&mut cursor)
            .map_err(|e| TcNetError::ParseError(e.to_string()))?;

        Self::from_raw(raw)
    }

    /// Convert from raw wire format to high-level type.
    pub fn from_raw(raw: RawManagementHeader) -> Result<Self> {
        let message_type = MessageType::from_u8(raw.message_type)
            .ok_or(TcNetError::UnknownMessageType(raw.message_type))?;

        let node_type = NodeType::from_u8(raw.node_type)
            .ok_or(TcNetError::UnknownNodeType(raw.node_type))?;

        Ok(Self {
            node_id: raw.node_id,
            version_major: raw.version_major,
            version_minor: raw.version_minor,
            message_type,
            node_name: *raw.node_name.as_bytes(),
            sequence: raw.sequence,
            node_type,
            node_options: NodeOptions(raw.node_options),
            timestamp_us: raw.timestamp_us,
        })
    }

    /// Serialize the header to bytes.
    pub fn to_bytes(&self) -> [u8; MANAGEMENT_HEADER_SIZE] {
        let mut buf = [0u8; MANAGEMENT_HEADER_SIZE];

        // Node ID (bytes 0-1)
        buf[0..2].copy_from_slice(&self.node_id.to_le_bytes());

        // Protocol version (bytes 2-3)
        buf[2] = self.version_major;
        buf[3] = self.version_minor;

        // Header signature (bytes 4-6)
        buf[4..7].copy_from_slice(TCNET_HEADER);

        // Message type (byte 7)
        buf[7] = self.message_type as u8;

        // Node name (bytes 8-15)
        buf[8..16].copy_from_slice(&self.node_name);

        // Sequence (byte 16)
        buf[16] = self.sequence;

        // Node type (byte 17)
        buf[17] = self.node_type as u8;

        // Node options (bytes 18-19)
        buf[18..20].copy_from_slice(&self.node_options.0.to_le_bytes());

        // Timestamp (bytes 20-23)
        buf[20..24].copy_from_slice(&self.timestamp_us.to_le_bytes());

        buf
    }

    /// Get the node name as a string.
    pub fn node_name_str(&self) -> String {
        String::from_utf8_lossy(&self.node_name)
            .trim_end()
            .to_string()
    }

    /// Get the protocol version as a string.
    pub fn version_string(&self) -> String {
        format!("{}.{}", self.version_major, self.version_minor)
    }

}
