//! Authentication handshake for TCNet peers that require it.
//!
//! Some TCNet nodes (notably Pro DJ Link Bridge) advertise `NEED_AUTHENTICATION` (flag 1)
//! in their Opt-IN packet. This module implements the Arena-style challenge/response
//! handshake used to establish communication with such peers.
//!
//! ## Protocol (observed from Resolume Arena ↔ Pro DJ Link Bridge)
//!
//! The handshake uses TCNet Application Specific Data packets (Type 30) with a 20-byte payload:
//!
//! 1. **Arena → Bridge (step=0)**: Request/keepalive
//!    - payload: `[step=0, arena_port, zeros...]`
//!
//! 2. **Bridge → Arena (step=1)**: Challenge
//!    - payload: `[step=1, bridge_port, challenge, zeros...]`
//!
//! 3. **Arena → Bridge (step=2)**: Response
//!    - payload: `[step=2, arena_port, challenge, response, trailer]`
//!    - where `response = challenge XOR xor_key`
//!
//! After the handshake completes, Arena continues sending step=0 packets as keepalives.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::Ordering;

use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tracing::{debug, info, trace};

use crate::header::ManagementHeader;
use crate::node::Node;
use crate::packets::{AppDataPacket, RESOLUME_ARENA_XOR_KEY_PORT_65446};
use crate::registry::NodeKey;

/// Pioneer's registered application code (observed in Packet Signature field).
pub const PIONEER_SIGNATURE: u32 = 0x0aa0_0000;

/// State of the authentication handshake with a single peer.
#[derive(Debug, Clone, Copy)]
pub struct AuthPeerState {
    /// Target address (ip + listener_port) for sending packets.
    pub target_addr: SocketAddr,
    /// Whether we've sent the initial step=0 request.
    pub step0_sent: bool,
    /// Whether the handshake has completed (we received step=1 and sent step=2).
    pub handshake_complete: bool,
}

/// Manages authentication handshakes with peers that require it.
#[derive(Debug, Default)]
pub struct AuthManager {
    /// Per-peer handshake state, keyed by `(ip, node_name)`.
    peers: Mutex<HashMap<NodeKey, AuthPeerState>>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            peers: Mutex::new(HashMap::new()),
        }
    }

    /// Get or create the peer state for a given key.
    /// Returns (state, is_new) where is_new indicates if this is a newly tracked peer.
    pub async fn get_or_create_peer(
        &self,
        key: NodeKey,
        target_addr: SocketAddr,
    ) -> (AuthPeerState, bool) {
        let mut peers = self.peers.lock().await;
        if let Some(state) = peers.get(&key) {
            (
                AuthPeerState {
                    target_addr, // always use latest target
                    ..*state
                },
                false,
            )
        } else {
            let state = AuthPeerState {
                target_addr,
                step0_sent: false,
                handshake_complete: false,
            };
            peers.insert(key, state);
            (state, true)
        }
    }

    /// Mark that we've sent step=0 to a peer.
    pub async fn mark_step0_sent(&self, key: &NodeKey) {
        let mut peers = self.peers.lock().await;
        if let Some(state) = peers.get_mut(key) {
            state.step0_sent = true;
        }
    }

    /// Mark that the handshake with a peer is complete.
    pub async fn mark_handshake_complete(&self, key: NodeKey, target_addr: SocketAddr) {
        let mut peers = self.peers.lock().await;
        let entry = peers.entry(key).or_insert(AuthPeerState {
            target_addr,
            step0_sent: true,
            handshake_complete: false,
        });
        entry.handshake_complete = true;
    }

    /// Remove tracking for a peer (e.g., on Opt-OUT).
    pub async fn remove_peer(&self, key: &NodeKey) {
        let mut peers = self.peers.lock().await;
        peers.remove(key);
    }

    /// Get all peers that have completed the handshake (for keepalives).
    pub async fn get_authenticated_peers(&self) -> Vec<SocketAddr> {
        let peers = self.peers.lock().await;
        peers
            .values()
            .filter(|p| p.handshake_complete)
            .map(|p| p.target_addr)
            .collect()
    }

    /// Check if we've already sent step=0 to a peer.
    pub async fn has_sent_step0(&self, key: &NodeKey) -> bool {
        let peers = self.peers.lock().await;
        peers.get(key).map(|s| s.step0_sent).unwrap_or(false)
    }

    /// Update the target address for a peer (if their listener port changed).
    pub async fn update_target_addr(&self, key: &NodeKey, target_addr: SocketAddr) {
        let mut peers = self.peers.lock().await;
        if let Some(state) = peers.get_mut(key) {
            state.target_addr = target_addr;
        }
    }
}

impl Node {
    /// Build an Arena-style step=0 request packet.
    pub(crate) fn build_auth_step0(&self) -> AppDataPacket {
        AppDataPacket::resolume_arena_step0(
            self.node_id,
            &self.config.node_name,
            self.next_sequence(),
            self.config.node_type,
            self.config.node_options,
            self.timestamp_us(),
            self.listener_port.load(Ordering::Relaxed),
        )
    }

    /// Build an Arena-style step=2 response packet.
    pub(crate) fn build_auth_step2(&self, challenge: u32) -> AppDataPacket {
        AppDataPacket::resolume_arena_step2(
            self.node_id,
            &self.config.node_name,
            self.next_sequence(),
            self.config.node_type,
            self.config.node_options,
            self.timestamp_us(),
            self.listener_port.load(Ordering::Relaxed),
            challenge,
            RESOLUME_ARENA_XOR_KEY_PORT_65446,
        )
    }

    /// Send an Arena-style step=0 request to a peer.
    pub(crate) async fn send_auth_step0(&self, target_addr: SocketAddr) {
        let socket_guard = self.unicast_socket.lock().await;
        let Some(socket) = socket_guard.as_ref() else {
            debug!("Cannot send auth step0: unicast socket not initialized");
            return;
        };

        let packet = self.build_auth_step0();
        if let Err(e) = socket.send_to(&packet.to_bytes(), target_addr).await {
            debug!("Failed to send auth step0 to {}: {}", target_addr, e);
        } else {
            trace!(
                "Sent auth step0 to {} (seq: {})",
                target_addr,
                packet.header.sequence
            );
        }
    }

    /// Send an Arena-style step=2 response to a peer.
    pub(crate) async fn send_auth_step2(&self, target_addr: SocketAddr, challenge: u32) {
        let socket_guard = self.unicast_socket.lock().await;
        let Some(socket) = socket_guard.as_ref() else {
            debug!("Cannot send auth step2: unicast socket not initialized");
            return;
        };

        let packet = self.build_auth_step2(challenge);
        if let Err(e) = socket.send_to(&packet.to_bytes(), target_addr).await {
            debug!(
                "Failed to send auth step2 to {} (challenge: 0x{:08x}): {}",
                target_addr, challenge, e
            );
        } else {
            trace!(
                "Sent auth step2 to {} (challenge: 0x{:08x}, seq: {})",
                target_addr,
                challenge,
                packet.header.sequence
            );
        }
    }

    /// Handle an incoming Application Specific Data packet (Type 30).
    ///
    /// If it's a Bridge challenge (step=1), we respond with step=2 and mark
    /// the handshake as complete.
    pub(crate) async fn handle_auth_app_data(
        &self,
        data: &[u8],
        addr: SocketAddr,
        header: &ManagementHeader,
    ) {
        // Type 30 envelope layout:
        // [0..24) = Management Header
        // [24] = Data Identifier 1
        // [25] = Data Identifier 2
        // [26..30) = Data Size (u32 LE)
        // [30..34) = Total Packets (u32 LE)
        // [34..38) = Packet No (u32 LE)
        // [38..42) = Packet Signature (u32 LE)
        // [42..] = Payload

        if data.len() < 42 {
            return;
        }

        let data_size = u32::from_le_bytes(data[26..30].try_into().unwrap_or([0; 4])) as usize;
        if data.len() < 42 + data_size {
            return;
        }

        // All observed Arena/Bridge handshake payloads are exactly 20 bytes.
        if data_size != 20 {
            return;
        }

        let signature = u32::from_le_bytes(data[38..42].try_into().unwrap_or([0; 4]));
        let payload = &data[42..42 + data_size];

        // Payload layout (20 bytes):
        // [0..2) = step (u16 LE)
        // [2..4) = port (u16 LE)
        // [4..8) = challenge (u32 LE)
        // [8..12) = response (u32 LE)
        // [12..20) = trailer (8 bytes)
        let step = u16::from_le_bytes(payload[0..2].try_into().unwrap_or([0; 2]));

        // Check for Bridge challenge (Pioneer signature, step=1)
        if signature == PIONEER_SIGNATURE && step == 1 {
            let challenge = u32::from_le_bytes(payload[4..8].try_into().unwrap_or([0; 4]));
            info!(
                "Received auth challenge from {} (challenge: 0x{:08x})",
                addr, challenge
            );

            // Respond with step=2
            self.send_auth_step2(addr, challenge).await;

            // Mark handshake complete for this peer
            let peer_name = header.node_name_str();
            let key = NodeKey::new(addr, peer_name);
            self.auth_manager.mark_handshake_complete(key, addr).await;
        }
    }

    /// Initiate the authentication handshake with a peer if not already done.
    ///
    /// Called when we receive an Opt-IN from a peer that advertises NEED_AUTHENTICATION.
    pub(crate) async fn initiate_auth_handshake(
        &self,
        key: NodeKey,
        target_addr: SocketAddr,
        node_name: &str,
    ) {
        // Update target address in case it changed
        self.auth_manager.update_target_addr(&key, target_addr).await;

        // Check if we've already sent step=0
        if self.auth_manager.has_sent_step0(&key).await {
            return;
        }

        // Get or create peer state
        let (_, is_new) = self.auth_manager.get_or_create_peer(key.clone(), target_addr).await;
        if is_new {
            info!(
                "Initiating NEED_AUTHENTICATION handshake with {} (target: {})",
                node_name, target_addr
            );
        }

        // Send step=0
        self.send_auth_step0(target_addr).await;
        self.auth_manager.mark_step0_sent(&key).await;
    }

    /// Remove auth tracking for a peer (call on Opt-OUT).
    pub(crate) async fn remove_auth_peer(&self, key: &NodeKey) {
        self.auth_manager.remove_peer(key).await;
    }
}
