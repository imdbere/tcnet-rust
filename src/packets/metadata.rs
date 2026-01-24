//! Metadata Packet (Data Type 4).
//!
//! Contains track metadata for a layer including artist name, track title,
//! musical key, and track ID.

use std::io::Cursor;

use binrw::BinRead;

use crate::error::{Result, TcNetError};
use crate::header::ManagementHeader;
use crate::types::Layer;
use crate::wire::RawMetadataPacket;

/// Data type identifier for Metadata packets.
pub const METADATA_DATA_TYPE: u8 = 4;

/// Minimum packet size for Metadata.
pub const METADATA_PACKET_SIZE: usize = 548;

/// Musical key representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TrackKey(pub u16);

impl TrackKey {
    /// Create a new TrackKey from the raw protocol value.
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    /// Get the raw key value.
    pub fn raw(&self) -> u16 {
        self.0
    }

    /// Check if the key is unknown/not set.
    pub fn is_unknown(&self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for TrackKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_unknown() {
            write!(f, "Unknown")
        } else {
            // Key value encoding varies by implementation
            // Common format: lower byte = note (0-11), upper byte = mode (major/minor)
            write!(f, "Key({})", self.0)
        }
    }
}

/// Metadata packet containing track information for a layer.
#[derive(Debug, Clone)]
pub struct MetadataPacket {
    /// Common management header
    pub header: ManagementHeader,
    /// Layer this metadata is for
    pub layer: Layer,
    /// Track artist name
    pub artist: String,
    /// Track title
    pub title: String,
    /// Musical key of the track
    pub key: TrackKey,
    /// Track ID (database ID of loaded track)
    pub track_id: u32,
}

impl MetadataPacket {
    /// Parse a Metadata packet from raw bytes.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < METADATA_PACKET_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: METADATA_PACKET_SIZE,
                actual: data.len(),
            });
        }

        let mut cursor = Cursor::new(data);
        let raw = RawMetadataPacket::read(&mut cursor)
            .map_err(|e| TcNetError::ParseError(e.to_string()))?;

        let header = ManagementHeader::from_raw(raw.header.clone())?;

        // Layer ID is 1-based in the protocol
        let layer = Layer::from_index((raw.layer_id.saturating_sub(1)) as usize)
            .unwrap_or(Layer::Layer1);

        // Decode artist and title strings
        // TCNet 1.0-3.4.9 uses UTF-8, 3.5.0+ uses UTF-16
        let artist = Self::decode_string(&raw.track_artist.0, &raw.header);
        let title = Self::decode_string(&raw.track_title.0, &raw.header);

        Ok(Self {
            header,
            layer,
            artist,
            title,
            key: TrackKey::new(raw.track_key),
            track_id: raw.track_id,
        })
    }

    /// Decode a string field based on protocol version.
    /// Pre-3.5 uses UTF-8, 3.5+ uses UTF-16LE.
    fn decode_string(bytes: &[u8], raw_header: &crate::wire::RawManagementHeader) -> String {
        let is_utf16 = raw_header.version_major >= 3 && raw_header.version_minor >= 5;

        if is_utf16 {
            Self::decode_utf16le(bytes)
        } else {
            Self::decode_utf8(bytes)
        }
    }

    /// Decode as UTF-8, trimming null bytes.
    fn decode_utf8(bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes)
            .trim_end_matches('\0')
            .trim()
            .to_string()
    }

    /// Decode as UTF-16LE, trimming null characters.
    fn decode_utf16le(bytes: &[u8]) -> String {
        // Convert bytes to u16 pairs (little-endian)
        let u16_values: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .take_while(|&c| c != 0) // Stop at null terminator
            .collect();

        String::from_utf16_lossy(&u16_values).trim().to_string()
    }

    /// Check if this metadata packet has valid track info loaded.
    pub fn has_track(&self) -> bool {
        !self.artist.is_empty() || !self.title.is_empty() || self.track_id != 0
    }

    /// Get a formatted display string (Artist - Title).
    pub fn display_string(&self) -> String {
        match (self.artist.is_empty(), self.title.is_empty()) {
            (true, true) => String::from("[No Track]"),
            (true, false) => self.title.clone(),
            (false, true) => self.artist.clone(),
            (false, false) => format!("{} - {}", self.artist, self.title),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_utf8() {
        let bytes = b"Test Artist\0\0\0\0\0\0\0\0";
        let result = MetadataPacket::decode_utf8(bytes);
        assert_eq!(result, "Test Artist");
    }

    #[test]
    fn test_decode_utf16le() {
        // "Test" in UTF-16LE: T=0x54, e=0x65, s=0x73, t=0x74
        let bytes: &[u8] = &[0x54, 0x00, 0x65, 0x00, 0x73, 0x00, 0x74, 0x00, 0x00, 0x00];
        let result = MetadataPacket::decode_utf16le(bytes);
        assert_eq!(result, "Test");
    }

    #[test]
    fn test_display_string() {
        let packet = MetadataPacket {
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
            artist: "Daft Punk".to_string(),
            title: "Around the World".to_string(),
            key: TrackKey::new(0),
            track_id: 42,
        };

        assert_eq!(packet.display_string(), "Daft Punk - Around the World");
        assert!(packet.has_track());
    }

    #[test]
    fn test_track_key() {
        let unknown = TrackKey::new(0);
        assert!(unknown.is_unknown());

        let known = TrackKey::new(0x0105);
        assert!(!known.is_unknown());
    }
}
