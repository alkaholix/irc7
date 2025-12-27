//! IRC7 Daemon - Main entry point for the IRC server
//!
//! This is the main executable that runs the IRC7 server.

use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::Arc;

use clap::Parser;
use tokio::signal;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

use irc::io::{DataStore, FloodProtectionManager, SocketServer, SocketServerConfig};
use irc::objects::{Server, ServerConfig};
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
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Parse command line arguments
    let args = Args::parse();

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
        Arc::new(DataStore::from_file("DefaultServer.json").await?)
    } else {
        Arc::new(DataStore::new())
    };

    // Create server configuration
    let config = ServerConfig {
        name: args.fqdn.clone(),
        title: "IRC7 Server".to_string(),
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

    // Add protocols
    let irc_protocol = create_irc_protocol();
    let ircx_protocol = create_ircx_protocol();
    server.add_protocol(ProtocolType::Irc, Arc::from(irc_protocol));
    server.add_protocol(ProtocolType::IrcX, Arc::from(ircx_protocol));

    // Configure socket server
    let socket_config = SocketServerConfig {
        addr,
        backlog: args.backlog,
        max_connections_per_ip: args.max_conn,
        max_connections: 10000,
        buffer_size: args.buffer,
    };

    let socket_server = Arc::new(SocketServer::new(socket_config));

    // Set up connection handler
    let server = Arc::new(parking_lot::RwLock::new(server));
    let server_clone = server.clone();

    socket_server.on_connection(move |connection| {
        info!("New connection from {}", connection.get_ip());
        // Handle new connection
        // In a full implementation, would create a User and add to server
    });

    // Start the socket server in a separate task
    let socket_server_clone = socket_server.clone();
    let server_task = tokio::spawn(async move {
        if let Err(e) = socket_server_clone.listen().await {
            error!("Socket server error: {}", e);
        }
    });

    info!("IRC7 Server started");

    // Wait for shutdown signal
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Shutdown signal received");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Shutdown
    info!("Shutting down...");
    socket_server.stop().await;
    server.write().shutdown();

    // Wait for server task to complete
    let _ = server_task.await;

    info!("Goodbye!");
    Ok(())
}

fn display_banner(args: &Args, addr: SocketAddr, server_type: ServerType) {
    let version = env!("CARGO_PKG_VERSION");

    println!("╔════════════════════════════════════════╗");
    println!("║            IRC7 Server Info            ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ Server Version: {:24}║", version);
    println!("║ Listening on IP: {:23}║", addr.ip());
    println!("║ Port: {:34}║", args.port);
    println!("║ Max Connections: {:23}║", args.max_conn);
    println!("║ Server Type: {:27}║", format!("{:?}", server_type).to_uppercase());
    println!("║ FQDN: {:34}║", args.fqdn);
    println!("║ Buffer Size: {:23} bytes ║", args.buffer);
    println!("║ Backlog Size: {:26}║", args.backlog);
    if let Some(ref chat_server) = args.server {
        println!("║ Chat Server IP: {:24}║", chat_server);
    }
    println!("╚════════════════════════════════════════╝");
}
