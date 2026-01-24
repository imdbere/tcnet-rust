//! # TCNet Protocol Implementation
//!
//! A Rust implementation of the TCNet protocol for real-time timecode sharing
//! between entertainment industry devices and software.
//!
//! ## Overview
//!
//! TCNet is an open protocol designed for sharing real-time Time Code and Meta Data
//! between devices. It uses UDP for communication and supports various node roles:
//!
//! - **Master**: Generates and sends timecode packets
//! - **Slave**: Receives timecode packets only
//! - **Repeater**: Can receive and forward timecode packets
//! - **Auto**: Can become master if needed
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use tcnet::{Node, NodeConfig, NodeEvent};
//!
//! #[tokio::main]
//! async fn main() -> tcnet::Result<()> {
//!     let config = NodeConfig::new("MYNODE");
//!     let node = Arc::new(Node::new(config));
//!     
//!     let mut events = node.subscribe();
//!     
//!     // Spawn event handler
//!     tokio::spawn(async move {
//!         while let Ok(event) = events.recv().await {
//!             match event {
//!                 NodeEvent::TimePacket(tp) => {
//!                     for layer in tp.active_layers() {
//!                         println!("{}: {}", layer.layer, layer.timecode);
//!                     }
//!                 }
//!                 _ => {}
//!             }
//!         }
//!     });
//!     
//!     // Run the node (blocks)
//!     node.run().await
//! }
//! ```
//!
//! ## Protocol Version
//!
//! This implementation targets TCNet protocol version 3.5.1B.

pub mod error;
pub mod header;
pub mod node;
pub mod packets;
pub mod registry;
pub mod types;
pub mod wire;

// Re-export main types for convenience
pub use error::{Result, TcNetError};
pub use header::ManagementHeader;
pub use node::{Node, NodeConfig, NodeEvent};
pub use packets::{
    LayerStatus, LayerTimeData, MetadataPacket, MetricsDataPacket, MixerChannel, MixerDataPacket,
    MixerType, OptInPacket, Packet, StatusPacket, TimePacket, TrackKey,
};
pub use registry::{NodeInfo, NodeRegistry, RegistryEvent, RemovalReason};
pub use types::{
    Layer, LayerState, MessageType, NodeOptions, NodeType, SmpteMode, Timecode, TimecodeState,
};
