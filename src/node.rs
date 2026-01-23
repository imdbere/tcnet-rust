//! TCNet node implementation for network participation.
//!
//! A node represents this application's presence on the TCNet network.

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::atomic::{AtomicU8, AtomicU16, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;
use tokio::sync::{broadcast, Mutex};
use tracing::{debug, error, info, trace};

use crate::error::Result;
use crate::packets::{
    MixerDataPacket, OptInBuilder, OptInPacket, Packet, RequestDataType, RequestPacket,
    StatusPacket, TimePacket,
};
use crate::registry::{NodeKey, NodeRegistry, RegistryEvent, RemovalReason};
use crate::types::{
    NodeOptions, NodeType, PORT_BROADCAST_CONTROL, PORT_BROADCAST_TIME, PORT_UNICAST_DEFAULT,
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
            node_name: "SYNM".to_string(),
            node_type: NodeType::Slave,
            node_options: NodeOptions::new(),
            listener_port: PORT_UNICAST_DEFAULT,
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
    config: NodeConfig,
    node_id: u16,
    sequence: AtomicU8,
    node_count: AtomicU16,
    start_time: Instant,
    event_tx: broadcast::Sender<NodeEvent>,
    registry: Mutex<NodeRegistry>,
}

impl Node {
    /// Create a new TCNet node with the given configuration.
    pub fn new(config: NodeConfig) -> Self {
        // Generate a simple node ID from first 2 bytes of a timestamp
        let node_id = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() & 0xFFFF) as u16;

        let (event_tx, _) = broadcast::channel(256);

        Self {
            config,
            node_id,
            sequence: AtomicU8::new(0),
            node_count: AtomicU16::new(0),
            start_time: Instant::now(),
            event_tx,
            registry: Mutex::new(NodeRegistry::new()),
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
    fn timestamp_us(&self) -> u32 {
        let micros = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros();
        (micros % 1_000_000) as u32
    }

    /// Get and increment the sequence number.
    fn next_sequence(&self) -> u8 {
        self.sequence.fetch_add(1, Ordering::Relaxed)
    }

    /// Build an Opt-IN packet with current state.
    fn build_opt_in(&self) -> OptInPacket {
        OptInBuilder::new(&self.config.node_name)
            .node_id(self.node_id)
            .node_type(self.config.node_type)
            .node_options(self.config.node_options)
            .listener_port(self.config.listener_port)
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
                let _ = self.event_tx.send(NodeEvent::NodeLeft { node_name, reason });
            }
        }
    }

    /// Run the node, listening for packets and sending Opt-IN messages.
    pub async fn run(self: Arc<Self>) -> Result<()> {
        // Bind to broadcast ports with SO_REUSEADDR/SO_REUSEPORT
        // This allows coexistence with other TCNet apps (e.g., Pro DJ Link Bridge)
        info!("Binding to control broadcast port {} (with port reuse)", PORT_BROADCAST_CONTROL);
        let control_socket: UdpSocket = Self::create_reusable_socket(PORT_BROADCAST_CONTROL)?;

        info!("Binding to time broadcast port {} (with port reuse)", PORT_BROADCAST_TIME);
        let time_socket: UdpSocket = Self::create_reusable_socket(PORT_BROADCAST_TIME)?;

        // Bind to our unicast listener port for receiving responses to requests
        info!("Binding to unicast listener port {}", self.config.listener_port);
        let unicast_socket: UdpSocket = Self::create_reusable_socket(self.config.listener_port)?;

        // Socket for sending Opt-IN broadcasts (ephemeral port, no reuse needed)
        let send_socket: UdpSocket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)).await?;
        send_socket.set_broadcast(true)?;

        let broadcast_addr = SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::BROADCAST,
            PORT_BROADCAST_CONTROL,
        ));

        info!(
            "TCNet node '{}' starting on ports {} (control), {} (time), {} (unicast)",
            self.config.node_name, PORT_BROADCAST_CONTROL, PORT_BROADCAST_TIME, self.config.listener_port
        );

        // Spawn Opt-IN sender task
        let node_clone = Arc::clone(&self);
        let send_socket: Arc<UdpSocket> = Arc::new(send_socket);
        let send_socket_clone: Arc<UdpSocket> = Arc::clone(&send_socket);
        
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
                    node_clone.node_count.store(registry.len() as u16, Ordering::Relaxed);
                    events
                };
                for event in events {
                    node_clone.emit_registry_event(event);
                }
            }
        });

        // Spawn mixer data request task - requests mixer data from master/repeater nodes every second
        let node_clone = Arc::clone(&self);
        let send_socket_clone = Arc::clone(&send_socket);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;

                // Get list of master/repeater nodes with their listener ports
                let targets: Vec<(std::net::IpAddr, u16)> = {
                    let registry = node_clone.registry.lock().await;
                    registry
                        .nodes()
                        .filter(|n| {
                            n.node_type == NodeType::Master || n.node_type == NodeType::Repeater
                        })
                        .map(|n| (n.address.ip(), n.listener_port))
                        .collect()
                };

                // Send mixer data request to each target
                for (ip, port) in targets {
                    debug!("Sending mixer data request to {} ({})", ip, port);
                    let request = node_clone.build_request(RequestDataType::MixerData, 0);
                    let bytes = request.to_bytes();
                    let target_addr = SocketAddr::new(ip, port);

                    if let Err(e) = send_socket_clone.send_to(&bytes, target_addr).await {
                        debug!("Failed to send mixer data request to {}: {}", target_addr, e);
                    } else {
                        trace!("Sent mixer data request to {}", target_addr);
                    }
                }
            }
        });

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
        tokio::spawn(async move {
            let mut buf = [0u8; 4096]; // Larger buffer for data packets like waveforms
            loop {
                match unicast_socket.recv_from(&mut buf).await {
                    Ok((len, addr)) => {
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
                trace!(
                    "Received time packet from {} ({}) {:#?}",
                    time_packet.header.node_name_str(),
                    addr,
                    time_packet
                );
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
                
                // Also refresh the node in registry if we know about it
                // (Status packets prove the node is still alive)
                let key = NodeKey::new(addr, node_name);
                {
                    let mut registry = self.registry.lock().await;
                    registry.refresh_node(&key);
                }
                
                let _ = self.event_tx.send(NodeEvent::StatusPacket(status_packet));
            }
            Packet::OptIn(opt_in) => {
                let node_name = opt_in.header.node_name_str();
                let key = NodeKey::new(addr, node_name.clone());
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
                        key,
                        node_name,
                        opt_in.header.node_type,
                        opt_in.header.node_id,
                        opt_in.vendor_name_str(),
                        opt_in.app_name_str(),
                        opt_in.app_version_str(),
                        opt_in.listener_port,
                        addr,
                        opt_in.node_count,
                        opt_in.uptime_secs,
                    )
                };

                if let Some(event) = event {
                    self.emit_registry_event(event);
                }
            }
            Packet::OptOut(header) => {
                let node_name = header.node_name_str();
                info!(
                    "Received Opt-OUT from {} ({})",
                    node_name,
                    addr
                );

                // Remove from registry
                let key = NodeKey::new(addr, node_name);
                let event = {
                    let mut registry = self.registry.lock().await;
                    registry.remove_node(&key)
                };

                if let Some(event) = event {
                    self.emit_registry_event(event);
                }
            }
            Packet::MixerData(mixer_packet) => {
                debug!(
                    "Received mixer data from {} (mixer: {}) {:#?}",
                    mixer_packet.header.node_name_str(),
                    mixer_packet.mixer_name,
                    mixer_packet
                );
                let _ = self.event_tx.send(NodeEvent::MixerDataPacket(mixer_packet));
            }
            Packet::Unknown(header) => {
                debug!(
                    "Received unknown packet type {:?} from {}",
                    header.message_type, addr
                );
            }
        }

        Ok(())
    }
}
