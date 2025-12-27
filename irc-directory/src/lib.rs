//! IRC7 Directory Server
//!
//! This crate provides the directory server implementation for IRC7.
//! The directory server allows clients to discover available chat rooms.

use std::sync::Arc;

use irc::objects::{Server, ServerConfig};
use irc::security::SecurityManager;
use irc::io::DataStore;
use irc::traits::Server as ServerTrait;

/// Directory server wrapper
pub struct DirectoryServer {
    server: Server,
    chat_server_ip: Option<String>,
}

impl DirectoryServer {
    /// Create a new directory server
    pub fn new(
        config: ServerConfig,
        security_manager: Arc<SecurityManager>,
        data_store: Arc<DataStore>,
        chat_server_ip: Option<String>,
    ) -> Self {
        let server = Server::new_directory_server(
            config,
            security_manager,
            data_store,
            None,
        );

        Self {
            server,
            chat_server_ip,
        }
    }

    /// Get the chat server IP
    pub fn chat_server_ip(&self) -> Option<&str> {
        self.chat_server_ip.as_deref()
    }

    /// Get a reference to the underlying server
    pub fn server(&self) -> &Server {
        &self.server
    }

    /// Get a mutable reference to the underlying server
    pub fn server_mut(&mut self) -> &mut Server {
        &mut self.server
    }
}
