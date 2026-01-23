//! Opt-IN Packet (Type 2) parsing and building.
//!
//! Opt-IN packets are used to join and maintain presence in a TCNet network.

use std::io::Cursor;

use binrw::BinRead;

use crate::error::{Result, TcNetError};
use crate::header::ManagementHeader;
use crate::types::{
    MessageType, NodeOptions, NodeType, OPT_IN_PACKET_SIZE, PORT_UNICAST_DEFAULT,
    PROTOCOL_VERSION_MAJOR, PROTOCOL_VERSION_MINOR,
};
use crate::wire::RawOptInPacket;

/// Opt-IN packet for network participation.
#[derive(Debug, Clone)]
pub struct OptInPacket {
    /// Common management header
    pub header: ManagementHeader,
    /// Number of nodes registered by this system
    pub node_count: u16,
    /// Listener port for unicast messages
    pub listener_port: u16,
    /// Uptime in seconds (rolls over every 12 hours)
    pub uptime_secs: u16,
    /// Vendor name (16 chars max)
    pub vendor_name: [u8; 16],
    /// Application/device name (16 chars max)
    pub app_name: [u8; 16],
    /// Application major version
    pub app_version_major: u8,
    /// Application minor version
    pub app_version_minor: u8,
    /// Application bug/patch version
    pub app_version_bug: u8,
}

impl OptInPacket {
    /// Create a new Opt-IN packet.
    pub fn new(
        node_id: u16,
        node_name: &str,
        sequence: u8,
        node_type: NodeType,
        node_options: NodeOptions,
        timestamp_us: u32,
        node_count: u16,
        listener_port: u16,
        uptime_secs: u16,
        vendor_name: &str,
        app_name: &str,
        app_version: (u8, u8, u8),
    ) -> Self {
        let header = ManagementHeader::new(
            node_id,
            MessageType::OptIn,
            node_name,
            sequence,
            node_type,
            node_options,
            timestamp_us,
        );

        let mut vendor_bytes = [0u8; 16];
        let vendor_slice = vendor_name.as_bytes();
        let len = vendor_slice.len().min(16);
        vendor_bytes[..len].copy_from_slice(&vendor_slice[..len]);

        let mut app_bytes = [0u8; 16];
        let app_slice = app_name.as_bytes();
        let len = app_slice.len().min(16);
        app_bytes[..len].copy_from_slice(&app_slice[..len]);

        Self {
            header,
            node_count,
            listener_port,
            uptime_secs,
            vendor_name: vendor_bytes,
            app_name: app_bytes,
            app_version_major: app_version.0,
            app_version_minor: app_version.1,
            app_version_bug: app_version.2,
        }
    }

    /// Parse an Opt-IN packet from bytes using binrw.
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < OPT_IN_PACKET_SIZE {
            return Err(TcNetError::PacketTooShort {
                expected: OPT_IN_PACKET_SIZE,
                actual: data.len(),
            });
        }

        let mut cursor = Cursor::new(data);
        let raw = RawOptInPacket::read(&mut cursor)
            .map_err(|e| TcNetError::ParseError(e.to_string()))?;

        let header = ManagementHeader::from_raw(raw.header)?;

        Ok(Self {
            header,
            node_count: raw.node_count,
            listener_port: raw.listener_port,
            uptime_secs: raw.uptime_secs,
            vendor_name: *raw.vendor_name.as_bytes(),
            app_name: *raw.app_name.as_bytes(),
            app_version_major: raw.app_version_major,
            app_version_minor: raw.app_version_minor,
            app_version_bug: raw.app_version_bug,
        })
    }

    /// Serialize the packet to bytes.
    pub fn to_bytes(&self) -> [u8; OPT_IN_PACKET_SIZE] {
        let mut buf = [0u8; OPT_IN_PACKET_SIZE];

        // Management header (bytes 0-23)
        buf[..24].copy_from_slice(&self.header.to_bytes());

        // Node count (bytes 24-25)
        buf[24..26].copy_from_slice(&self.node_count.to_le_bytes());

        // Listener port (bytes 26-27)
        buf[26..28].copy_from_slice(&self.listener_port.to_le_bytes());

        // Uptime (bytes 28-29)
        buf[28..30].copy_from_slice(&self.uptime_secs.to_le_bytes());

        // Reserved (bytes 30-31)

        // Vendor name (bytes 32-47)
        buf[32..48].copy_from_slice(&self.vendor_name);

        // App name (bytes 48-63)
        buf[48..64].copy_from_slice(&self.app_name);

        // Version (bytes 64-66)
        buf[64] = self.app_version_major;
        buf[65] = self.app_version_minor;
        buf[66] = self.app_version_bug;

        // Reserved (byte 67)

        buf
    }

    /// Get the vendor name as a string.
    pub fn vendor_name_str(&self) -> String {
        String::from_utf8_lossy(&self.vendor_name)
            .trim_end_matches('\0')
            .trim()
            .to_string()
    }

    /// Get the application name as a string.
    pub fn app_name_str(&self) -> String {
        String::from_utf8_lossy(&self.app_name)
            .trim_end_matches('\0')
            .trim()
            .to_string()
    }

    /// Get the application version as a string.
    pub fn app_version_str(&self) -> String {
        format!(
            "{}.{}.{}",
            self.app_version_major, self.app_version_minor, self.app_version_bug
        )
    }
}

/// Builder for creating Opt-IN packets with sensible defaults.
pub struct OptInBuilder {
    node_id: u16,
    node_name: String,
    node_type: NodeType,
    node_options: NodeOptions,
    listener_port: u16,
    vendor_name: String,
    app_name: String,
    app_version: (u8, u8, u8),
}

impl OptInBuilder {
    pub fn new(node_name: &str) -> Self {
        Self {
            node_id: 1,
            node_name: node_name.to_string(),
            node_type: NodeType::Slave,
            node_options: NodeOptions::new(),
            listener_port: PORT_UNICAST_DEFAULT,
            vendor_name: "tcnet-rust".to_string(),
            app_name: "TCNet Listener".to_string(),
            app_version: (
                PROTOCOL_VERSION_MAJOR,
                PROTOCOL_VERSION_MINOR,
                0,
            ),
        }
    }

    pub fn node_id(mut self, id: u16) -> Self {
        self.node_id = id;
        self
    }

    pub fn node_type(mut self, node_type: NodeType) -> Self {
        self.node_type = node_type;
        self
    }

    pub fn node_options(mut self, options: NodeOptions) -> Self {
        self.node_options = options;
        self
    }

    pub fn listener_port(mut self, port: u16) -> Self {
        self.listener_port = port;
        self
    }

    pub fn vendor(mut self, name: &str) -> Self {
        self.vendor_name = name.to_string();
        self
    }

    pub fn app_name(mut self, name: &str) -> Self {
        self.app_name = name.to_string();
        self
    }

    pub fn app_version(mut self, major: u8, minor: u8, bug: u8) -> Self {
        self.app_version = (major, minor, bug);
        self
    }

    /// Build the Opt-IN packet with the given sequence and timestamp.
    pub fn build(&self, sequence: u8, timestamp_us: u32, node_count: u16, uptime_secs: u16) -> OptInPacket {
        OptInPacket::new(
            self.node_id,
            &self.node_name,
            sequence,
            self.node_type,
            self.node_options,
            timestamp_us,
            node_count,
            self.listener_port,
            uptime_secs,
            &self.vendor_name,
            &self.app_name,
            self.app_version,
        )
    }
}
