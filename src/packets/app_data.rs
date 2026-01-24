//! Application Specific Data Packet (Type 30).
//!
//! These packets are used for application-specific communication between TCNet nodes.
//! Resolume Arena uses these to subscribe to mixer data updates.

use crate::header::ManagementHeader;
use crate::types::{MessageType, NodeOptions, NodeType};

/// Application Specific Data packet.
#[derive(Debug, Clone)]
pub struct AppDataPacket {
    /// Common management header
    pub header: ManagementHeader,
    /// Data Identifier 1 (byte 24)
    pub data_id1: u8,
    /// Data Identifier 2 (byte 25)
    pub data_id2: u8,
    /// Data size
    pub data_size: u32,
    /// Total packets
    pub total_packets: u32,
    /// Packet number
    pub packet_no: u32,
    /// Packet signature
    pub signature: u32,
    /// Application data
    pub data: Vec<u8>,
}

impl AppDataPacket {
    /// Create a new Application Specific Data packet.
    pub fn new(
        node_id: u16,
        node_name: &str,
        sequence: u8,
        node_type: NodeType,
        node_options: NodeOptions,
        timestamp_us: u32,
        data_id1: u8,
        data_id2: u8,
        data: Vec<u8>,
    ) -> Self {
        Self::new_with_signature(
            node_id,
            node_name,
            sequence,
            node_type,
            node_options,
            timestamp_us,
            data_id1,
            data_id2,
            0,
            data,
        )
    }

    /// Create a new Application Specific Data packet with custom signature.
    pub fn new_with_signature(
        node_id: u16,
        node_name: &str,
        sequence: u8,
        node_type: NodeType,
        node_options: NodeOptions,
        timestamp_us: u32,
        data_id1: u8,
        data_id2: u8,
        signature: u32,
        data: Vec<u8>,
    ) -> Self {
        let header = ManagementHeader::new(
            node_id,
            MessageType::ApplicationData,
            node_name,
            sequence,
            node_type,
            node_options,
            timestamp_us,
        );

        Self {
            header,
            data_id1,
            data_id2,
            data_size: data.len() as u32,
            total_packets: 1,
            packet_no: 1,
            signature,
            data,
        }
    }

    /// Create Resolume-style subscription packets.
    /// Resolume sends TWO app data packets to subscribe to mixer updates.
    /// Returns a Vec of two packets that should both be sent.
    pub fn resolume_style_pair(
        node_id: u16,
        node_name: &str,
        sequence: u8,
        node_type: NodeType,
        node_options: NodeOptions,
        timestamp_us: u32,
    ) -> Vec<Self> {
        // Signature used by Resolume: 0x0abc0000 (little endian: 0000bc0a)
        let signature = 0x0abc0000u32;

        // Packet 1 data (20 bytes): 00002aff + zeros
        let data1 = vec![
            0x00, 0x00, 0xcf, 0xff,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        // Packet 2 data (20 bytes): 020099ff434797a41e0062c80b48c85a9696a0fc
        let data2 = vec![
            0x02, 0x00, 0x99, 0xff,
            0x43, 0x47, 0x97, 0xa4,
            0x1e, 0x00, 0x62, 0xc8,
            0x0b, 0x48, 0xc8, 0x5a,
            0x96, 0x96, 0xa0, 0xfc,
        ];

        vec![
            Self::new_with_signature(
                node_id,
                node_name,
                sequence,
                node_type,
                node_options,
                timestamp_us,
                0xff, // Data Identifier 1
                0xff, // Data Identifier 2
                signature,
                data1,
            ),
            Self::new_with_signature(
                node_id,
                node_name,
                sequence.wrapping_add(1), // Increment sequence for second packet
                node_type,
                node_options,
                timestamp_us,
                0xff, // Data Identifier 1
                0xff, // Data Identifier 2
                signature,
                data2,
            ),
        ]
    }

    /// Serialize the packet to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(42 + self.data.len());

        // Management header (bytes 0-23)
        buf.extend_from_slice(&self.header.to_bytes());

        // Data Identifier 1 & 2 (bytes 24-25)
        buf.push(self.data_id1);
        buf.push(self.data_id2);

        // Data Size (bytes 26-29)
        buf.extend_from_slice(&self.data_size.to_le_bytes());

        // Total Packets (bytes 30-33)
        buf.extend_from_slice(&self.total_packets.to_le_bytes());

        // Packet No (bytes 34-37)
        buf.extend_from_slice(&self.packet_no.to_le_bytes());

        // Packet Signature (bytes 38-41)
        buf.extend_from_slice(&self.signature.to_le_bytes());

        // Data (bytes 42+)
        buf.extend_from_slice(&self.data);

        buf
    }
}
