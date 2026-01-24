//! TCNet Listener CLI
//!
//! A simple command-line tool that listens to TCNet time packets
//! and displays active layer timecodes.

use std::io::{self, Write};
use std::sync::Arc;

use clap::Parser;
use tcnet::{LayerState, Node, NodeConfig, NodeEvent, NodeType, TimePacket};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

/// TCNet Listener - Display real-time timecode from TCNet network
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Node name (max 8 characters)
    #[arg(short, long, default_value = "Arena")]
    name: String,

    /// Show all layers, not just active ones
    #[arg(short, long)]
    all_layers: bool,

    /// Debug output (show debug messages)
    #[arg(short, long)]
    debug: bool,

    /// Trace output (show trace messages)
    #[arg(short, long)]
    trace: bool,

    /// Show node join/leave events
    #[arg(short, long)]
    show_nodes: bool,

    /// Compact single-line output mode
    #[arg(short, long)]
    compact: bool,
}

/// Format a duration as MM:SS
fn format_duration_ms(ms: u32) -> String {
    let total_secs = ms / 1000;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    let millis = ms % 1000;
    format!("{:02}:{:02}.{:03}", mins, secs, millis)
}

/// Format layer state with color
fn format_state(state: LayerState) -> &'static str {
    match state {
        LayerState::Idle => "IDLE",
        LayerState::Playing => "\x1b[32mPLAY\x1b[0m",
        LayerState::Looping => "\x1b[33mLOOP\x1b[0m",
        LayerState::Paused => "\x1b[33mPAUS\x1b[0m",
        LayerState::Stopped => "\x1b[31mSTOP\x1b[0m",
        LayerState::CueButtonDown => "\x1b[36mCUE\x1b[0m",
        LayerState::PlatterDown => "\x1b[36mPLAT\x1b[0m",
        LayerState::FastForward => "\x1b[35mFFWD\x1b[0m",
        LayerState::FastReverse => "\x1b[35mFFRV\x1b[0m",
        LayerState::Hold => "\x1b[33mHOLD\x1b[0m",
    }
}

/// Print time packet data
fn print_time_packet(packet: &TimePacket, show_all: bool, compact: bool) {
    let active_layers: Vec<_> = if show_all {
        packet.layers.iter().collect()
    } else {
        packet.layers.iter().filter(|l| l.is_active()).collect()
    };

    if active_layers.is_empty() {
        return;
    }

    if compact {
        // Single line output
        let parts: Vec<String> = active_layers
            .iter()
            .map(|l| {
                format!(
                    "{}:{} {}",
                    l.layer,
                    l.timecode,
                    format_state(l.state)
                )
            })
            .collect();
        print!("\r\x1b[K{}", parts.join(" | "));
        io::stdout().flush().ok();
    } else {
        // Clear screen and print header
        print!("\x1b[2J\x1b[H");
        println!(
            "\x1b[1;36m╔═══════════════════════════════════════════════════════════════════╗\x1b[0m"
        );
        println!(
            "\x1b[1;36m║\x1b[0m  \x1b[1mTCNet Listener\x1b[0m - Source: \x1b[33m{:<8}\x1b[0m  SMPTE: \x1b[33m{}\x1b[0m              \x1b[1;36m║\x1b[0m",
            packet.header.node_name_str(),
            packet.smpte_mode
        );
        println!(
            "\x1b[1;36m╠═══════════════════════════════════════════════════════════════════╣\x1b[0m"
        );
        println!(
            "\x1b[1;36m║\x1b[0m  \x1b[1mLayer  Timecode      Position     Total      State  OnAir\x1b[0m    \x1b[1;36m║\x1b[0m"
        );
        println!(
            "\x1b[1;36m╠═══════════════════════════════════════════════════════════════════╣\x1b[0m"
        );

        for layer in active_layers {
            let on_air_indicator = if layer.on_air > 0 {
                format!("\x1b[32m{:3}\x1b[0m", layer.on_air)
            } else {
                "  -".to_string()
            };

            println!(
                "\x1b[1;36m║\x1b[0m  \x1b[1m{:<6}\x1b[0m \x1b[1;37m{}\x1b[0m   {}  {}   {}   {}   \x1b[1;36m║\x1b[0m",
                layer.layer,
                layer.timecode,
                format_duration_ms(layer.time_ms),
                format_duration_ms(layer.total_time_ms),
                format_state(layer.state),
                on_air_indicator
            );
        }

        println!(
            "\x1b[1;36m╚═══════════════════════════════════════════════════════════════════╝\x1b[0m"
        );
        println!("\n\x1b[90mPress Ctrl+C to exit\x1b[0m");
    }
}

#[tokio::main]
async fn main() -> tcnet::Result<()> {
    let args = Args::parse();

    // Initialize logging
    let filter = if args.trace {
        EnvFilter::new("trace")
    } else if args.debug {
        EnvFilter::new("debug")
    } else {
        EnvFilter::new("info")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();

    // Create node config
    let config = NodeConfig::new(&args.name)
        .with_type(NodeType::Slave)
        .with_vendor("Resolume")
        .with_app("Arena", (7, 23, 2));

    info!("Starting TCNet listener as node '{}'", args.name);
    info!("Listening for time packets on UDP port 60001");
    info!("Sending Opt-IN broadcasts on UDP port 60000");

    let node = Arc::new(Node::new(config));
    let mut events = node.subscribe();

    // Spawn event handler
    let show_all = args.all_layers;
    let compact = args.compact;
    let show_nodes = args.show_nodes;

    tokio::spawn(async move {
        let mut last_source: Option<String> = None;

        while let Ok(event) = events.recv().await {
            match event {
                NodeEvent::TimePacket(packet) => {
                    let source = packet.header.node_name_str();
                    
                    // Show source change
                    if show_nodes && last_source.as_ref() != Some(&source) {
                        if !compact {
                            info!("Receiving from: {}", source);
                        }
                        last_source = Some(source);
                    }

                    //print_time_packet(&packet, show_all, compact);
                }
                NodeEvent::StatusPacket(status) => {
                    if show_nodes {
                        info!(
                            "Status from {}: SMPTE={}, AutoMaster={}",
                            status.header.node_name_str(),
                            status.smpte_mode,
                            status.auto_master_mode_str()
                        );
                    }
                }
                NodeEvent::MetricsDataPacket(metrics) => {
                    if show_nodes {
                        info!(
                            "Metrics from {} [{}]: {} @ {:.2} BPM, pos={}, beat={}",
                            metrics.header.node_name_str(),
                            metrics.layer,
                            metrics.layer_state,
                            metrics.bpm(),
                            metrics.position_string(),
                            metrics.beat_number,
                        );
                    }
                }
                NodeEvent::MetadataPacket(metadata) => {
                    if show_nodes {
                        info!(
                            "Metadata from {} [{}]: {}",
                            metadata.header.node_name_str(),
                            metadata.layer,
                            metadata.display_string(),
                        );
                    }
                }
                NodeEvent::MixerDataPacket(mixer) => {
                    if show_nodes {
                        // Build channel summary
                        let active_channels: Vec<String> = mixer
                            .channels
                            .iter()
                            .filter(|ch| ch.fader_level > 0)
                            .map(|ch| format!("Ch{}:{}", ch.number, ch.fader_level))
                            .collect();

                        info!(
                            "Mixer from {} [{}]: master={}, xfader={}, channels=[{}]",
                            mixer.header.node_name_str(),
                            mixer.mixer_name,
                            mixer.master.fader_level,
                            mixer.crossfader.position,
                            active_channels.join(", ")
                        );
                    }
                }
                NodeEvent::NodeDiscovered {
                    node_name,
                    node_type,
                    vendor,
                    app,
                } => {
                    if show_nodes {
                        info!(
                            "\x1b[32m+ Node discovered:\x1b[0m {} ({}) - {} {}",
                            node_name, node_type, vendor, app
                        );
                    }
                }
                NodeEvent::NodeUpdated {
                    node_name,
                    node_type,
                    vendor,
                    app,
                } => {
                    if show_nodes {
                        info!(
                            "\x1b[33m~ Node updated:\x1b[0m {} ({}) - {} {}",
                            node_name, node_type, vendor, app
                        );
                    }
                }
                NodeEvent::NodeLeft { node_name, reason } => {
                    if show_nodes {
                        let reason_str = match reason {
                            tcnet::RemovalReason::OptOut => "sent Opt-OUT",
                            tcnet::RemovalReason::Timeout => "timed out",
                        };
                        info!("\x1b[31m- Node left:\x1b[0m {} ({})", node_name, reason_str);
                    }
                }
                NodeEvent::Error(msg) => {
                    error!("Error: {}", msg);
                }
            }
        }
    });

    // Run the node (this blocks)
    node.run().await
}
