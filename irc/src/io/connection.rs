//! Connection implementation
//!
//! Provides TCP connection handling with buffered reading and writing.
//! Messages are read line-by-line (delimited by \r\n) and parsed as IRC messages.

use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use crate::io::{DataRegulator, DataRegulatorConfig};
use crate::message::IrcMessage;
use crate::traits::{ChatMessage, Connection as ConnectionTrait, DataRegulator as DataRegulatorTrait};

/// Callback type for receiving data
pub type OnReceiveCallback = Box<dyn Fn(&str) + Send + Sync>;

/// TCP connection wrapper with buffered I/O
pub struct TcpConnection {
    /// Remote socket address
    addr: SocketAddr,
    /// Remote IP as string
    ip: String,
    /// Channel for sending outgoing data
    tx: mpsc::Sender<String>,
    /// Connection state
    connected: Arc<AtomicBool>,
    /// Data regulator for this connection
    data_regulator: Arc<DataRegulator>,
}

impl TcpConnection {
    /// Create a new TCP connection
    ///
    /// Returns the connection and a receiver for reading incoming lines
    pub fn new(stream: TcpStream, addr: SocketAddr) -> (Self, mpsc::Receiver<String>) {
        let ip = addr.ip().to_string();
        let (tx, rx) = mpsc::channel(100);
        let connected = Arc::new(AtomicBool::new(true));
        let data_regulator = Arc::new(DataRegulator::new(DataRegulatorConfig::default()));

        let conn = Self {
            addr,
            ip,
            tx,
            connected,
            data_regulator,
        };

        // Start the read/write tasks
        conn.start_io(stream, rx);

        (conn, mpsc::channel(1).1) // Return dummy receiver - actual data goes to data_regulator
    }

    /// Create a new connection with a data regulator receiver
    pub fn new_with_regulator(
        stream: TcpStream,
        addr: SocketAddr,
        config: DataRegulatorConfig,
    ) -> (Self, Arc<DataRegulator>) {
        let ip = addr.ip().to_string();
        let (tx, rx) = mpsc::channel(100);
        let connected = Arc::new(AtomicBool::new(true));
        let data_regulator = Arc::new(DataRegulator::new(config));

        let conn = Self {
            addr,
            ip,
            tx,
            connected: connected.clone(),
            data_regulator: data_regulator.clone(),
        };

        // Start the read/write tasks
        conn.start_io_internal(stream, rx, data_regulator.clone(), connected);

        (conn, data_regulator)
    }

    /// Start the I/O tasks for reading and writing
    fn start_io(&self, stream: TcpStream, write_rx: mpsc::Receiver<String>) {
        self.start_io_internal(stream, write_rx, self.data_regulator.clone(), self.connected.clone());
    }

    fn start_io_internal(
        &self,
        stream: TcpStream,
        mut write_rx: mpsc::Receiver<String>,
        data_regulator: Arc<DataRegulator>,
        connected: Arc<AtomicBool>,
    ) {
        let (read_half, write_half) = stream.into_split();

        // Spawn read task
        let read_connected = connected.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(read_half);
            let mut line = String::new();

            loop {
                if !read_connected.load(Ordering::Relaxed) {
                    break;
                }

                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        // Connection closed
                        read_connected.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(_) => {
                        // Remove trailing \r\n
                        let trimmed = line.trim_end_matches(|c| c == '\r' || c == '\n');
                        if !trimmed.is_empty() {
                            let message = IrcMessage::parse(trimmed);
                            if message.has_command() {
                                data_regulator.push_incoming_message(message);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Read error: {}", e);
                        read_connected.store(false, Ordering::Relaxed);
                        break;
                    }
                }
            }
        });

        // Spawn write task
        let write_connected = connected;
        tokio::spawn(async move {
            let mut writer = write_half;

            while let Some(data) = write_rx.recv().await {
                if !write_connected.load(Ordering::Relaxed) {
                    break;
                }

                if let Err(e) = writer.write_all(data.as_bytes()).await {
                    tracing::error!("Write error: {}", e);
                    write_connected.store(false, Ordering::Relaxed);
                    break;
                }
            }
        });
    }

    /// Get the data regulator for this connection
    pub fn data_regulator(&self) -> &Arc<DataRegulator> {
        &self.data_regulator
    }

    /// Flush outgoing messages from the data regulator
    pub async fn flush(&self) {
        while let Some(msg) = self.data_regulator.pop_outgoing() {
            let _ = self.send(&format!("{}\r\n", msg)).await;
        }
    }

    /// Start writing to the connection (legacy compatibility)
    pub fn start_writing(&self, _rx: mpsc::Receiver<String>) {
        // This is now handled in start_io
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
        let _ = self.send(&format!("{}\r\n", message)).await;

        // Close connection
        self.connected.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    fn set_on_receive(&mut self, _callback: OnReceiveCallback) {
        // Callbacks are handled through the data regulator now
    }
}
