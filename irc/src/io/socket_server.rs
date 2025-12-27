//! TCP socket server implementation

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

use crate::io::TcpConnection;
use crate::traits::SocketServer as SocketServerTrait;

/// Socket server configuration
#[derive(Debug, Clone)]
pub struct SocketServerConfig {
    /// Bind address
    pub addr: SocketAddr,
    /// Connection backlog
    pub backlog: u32,
    /// Maximum connections per IP
    pub max_connections_per_ip: usize,
    /// Maximum total connections
    pub max_connections: usize,
    /// Input buffer size
    pub buffer_size: usize,
}

impl Default for SocketServerConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:6667".parse().unwrap(),
            backlog: 512,
            max_connections_per_ip: 128,
            max_connections: 10000,
            buffer_size: 512,
        }
    }
}

/// Connection event handler
pub type OnConnectionCallback = Box<dyn Fn(Arc<TcpConnection>) + Send + Sync>;

/// Socket server implementation
pub struct SocketServer {
    config: SocketServerConfig,
    running: Arc<RwLock<bool>>,
    connections_per_ip: Arc<RwLock<HashMap<String, usize>>>,
    connection_count: Arc<RwLock<usize>>,
    on_connection: Arc<RwLock<Option<OnConnectionCallback>>>,
}

impl SocketServer {
    /// Create a new socket server
    pub fn new(config: SocketServerConfig) -> Self {
        Self {
            config,
            running: Arc::new(RwLock::new(false)),
            connections_per_ip: Arc::new(RwLock::new(HashMap::new())),
            connection_count: Arc::new(RwLock::new(0)),
            on_connection: Arc::new(RwLock::new(None)),
        }
    }

    /// Set the connection callback
    pub fn on_connection<F>(&self, callback: F)
    where
        F: Fn(Arc<TcpConnection>) + Send + Sync + 'static,
    {
        *self.on_connection.write() = Some(Box::new(callback));
    }

    /// Check if an IP can connect
    fn can_accept_ip(&self, ip: &str) -> bool {
        let per_ip = self.connections_per_ip.read();
        let count = per_ip.get(ip).copied().unwrap_or(0);
        count < self.config.max_connections_per_ip
    }

    /// Register a connection
    fn register_connection(&self, ip: &str) {
        *self.connection_count.write() += 1;
        let mut per_ip = self.connections_per_ip.write();
        *per_ip.entry(ip.to_string()).or_insert(0) += 1;
    }

    /// Unregister a connection
    pub fn unregister_connection(&self, ip: &str) {
        *self.connection_count.write() = self.connection_count.read().saturating_sub(1);
        let mut per_ip = self.connections_per_ip.write();
        if let Some(count) = per_ip.get_mut(ip) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                per_ip.remove(ip);
            }
        }
    }

    /// Accept a connection
    async fn accept_connection(&self, stream: TcpStream, addr: SocketAddr) {
        let ip = addr.ip().to_string();

        // Check connection limits
        if *self.connection_count.read() >= self.config.max_connections {
            tracing::warn!("Max connections reached, rejecting {}", addr);
            return;
        }

        if !self.can_accept_ip(&ip) {
            tracing::warn!("Max connections per IP reached for {}", ip);
            return;
        }

        self.register_connection(&ip);

        let (connection, rx) = TcpConnection::new(stream, addr);
        let connection = Arc::new(connection);

        // Start write handler
        connection.start_writing(rx);

        // Call connection callback
        if let Some(ref callback) = *self.on_connection.read() {
            callback(connection);
        }
    }
}

#[async_trait]
impl SocketServerTrait for SocketServer {
    async fn listen(&self) -> Result<(), std::io::Error> {
        let listener = TcpListener::bind(self.config.addr).await?;
        *self.running.write() = true;

        tracing::info!("Listening on {}", self.config.addr);

        while *self.running.read() {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    self.accept_connection(stream, addr).await;
                }
                Err(e) => {
                    tracing::error!("Accept error: {}", e);
                }
            }
        }

        Ok(())
    }

    async fn stop(&self) {
        *self.running.write() = false;
    }

    fn is_running(&self) -> bool {
        *self.running.read()
    }

    fn bind_address(&self) -> SocketAddr {
        self.config.addr
    }

    fn max_connections_per_ip(&self) -> usize {
        self.config.max_connections_per_ip
    }

    fn connection_count(&self) -> usize {
        *self.connection_count.read()
    }
}
