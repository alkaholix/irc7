//! Connection trait definitions

use async_trait::async_trait;
use std::net::SocketAddr;

/// Callback type for receiving data
pub type OnReceiveCallback = Box<dyn Fn(&[u8]) + Send + Sync>;

/// A network connection
#[async_trait]
pub trait Connection: Send + Sync {
    /// Get the remote IP address
    fn get_ip(&self) -> &str;

    /// Get the socket address
    fn get_socket_addr(&self) -> SocketAddr;

    /// Send data to the connection
    async fn send(&self, data: &str) -> Result<(), std::io::Error>;

    /// Disconnect the connection
    async fn disconnect(&self, message: &str) -> Result<(), std::io::Error>;

    /// Check if the connection is connected
    fn is_connected(&self) -> bool;

    /// Set the receive callback
    fn set_on_receive(&mut self, callback: OnReceiveCallback);
}

/// A socket server for accepting connections
#[async_trait]
pub trait SocketServer: Send + Sync {
    /// Start listening for connections
    async fn listen(&self) -> Result<(), std::io::Error>;

    /// Stop the server
    async fn stop(&self);

    /// Check if the server is running
    fn is_running(&self) -> bool;

    /// Get the bind address
    fn bind_address(&self) -> SocketAddr;

    /// Get the maximum connections per IP
    fn max_connections_per_ip(&self) -> usize;

    /// Get current connection count
    fn connection_count(&self) -> usize;
}
