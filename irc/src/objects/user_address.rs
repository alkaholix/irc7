//! User address representation

use serde::{Deserialize, Serialize};

/// Represents a user's address (nick!user@host)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserAddress {
    /// The user's nickname
    pub nickname: String,
    /// The username/ident
    pub user: String,
    /// The hostname
    pub host: String,
    /// The real IP address
    pub ip: String,
    /// The masked IP address
    pub masked_ip: String,
    /// The server name
    pub server: String,
}

impl UserAddress {
    /// Create a new empty user address
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the IP address and generate masked version
    pub fn set_ip(&mut self, ip: &str) {
        self.ip = ip.to_string();
        self.masked_ip = Self::mask_ip(ip);
    }

    /// Set the nickname
    pub fn set_nickname(&mut self, nickname: &str) {
        self.nickname = nickname.to_string();
    }

    /// Mask an IP address
    fn mask_ip(ip: &str) -> String {
        // Simple masking: replace last octet with x for IPv4
        if let Some(last_dot) = ip.rfind('.') {
            format!("{}.x", &ip[..last_dot])
        } else {
            ip.to_string()
        }
    }

    /// Get the user@host string
    pub fn user_host(&self) -> String {
        format!("{}@{}", self.user, self.host)
    }

    /// Get the nick!user@host string
    pub fn full_address(&self) -> String {
        format!("{}!{}@{}", self.nickname, self.user, self.host)
    }

    /// Get just the address part (user@host)
    pub fn address(&self) -> String {
        format!("{}@{}", self.user, self.host)
    }
}

impl std::fmt::Display for UserAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}!{}@{}", self.nickname, self.user, self.host)
    }
}
