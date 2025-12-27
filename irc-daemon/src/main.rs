//! IRC7 Daemon - Main entry point for the IRC server
//!
//! This is the main executable that runs the IRC7 server.

use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::Arc;

use clap::Parser;
use tokio::signal;
use tokio::sync::broadcast;
use tracing::{info, error, warn, Level};
use tracing_subscriber::FmtSubscriber;

use irc::io::{DataStore, SocketServer, SocketServerConfig};
use irc::objects::{Server, ServerConfig};
use irc::processor::{ServerProcessor, ProcessorConfig};
use irc::protocols::{create_irc_protocol, create_ircx_protocol};
use irc::security::SecurityManager;
use irc::enums::ProtocolType;
use irc::traits::{Connection, Server as ServerTrait, SocketServer as SocketServerTrait};

/// IRC7 Server Daemon
#[derive(Parser, Debug)]
#[command(name = "irc7d")]
#[command(author, version, about = "IRC7 Server Daemon", long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "./config.json")]
    config: String,

    /// IP address to bind on
    #[arg(short, long, default_value = "0.0.0.0")]
    ip: String,

    /// Port to bind on
    #[arg(short, long, default_value_t = 6667)]
    port: u16,

    /// Socket backlog size
    #[arg(short = 'k', long, default_value_t = 512)]
    backlog: u32,

    /// Maximum connections per IP
    #[arg(short, long, default_value_t = 128)]
    max_conn: usize,

    /// Input buffer size in bytes
    #[arg(short = 'z', long, default_value_t = 512)]
    buffer: usize,

    /// Fully qualified domain name
    #[arg(short, long, default_value = "localhost")]
    fqdn: String,

    /// Server type (IRC, IRCX, ACS, ADS)
    #[arg(short = 't', long, default_value = "ACS")]
    server_type: String,

    /// Chat server IP (for directory servers)
    #[arg(short, long)]
    server: Option<String>,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,
}

/// Server type enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
enum ServerType {
    /// Standard IRC server
    Irc,
    /// IRCX extended server
    IrcX,
    /// Access Control Server (Chat)
    Acs,
    /// Access Directory Server
    Ads,
}

impl From<&str> for ServerType {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "IRC" => Self::Irc,
            "IRCX" => Self::IrcX,
            "ACS" => Self::Acs,
            "ADS" => Self::Ads,
            _ => Self::Acs,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.debug { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Parse bind address
    let ip: IpAddr = args.ip.parse()?;
    let addr = SocketAddr::new(ip, args.port);

    // Determine server type
    let server_type = ServerType::from(args.server_type.as_str());
    let is_directory_server = matches!(server_type, ServerType::Ads);

    // Display startup banner
    display_banner(&args, addr, server_type);

    // Load data store
    let data_store = if Path::new("DefaultServer.json").exists() {
        match DataStore::from_file("DefaultServer.json").await {
            Ok(ds) => Arc::new(ds),
            Err(e) => {
                warn!("Failed to load data store: {}, using default", e);
                Arc::new(DataStore::new())
            }
        }
    } else {
        Arc::new(DataStore::new())
    };

    // Create server configuration
    let config = ServerConfig {
        name: args.fqdn.clone(),
        title: "IRC7 Server".to_string(),
        max_connections: args.max_conn,
        max_input_bytes: args.buffer,
        ..Default::default()
    };

    // Create security manager
    let security_manager = Arc::new(SecurityManager::new());

    // Create the server
    let mut server = if is_directory_server {
        Server::new_directory_server(config, security_manager.clone(), data_store.clone(), None)
    } else {
        Server::new(config, security_manager.clone(), data_store.clone(), None)
    };

    // Add protocols based on server type
    match server_type {
        ServerType::Irc => {
            let irc_protocol = create_irc_protocol();
            server.add_protocol(ProtocolType::Irc, Arc::from(irc_protocol));
        }
        ServerType::IrcX | ServerType::Acs | ServerType::Ads => {
            let irc_protocol = create_irc_protocol();
            let ircx_protocol = create_ircx_protocol();
            server.add_protocol(ProtocolType::Irc, Arc::from(irc_protocol));
            server.add_protocol(ProtocolType::IrcX, Arc::from(ircx_protocol));
        }
    }

    // Configure socket server
    let socket_config = SocketServerConfig {
        addr,
        backlog: args.backlog,
        max_connections_per_ip: args.max_conn,
        max_connections: 10000,
        buffer_size: args.buffer,
    };

    let socket_server = Arc::new(SocketServer::new(socket_config));
    let server = Arc::new(parking_lot::RwLock::new(server));

    // Create shutdown channel
    let (shutdown_tx, _shutdown_rx) = broadcast::channel::<()>(1);

    // Set up connection handler
    let server_for_handler = server.clone();
    socket_server.on_connection(move |connection| {
        let ip = connection.get_ip().to_string();
        info!("New connection from {}", ip);

        // In a full implementation, this would:
        // 1. Create a User object wrapping the connection
        // 2. Add the user to the server
        // 3. The processor loop will pick up messages from the user's data regulator

        let _server = server_for_handler.read();
        // let user = server.create_user(connection);
        // server.add_user(user);
    });

    // Start the socket server
    let socket_server_clone = socket_server.clone();
    let socket_task = tokio::spawn(async move {
        if let Err(e) = socket_server_clone.listen().await {
            error!("Socket server error: {}", e);
        }
    });

    // Create and start the processor
    let processor = ServerProcessor::new(ProcessorConfig {
        tick_rate_ms: 10,
        max_backoff_ms: 1000,
        backoff_increment_ms: 10,
    });

    // Note: The processor would be started here in a full implementation
    // For now, we just run the socket server and wait for shutdown

    info!("IRC7 Server started on {}", addr);
    info!("Press Ctrl+C to shutdown");

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutdown signal received");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Shutdown sequence
    info!("Shutting down...");

    // Stop accepting new connections
    socket_server.stop().await;

    // Signal all tasks to stop
    let _ = shutdown_tx.send(());

    // Shutdown the server (disconnects all users)
    server.write().shutdown();

    // Wait for tasks to complete
    let _ = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        socket_task,
    ).await;

    info!("Goodbye!");
    Ok(())
}

fn display_banner(args: &Args, addr: SocketAddr, server_type: ServerType) {
    let version = env!("CARGO_PKG_VERSION");

    println!();
    println!("╔════════════════════════════════════════╗");
    println!("║            IRC7 Server v{}            ║", version);
    println!("╠════════════════════════════════════════╣");
    println!("║ Listening: {:>28} ║", format!("{}:{}", addr.ip(), addr.port()));
    println!("║ Type: {:>33} ║", format!("{:?}", server_type).to_uppercase());
    println!("║ FQDN: {:>33} ║", &args.fqdn[..args.fqdn.len().min(33)]);
    println!("║ Max Connections: {:>22} ║", args.max_conn);
    println!("║ Buffer Size: {:>22} B ║", args.buffer);
    if let Some(ref chat_server) = args.server {
        println!("║ Directory Server: {:>21} ║", chat_server);
    }
    println!("╚════════════════════════════════════════╝");
    println!();
}
