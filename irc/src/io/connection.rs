//! Connection implementation

use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, Mutex};

use crate::traits::{Connection as ConnectionTrait, OnReceiveCallback};

/// TCP connection wrapper
pub struct TcpConnection {
    addr: SocketAddr,
    ip: String,
    stream: Arc<Mutex<Option<TcpStream>>>,
    connected: Arc<AtomicBool>,
    tx: mpsc::Sender<String>,
}

impl TcpConnection {
    /// Create a new TCP connection
    pub fn new(stream: TcpStream, addr: SocketAddr) -> (Self, mpsc::Receiver<String>) {
        let ip = addr.ip().to_string();
        let (tx, rx) = mpsc::channel(100);

        let conn = Self {
            addr,
            ip,
            stream: Arc::new(Mutex::new(Some(stream))),
            connected: Arc::new(AtomicBool::new(true)),
            tx,
        };

        (conn, rx)
    }

    /// Start reading from the connection
    pub fn start_reading<F>(&self, _on_receive: F)
    where
        F: FnMut(&[u8]) + Send + 'static,
    {
        let connected = self.connected.clone();

        tokio::spawn(async move {
            loop {
                if !connected.load(Ordering::Relaxed) {
                    break;
                }

                // Read would happen here in actual implementation
                // For now, just sleep to prevent busy loop
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });
    }

    /// Start writing to the connection
    pub fn start_writing(&self, mut rx: mpsc::Receiver<String>) {
        let stream = self.stream.clone();
        let connected = self.connected.clone();

        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                if !connected.load(Ordering::Relaxed) {
                    break;
                }

                let mut stream_guard = stream.lock().await;
                if let Some(ref mut s) = *stream_guard {
                    if s.write_all(data.as_bytes()).await.is_err() {
                        break;
                    }
                }
            }
        });
    }
}

#[async_trait]
impl ConnectionTrait for TcpConnection {
    fn get_ip(&self) -> &str {
        &self.ip
    }

    fn get_socket_addr(&self) -> SocketAddr {
        self.addr
    }

    async fn send(&self, data: &str) -> Result<(), std::io::Error> {
        self.tx
            .send(data.to_string())
            .await
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Channel closed"))
    }

    async fn disconnect(&self, message: &str) -> Result<(), std::io::Error> {
        // Send final message
        let _ = self.send(message).await;

        // Close connection
        self.connected.store(false, Ordering::Relaxed);

        let mut stream = self.stream.lock().await;
        if let Some(s) = stream.take() {
            drop(s);
        }

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    fn set_on_receive(&mut self, _callback: OnReceiveCallback) {
        // Callback handling
    }
}
