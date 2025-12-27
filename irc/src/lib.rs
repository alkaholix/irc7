//! IRC7 - A comprehensive IRC server library written in Rust
//!
//! This library provides a complete implementation of an IRC server with
//! support for standard IRC (RFC 1459), IRCX extensions, and multiple
//! authentication methods.
//!
//! # Features
//!
//! - Full IRC protocol support (RFC 1459)
//! - IRCX protocol extensions (Microsoft Chat)
//! - Multiple authentication packages (ANON, NTLM, GateKeeper)
//! - Channel modes and user modes
//! - Access control lists
//! - Flood protection
//! - Async I/O with Tokio
//!
//! # Example
//!
//! ```no_run
//! use irc::objects::{Server, ServerConfig};
//! use irc::security::SecurityManager;
//! use irc::io::DataStore;
//! use std::sync::Arc;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ServerConfig::default();
//!     let security = Arc::new(SecurityManager::new());
//!     let data_store = Arc::new(DataStore::new());
//!
//!     let server = Server::new(config, security, data_store, None);
//!     // Start the server...
//! }
//! ```

pub mod access;
pub mod commands;
pub mod constants;
pub mod enums;
pub mod io;
pub mod modes;
pub mod objects;
pub mod protocols;
pub mod security;
pub mod traits;

// Re-export commonly used types
pub use enums::*;
pub use objects::{Channel, Member, Server, ServerConfig, User, UserAddress};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = "irc7";
