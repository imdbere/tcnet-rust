//! TCNet node implementation for network participation.
//!
//! A node represents this application's presence on the TCNet network.

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::atomic::{AtomicU16, AtomicU8, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, Mutex};
use tracing::{debug, error, info, trace};

use crate::auth::AuthManager;
use crate::error::Result;
use crate::packets::{
    ErrorNotificationPacket, MetadataPacket, MetricsDataPacket, MixerDataPacket, OptInBuilder,
    OptInPacket, Packet, RequestDataType, RequestPacket, StatusPacket, TimePacket,
};
use crate::registry::{NodeKey, NodeRegistry, RegistryEvent, RemovalReason};
use crate::types::{
    MessageType, NodeOptions, NodeType, PORT_BROADCAST_CONTROL, PORT_BROADCAST_TIME,
    PORT_UNICAST_DEFAULT,
};

/// Configuration for a TCNet node.
#[derive(Debug, Clone)]
pub struct NodeConfig {
    /// Node name (max 8 characters)
    pub node_name: String,
    /// Node type (Slave recommended for listeners)
    pub node_type: NodeType,
    /// Node options flags
    pub node_options: NodeOptions,
    /// Port to listen on for unicast messages
    pub listener_port: u16,
    /// Vendor name
    pub vendor_name: String,
    /// Application name
    pub app_name: String,
    /// Application version (major, minor, bug)
    pub app_version: (u8, u8, u8),
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            node_name: "Synm".to_string(),
            node_type: NodeType::Slave,
            node_options: NodeOptions::new(),
            // 0 means "auto-select a free port in the TCNet unicast range"
            listener_port: 0,
            vendor_name: "tcnet-rust".to_string(),
            app_name: "TCNet Listener".to_string(),
            app_version: (0, 1, 0),
        }
    }
}

impl NodeConfig {
    pub fn new(node_name: &str) -> Self {
        Self {
            node_name: node_name.chars().take(8).collect(),
            ..Default::default()
        }
    }

    pub fn with_type(mut self, node_type: NodeType) -> Self {
        self.node_type = node_type;
        self
    }

    pub fn with_vendor(mut self, vendor: &str) -> Self {
        self.vendor_name = vendor.to_string();
        self
    }

    pub fn with_app(mut self, name: &str, version: (u8, u8, u8)) -> Self {
        self.app_name = name.to_string();
        self.app_version = version;
        self
    }
}

/// Events emitted by the TCNet node.
#[derive(Debug, Clone)]
pub enum NodeEvent {
    /// Received a time packet
    TimePacket(TimePacket),
    /// Received a status packet
    StatusPacket(StatusPacket),
    /// Received an error/notification packet
    ErrorNotificationPacket(ErrorNotificationPacket),
    /// Received a metrics data packet
    MetricsDataPacket(MetricsDataPacket),
    /// Received a metadata packet
    MetadataPacket(MetadataPacket),
    /// Received a mixer data packet
    MixerDataPacket(MixerDataPacket),
    /// A new node was discovered on the network
    NodeDiscovered {
        node_name: String,
        node_type: NodeType,
        vendor: String,
        app: String,
    },
    /// A known node's information was updated
    NodeUpdated {
        node_name: String,
        node_type: NodeType,
        vendor: String,
        app: String,
    },
    /// A node left the network (timeout or Opt-OUT)
    NodeLeft {
        node_name: String,
        reason: RemovalReason,
    },
    /// Error occurred
    Error(String),
}

/// A TCNet node that participates in the network.
pub struct Node {
    pub(crate) config: NodeConfig,
    pub(crate) node_id: u16,
    sequence: AtomicU8,
    node_count: AtomicU16,
    /// Actual bound unicast listener port (advertised in Opt-IN).
    ///
    /// If `config.listener_port` is 0 or unavailable, we auto-select a free port in the
    /// TCNet unicast range (65023-65535) at startup.
    pub(crate) listener_port: AtomicU16,
    start_time: Instant,
    event_tx: broadcast::Sender<NodeEvent>,
    registry: Mutex<NodeRegistry>,
    /// Manages authentication handshakes with peers that require it.
    pub(crate) auth_manager: AuthManager,
    /// Socket for sending unicast subscription packets (set during run())
    pub(crate) unicast_socket: Mutex<Option<Arc<UdpSocket>>>,
}

impl Node {
    /// Create a new TCNet node with the given configuration.
    pub fn new(config: NodeConfig) -> Self {
        // Generate a simple node ID from first 2 bytes of a timestamp
        let node_id = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros()
            & 0xFFFF) as u16;

        let (event_tx, _) = broadcast::channel(256);

        Self {
            listener_port: AtomicU16::new(config.listener_port),
            config,
            node_id,
            sequence: AtomicU8::new(0),
            node_count: AtomicU16::new(0),
            start_time: Instant::now(),
            event_tx,
            registry: Mutex::new(NodeRegistry::new()),
            auth_manager: AuthManager::new(),
            unicast_socket: Mutex::new(None),
        }
    }

    /// Subscribe to node events.
    pub fn subscribe(&self) -> broadcast::Receiver<NodeEvent> {
        self.event_tx.subscribe()
    }

    /// Get the current uptime in seconds (rolls over every 12 hours as per spec).
    fn uptime_secs(&self) -> u16 {
        let secs = self.start_time.elapsed().as_secs();
        (secs % 43200) as u16 // 12 hours = 43200 seconds
    }

    /// Get the current timestamp in microseconds (0-999999).
    pub(crate) fn timestamp_us(&self) -> u32 {
        let micros = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        (micros % 1_000_000) as u32
    }

    /// Get and increment the sequence number.
    pub(crate) fn next_sequence(&self) -> u8 {
        self.sequence.fetch_add(1, Ordering::Relaxed)
    }

    /// Build an Opt-IN packet with current state.
    fn build_opt_in(&self) -> OptInPacket {
        OptInBuilder::new(&self.config.node_name)
            .node_id(self.node_id)
            .node_type(self.config.node_type)
            .node_options(self.config.node_options)
            .listener_port(self.listener_port.load(Ordering::Relaxed))
            .vendor(&self.config.vendor_name)
            .app_name(&self.config.app_name)
            .app_version(
                self.config.app_version.0,
                self.config.app_version.1,
                self.config.app_version.2,
            )
            .build(
                self.next_sequence(),
                self.timestamp_us(),
                self.node_count.load(Ordering::Relaxed),
                self.uptime_secs(),
            )
    }

    /// Build a request packet for specific data type.
    fn build_request(&self, data_type: RequestDataType, layer: u8) -> RequestPacket {
        RequestPacket::new(
            self.node_id,
            &self.config.node_name,
            self.next_sequence(),
            self.config.node_type,
            self.config.node_options,
            self.timestamp_us(),
            data_type,
            layer,
        )
    }

    async fn send_opt_in_unicast(&self, socket: &UdpSocket, target_addr: SocketAddr) {
        let packet = self.build_opt_in();
        if let Err(e) = socket.send_to(&packet.to_bytes(), target_addr).await {
            debug!("Failed to unicast Opt-IN to {}: {}", target_addr, e);
        } else {
            trace!(
                "Unicasted Opt-IN to {} (seq: {}, listener_port: {})",
                target_addr,
                packet.header.sequence,
                self.listener_port.load(Ordering::Relaxed)
            );
        }
    }

    /// Send subscription packets to a node to start receiving mixer data.
    /// This mimics what Resolume does: sends app data + request packets for types 2 and 4.
    async fn send_subscriptions(&self, target_addr: SocketAddr) {
        let socket_guard = self.unicast_socket.lock().await;
        let Some(socket) = socket_guard.as_ref() else {
            debug!("Cannot send subscriptions: unicast socket not initialized");
            return;
        };

        info!("Sending subscriptions to {}", target_addr);

        // // Send request for layer status (type 2)
        let request_status = self.build_request(RequestDataType::LayerStatus, 1);
        if let Err(e) = socket.send_to(&request_status.to_bytes(), target_addr).await {
            debug!("Failed to send layer status request to {}: {}", target_addr, e);
        }

        // // Send request for metadata (type 4)
        for layer in 1..8 {
            let request_metadata = self.build_request(RequestDataType::Metadata, layer);
            if let Err(e) = socket
                .send_to(&request_metadata.to_bytes(), target_addr)
                .await
            {
                debug!("Failed to send metadata request to {}: {}", target_addr, e);
            }
        }

        // // // Also send explicit mixer data request
        // let request_mixer = self.build_request(RequestDataType::MixerData, 0);
        // if let Err(e) = socket.send_to(&request_mixer.to_bytes(), target_addr).await {
        //     debug!("Failed to send mixer data request to {}: {}", target_addr, e);
        // }

        debug!("Sent all subscription packets to {}", target_addr);
    }

    /// Create a UDP socket with SO_REUSEADDR and SO_REUSEPORT for sharing ports with other apps.
    /// This allows multiple applications (like Pro DJ Link Bridge) to receive broadcasts on the same port.
    fn create_reusable_socket(port: u16) -> std::io::Result<UdpSocket> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        // Allow address reuse - required for multiple processes to bind to same port
        socket.set_reuse_address(true)?;

        // Enable SO_REUSEPORT - allows multiple sockets to receive the same broadcast packets
        // This is key for coexisting with other TCNet apps like Pro DJ Link Bridge
        socket.set_reuse_port(true)?;

        // Enable broadcast receiving
        socket.set_broadcast(true)?;

        // Set non-blocking for tokio compatibility
        socket.set_nonblocking(true)?;

        // Bind to the port
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
        socket.bind(&addr.into())?;

        // Convert socket2::Socket -> std::net::UdpSocket -> tokio::net::UdpSocket
        let std_socket: std::net::UdpSocket = socket.into();
        UdpSocket::from_std(std_socket)
    }

    /// Create a unicast socket bound to a specific port (no SO_REUSEPORT).
    ///
    /// For TCNet unicast traffic the listener port should be unique and free in the
    /// range 65023-65535.
    fn create_unicast_socket(port: u16) -> std::io::Result<UdpSocket> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        // Allow address reuse (harmless for UDP), but do NOT enable SO_REUSEPORT here:
        // we want an actually free, unique port for unicast replies.
        socket.set_reuse_address(true)?;
        socket.set_nonblocking(true)?;

        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
        socket.bind(&addr.into())?;

        let std_socket: std::net::UdpSocket = socket.into();
        UdpSocket::from_std(std_socket)
    }

    /// Select a free unicast port in the TCNet range (65023-65535) and bind to it.
    ///
    /// - If `preferred_port` is non-zero and within range, we try it first.
    /// - Otherwise, we scan the range until we find a free port.
    fn bind_unicast_in_tcnet_range(preferred_port: u16) -> std::io::Result<(UdpSocket, u16)> {
        let start = PORT_UNICAST_DEFAULT;
        let end = crate::types::PORT_UNICAST_MAX;

        let mut candidates: Vec<u16> = Vec::with_capacity((end - start + 1) as usize);
        if preferred_port != 0 && (start..=end).contains(&preferred_port) {
            candidates.push(preferred_port);
        }
        for port in start..=end {
            if port != preferred_port {
                candidates.push(port);
            }
        }

        let mut last_err: Option<std::io::Error> = None;
        for port in candidates {
            match Self::create_unicast_socket(port) {
                Ok(sock) => return Ok((sock, port)),
                Err(e) => last_err = Some(e),
            }
        }

        Err(last_err.unwrap_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "no free unicast port found")
        }))
    }

    /// Emit events from registry changes.
    fn emit_registry_event(&self, event: RegistryEvent) {
        match event {
            RegistryEvent::NodeDiscovered(info) => {
                let _ = self.event_tx.send(NodeEvent::NodeDiscovered {
                    node_name: info.node_name,
                    node_type: info.node_type,
                    vendor: info.vendor,
                    app: info.app,
                });
            }
            RegistryEvent::NodeUpdated(info) => {
                let _ = self.event_tx.send(NodeEvent::NodeUpdated {
                    node_name: info.node_name,
                    node_type: info.node_type,
                    vendor: info.vendor,
                    app: info.app,
                });
            }
            RegistryEvent::NodeRemoved { node_name, reason } => {
                let _ = self
                    .event_tx
                    .send(NodeEvent::NodeLeft { node_name, reason });
            }
        }
    }

    /// Run the node, listening for packets and sending Opt-IN messages.
    pub async fn run(self: Arc<Self>) -> Result<()> {
        // Bind to broadcast ports with SO_REUSEADDR/SO_REUSEPORT
        // This allows coexistence with other TCNet apps (e.g., Pro DJ Link Bridge)
        info!(
            "Binding to control broadcast port {} (with port reuse)",
            PORT_BROADCAST_CONTROL
        );
        let control_socket: UdpSocket = Self::create_reusable_socket(PORT_BROADCAST_CONTROL)?;

        info!(
            "Binding to time broadcast port {} (with port reuse)",
            PORT_BROADCAST_TIME
        );
        let time_socket: UdpSocket = Self::create_reusable_socket(PORT_BROADCAST_TIME)?;

        // Bind to a free unicast listener port in the spec range (65023-65535)
        let preferred = self.config.listener_port;
        let (unicast_socket, bound_port) = Self::bind_unicast_in_tcnet_range(preferred)?;
        self.listener_port.store(bound_port, Ordering::Relaxed);
        info!("Bound unicast listener port {}", bound_port);

        let unicast_socket: Arc<UdpSocket> = Arc::new(unicast_socket);

        // Store unicast socket for sending subscriptions
        {
            let mut socket_guard = self.unicast_socket.lock().await;
            *socket_guard = Some(Arc::clone(&unicast_socket));
        }

        // Socket for sending Opt-IN broadcasts (ephemeral port, no reuse needed)
        let send_socket: UdpSocket =
            UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).await?;
        send_socket.set_broadcast(true)?;
        let send_socket = Arc::new(send_socket);

        let broadcast_addr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::BROADCAST,
            PORT_BROADCAST_CONTROL,
        ));

        info!(
            "TCNet node '{}' starting on ports {} (control), {} (time), {} (unicast)",
            self.config.node_name,
            PORT_BROADCAST_CONTROL,
            PORT_BROADCAST_TIME,
            self.listener_port.load(Ordering::Relaxed)
        );

        // Send an Opt-IN immediately so peers learn our chosen unicast port ASAP.
        // Some devices appear to reply (unicast) based on the listener port advertised in Opt-IN,
        // not the UDP source port of the first application-specific packet.
        {
            let packet = self.build_opt_in();
            if let Err(e) = send_socket.send_to(&packet.to_bytes(), broadcast_addr).await {
                error!("Failed to send initial Opt-IN: {}", e);
            } else {
                trace!(
                    "Sent initial Opt-IN packet (seq: {}, listener_port: {})",
                    packet.header.sequence,
                    self.listener_port.load(Ordering::Relaxed)
                );
            }
        }

        // Spawn Opt-IN sender task
        let node_clone = Arc::clone(&self);
        let send_socket_clone = Arc::clone(&send_socket);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(500));
            loop {
                interval.tick().await;
                let packet = node_clone.build_opt_in();
                let bytes = packet.to_bytes();

                if let Err(e) = send_socket_clone.send_to(&bytes, broadcast_addr).await {
                    error!("Failed to send Opt-IN: {}", e);
                } else {
                    trace!("Sent Opt-IN packet (seq: {})", packet.header.sequence);
                }
            }
        });

        // Unicast Opt-IN to discovered nodes (spec-recommended) so nodes that don't receive
        // broadcasts still learn our listener port.
        let node_clone = Arc::clone(&self);
        let send_socket_clone = Arc::clone(&send_socket);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let targets = {
                    let registry = node_clone.registry.lock().await;
                    registry
                        .nodes()
                        .map(|n| SocketAddr::new(n.address.ip(), n.listener_port))
                        .collect::<Vec<_>>()
                };

                for target in targets {
                    node_clone.send_opt_in_unicast(&send_socket_clone, target).await;
                }
            }
        });

        // Spawn registry cleanup task (remove stale nodes)
        let node_clone = Arc::clone(&self);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                let events = {
                    let mut registry = node_clone.registry.lock().await;
                    let events = registry.cleanup_stale_nodes();
                    // Update our node count
                    node_clone
                        .node_count
                        .store(registry.len() as u16, Ordering::Relaxed);
                    events
                };
                for event in events {
                    node_clone.emit_registry_event(event);
                }
            }
        });

        // Spawn Arena-style keepalive: every 1s, send step0 ONLY to peers where the
        // NEED_AUTHENTICATION handshake has completed.
        // let node_clone = Arc::clone(&self);
        // tokio::spawn(async move {
        //     let mut interval = tokio::time::interval(Duration::from_secs(1));
        //     loop {
        //         interval.tick().await;
        //         let targets = {
        //             let peers = node_clone.auth_peers.lock().await;
        //             peers
        //                 .values()
        //                 .filter(|p| p.handshake_complete)
        //                 .map(|p| p.target_addr)
        //                 .collect::<Vec<_>>()
        //         };

        //         for target in targets {
        //             node_clone.send_arena_a6ff_step0_unicast(target).await;
        //         }
        //     }
        // });

        // Spawn control port listener
        let node_clone = Arc::clone(&self);
        tokio::spawn(async move {
            let mut buf = [0u8; 1500];
            loop {
                match control_socket.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        // Log raw packet type for debugging
                        if len >= 8 {
                            trace!("Control packet from {}: len={}, type={}", addr, len, buf[7]);
                        }
                        if let Err(e) = node_clone.handle_packet(&buf[..len], addr).await {
                            debug!("Failed to parse control packet from {}: {}", addr, e);
                        }
                    }
                    Err(e) => {
                        error!("Control socket error: {}", e);
                    }
                }
            }
        });

        // Spawn unicast listener for responses to request packets
        let node_clone = Arc::clone(&self);
        let unicast_socket_clone = Arc::clone(&unicast_socket);
        tokio::spawn(async move {
            let mut buf = [0u8; 4096]; // Larger buffer for data packets like waveforms
            loop {
                match unicast_socket_clone.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
                        trace!("Received unicast packet from {} (len: {})", addr, len);
                        if len >= 8 {
                            trace!("Unicast packet from {}: len={}, type={}", addr, len, buf[7]);
                        }
                        if let Err(e) = node_clone.handle_packet(&buf[..len], addr).await {
                            debug!("Failed to parse unicast packet from {}: {}", addr, e);
                        }
                    }
                    Err(e) => {
                        error!("Unicast socket error: {}", e);
                    }
                }
            }
        });

        // Main loop: listen for time packets
        let mut buf = [0u8; 1500];
        loop {
            match time_socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    //debug!("Received time packet from {}", addr);
                    if let Err(e) = self.handle_packet(&buf[..len], addr).await {
                        debug!("Failed to parse time packet from {}: {}", addr, e);
                    }
                }
                Err(e) => {
                    error!("Time socket error: {}", e);
                    return Err(crate::error::TcNetError::Io(e));
                }
            }
        }
    }

    /// Handle an incoming packet.
    async fn handle_packet(&self, data: &[u8], addr: SocketAddr) -> Result<()> {
        let packet = Packet::parse(data)?;

        match packet {
            Packet::Time(time_packet) => {
                debug!(
                    "Received time packet from {} ({})",
                    time_packet.header.node_name_str(),
                    addr
                );
                // trace!(
                //     "Received time packet from {} ({}) {:#?}",
                //     time_packet.header.node_name_str(),
                //     addr,
                //     time_packet
                // );
                let _ = self.event_tx.send(NodeEvent::TimePacket(time_packet));
            }
            Packet::Status(status_packet) => {
                let node_name = status_packet.header.node_name_str();
                trace!(
                    "Received status packet from {} (addr: {}) {:#?}",
                    node_name,
                    addr,
                    status_packet
                );
                // trace!("Received status packet from {} (addr: {})", node_name, addr);

                // Also refresh the node in registry if we know about it
                // (Status packets prove the node is still alive)
                let key = NodeKey::new(addr, node_name);
                {
                    let mut registry = self.registry.lock().await;
                    registry.refresh_node(&key);
                }

                let _ = self.event_tx.send(NodeEvent::StatusPacket(status_packet));
            }
            Packet::ErrorNotification(error_packet) => {
                debug!(
                    "Received error/notification from {}: {} (code: {}, request: {})",
                    error_packet.header.node_name_str(),
                    error_packet.request_description(),
                    error_packet.code,
                    error_packet.request_message_type,
                );
                let _ = self
                    .event_tx
                    .send(NodeEvent::ErrorNotificationPacket(error_packet));
            }
            Packet::OptIn(opt_in) => {
                let node_name = opt_in.header.node_name_str();
                let key = NodeKey::new(addr, node_name.clone());
                let node_type = opt_in.header.node_type;
                let listener_port = opt_in.listener_port;
                trace!(
                    "Received Opt-IN from {} (key: {:?}, addr: {}) - {} {}",
                    node_name,
                    key,
                    addr,
                    opt_in.vendor_name_str(),
                    opt_in.app_name_str()
                );

                // Update registry
                let event = {
                    let mut registry = self.registry.lock().await;
                    registry.update_node(
                        key.clone(),
                        node_name,
                        node_type,
                        opt_in.header.node_id,
                        opt_in.vendor_name_str(),
                        opt_in.app_name_str(),
                        opt_in.app_version_str(),
                        listener_port,
                        addr,
                        opt_in.node_count,
                        opt_in.uptime_secs,
                    )
                };

                if let Some(ref event) = event {
                    self.emit_registry_event(event.clone());

                    // If a new Master/Repeater was discovered, send subscription packets
                    if matches!(event, RegistryEvent::NodeDiscovered(_))
                        && (node_type == NodeType::Master || node_type == NodeType::Repeater)
                    {
                        let target_addr = SocketAddr::new(addr.ip(), listener_port);
                        self.send_subscriptions(target_addr).await;
                    }
                }

                // If the peer advertises NEED_AUTHENTICATION (flag 1), initiate the Arena-style
                // handshake once per peer, then switch to keepalives after completion.
                if opt_in.header.node_options.needs_auth() {
                    let target_addr = SocketAddr::new(addr.ip(), listener_port);
                    self.initiate_auth_handshake(key, target_addr, &opt_in.header.node_name_str())
                        .await;
                }
            }
            Packet::OptOut(header) => {
                let node_name = header.node_name_str();
                info!("Received Opt-OUT from {} ({})", node_name, addr);

                let key = NodeKey::new(addr, node_name.clone());

                // Remove auth tracking for this peer
                self.remove_auth_peer(&key).await;

                // Remove from registry
                let event = {
                    let mut registry = self.registry.lock().await;
                    registry.remove_node(&key)
                };

                if let Some(event) = event {
                    self.emit_registry_event(event);
                }
            }
            Packet::MetricsData(metrics_packet) => {
                trace!(
                    "Received metrics data from {} (layer: {}, bpm: {:.2})",
                    metrics_packet.header.node_name_str(),
                    metrics_packet.layer,
                    metrics_packet.bpm(),
                );
                let _ = self
                    .event_tx
                    .send(NodeEvent::MetricsDataPacket(metrics_packet));
            }
            Packet::Metadata(metadata_packet) => {
                debug!(
                    "Received metadata from {} (layer: {}, track: {})",
                    metadata_packet.header.node_name_str(),
                    metadata_packet.layer,
                    metadata_packet.display_string()
                );
                let _ = self
                    .event_tx
                    .send(NodeEvent::MetadataPacket(metadata_packet));
            }
            Packet::MixerData(mixer_packet) => {
                // debug!(
                //     "Received mixer data from {} (mixer: {})",
                //     mixer_packet.header.node_name_str(),
                //     mixer_packet.mixer_name,
                // );
                // debug!(
                //     "Received mixer data from {} (mixer: {}) {:#?}",
                //     mixer_packet.header.node_name_str(),
                //     mixer_packet.mixer_name,
                //     mixer_packet
                // );
                let _ = self.event_tx.send(NodeEvent::MixerDataPacket(mixer_packet));
            }
            Packet::Unknown(header) => {
                if header.message_type == MessageType::ApplicationData {
                    trace!("Received AppData packet from {} (addr: {})", header.node_name_str(), addr);
                    self.handle_auth_app_data(data, addr, &header).await;
                } else {
                    debug!(
                        "Received unknown packet type {:?} from {}",
                        header.message_type, addr
                    );
                }
            }
        }

        Ok(())
    }
}
