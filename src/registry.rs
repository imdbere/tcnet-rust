//! Node registry for tracking active TCNet nodes.
//!
//! Maintains a list of discovered nodes and handles timeouts for stale nodes.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use crate::types::NodeType;

/// Timeout for considering a node as disconnected (no packets received).
/// TCNet spec says Opt-IN should be sent every 1000ms, but some implementations
/// may send less frequently. We also refresh on Status packets.
pub const NODE_TIMEOUT: Duration = Duration::from_secs(10);

/// Information about a discovered TCNet node.
#[derive(Debug, Clone)]
pub struct NodeInfo {
    /// Node name (8 characters)
    pub node_name: String,
    /// Node type (Auto, Master, Slave, Repeater)
    pub node_type: NodeType,
    /// Node ID
    pub node_id: u16,
    /// Vendor name
    pub vendor: String,
    /// Application name
    pub app: String,
    /// Application version string
    pub app_version: String,
    /// Listener port for unicast messages
    pub listener_port: u16,
    /// Last known address
    pub address: SocketAddr,
    /// Last time we received an Opt-IN from this node
    pub last_seen: Instant,
    /// Number of nodes this node knows about
    pub node_count: u16,
    /// Uptime in seconds
    pub uptime_secs: u16,
}

/// Key used to uniquely identify a node.
/// Combination of IP address and node name (for multiple apps on same host).
/// Note: We use node_name instead of node_id because some implementations (like Pro DJ Link Bridge)
/// change their node_id with each packet, but the node_name remains stable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeKey {
    pub ip: std::net::IpAddr,
    pub node_name: String,
}

impl NodeKey {
    pub fn new(addr: SocketAddr, node_name: String) -> Self {
        Self {
            ip: addr.ip(),
            node_name,
        }
    }
}

/// Events emitted by the registry when node state changes.
#[derive(Debug, Clone)]
pub enum RegistryEvent {
    /// A new node was discovered
    NodeDiscovered(NodeInfo),
    /// A known node was updated (e.g., type changed)
    NodeUpdated(NodeInfo),
    /// A node timed out or sent Opt-OUT
    NodeRemoved { node_name: String, reason: RemovalReason },
}

/// Reason for node removal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemovalReason {
    /// Node sent an Opt-OUT packet
    OptOut,
    /// Node timed out (no Opt-IN received)
    Timeout,
}

/// Registry of active TCNet nodes.
#[derive(Debug, Default)]
pub struct NodeRegistry {
    nodes: HashMap<NodeKey, NodeInfo>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Update or insert a node based on received Opt-IN packet.
    /// Returns Some(event) if this caused a state change.
    pub fn update_node(
        &mut self,
        key: NodeKey,
        node_name: String,
        node_type: NodeType,
        node_id: u16,
        vendor: String,
        app: String,
        app_version: String,
        listener_port: u16,
        address: SocketAddr,
        node_count: u16,
        uptime_secs: u16,
    ) -> Option<RegistryEvent> {
        let now = Instant::now();

        if let Some(existing) = self.nodes.get_mut(&key) {
            // Node already known - update last_seen and check for changes
            existing.last_seen = now;
            existing.node_count = node_count;
            existing.uptime_secs = uptime_secs;
            existing.address = address;

            // Check if anything significant changed
            let changed = existing.node_type != node_type
                || existing.vendor != vendor
                || existing.app != app;

            if changed {
                existing.node_type = node_type;
                existing.vendor = vendor;
                existing.app = app;
                existing.app_version = app_version;
                existing.listener_port = listener_port;
                return Some(RegistryEvent::NodeUpdated(existing.clone()));
            }

            None
        } else {
            // New node discovered
            let info = NodeInfo {
                node_name,
                node_type,
                node_id,
                vendor,
                app,
                app_version,
                listener_port,
                address,
                last_seen: now,
                node_count,
                uptime_secs,
            };
            self.nodes.insert(key, info.clone());
            Some(RegistryEvent::NodeDiscovered(info))
        }
    }

    /// Refresh a node's last_seen timestamp without changing other data.
    /// This can be called when we receive any packet from the node (not just Opt-IN).
    /// Returns true if the node was found and refreshed.
    pub fn refresh_node(&mut self, key: &NodeKey) -> bool {
        if let Some(existing) = self.nodes.get_mut(key) {
            existing.last_seen = Instant::now();
            true
        } else {
            false
        }
    }

    /// Remove a node due to Opt-OUT.
    /// Returns Some(event) if the node existed.
    pub fn remove_node(&mut self, key: &NodeKey) -> Option<RegistryEvent> {
        self.nodes.remove(key).map(|info| RegistryEvent::NodeRemoved {
            node_name: info.node_name,
            reason: RemovalReason::OptOut,
        })
    }

    /// Check for and remove timed-out nodes.
    /// Returns a list of removal events for nodes that timed out.
    pub fn cleanup_stale_nodes(&mut self) -> Vec<RegistryEvent> {
        let now = Instant::now();
        let mut events = Vec::new();

        self.nodes.retain(|_, info| {
            if now.duration_since(info.last_seen) > NODE_TIMEOUT {
                events.push(RegistryEvent::NodeRemoved {
                    node_name: info.node_name.clone(),
                    reason: RemovalReason::Timeout,
                });
                false
            } else {
                true
            }
        });

        events
    }

    /// Get all known nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &NodeInfo> {
        self.nodes.values()
    }

    /// Get a specific node by key.
    pub fn get(&self, key: &NodeKey) -> Option<&NodeInfo> {
        self.nodes.get(key)
    }

    /// Get the number of known nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
