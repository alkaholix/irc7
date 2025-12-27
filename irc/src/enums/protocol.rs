//! Protocol type enumerations

use serde::{Deserialize, Serialize};

/// IRC protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ProtocolType {
    /// Standard IRC (RFC 1459)
    Irc = 0,
    /// IRCX protocol (Microsoft extension)
    IrcX = 1,
    /// IRC3 protocol
    Irc3 = 3,
    /// IRC4 protocol
    Irc4 = 4,
    /// IRC5 protocol
    Irc5 = 5,
    /// IRC6 protocol
    Irc6 = 6,
    /// IRC7 protocol
    Irc7 = 7,
    /// IRC8 protocol
    Irc8 = 8,
}

impl Default for ProtocolType {
    fn default() -> Self {
        Self::Irc
    }
}

impl ProtocolType {
    /// Check if this protocol supports IRCX extensions
    pub fn supports_ircx(&self) -> bool {
        !matches!(self, Self::Irc)
    }

    /// Get the protocol version number
    pub fn version(&self) -> u8 {
        match self {
            Self::Irc => 0,
            Self::IrcX => 1,
            Self::Irc3 => 3,
            Self::Irc4 => 4,
            Self::Irc5 => 5,
            Self::Irc6 => 6,
            Self::Irc7 => 7,
            Self::Irc8 => 8,
        }
    }
}

/// Support package authentication sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportPackageSequence {
    /// Initial state
    Initial,
    /// Authentication in progress
    InProgress,
    /// Authentication complete
    Complete,
    /// Authentication failed
    Failed,
}

impl Default for SupportPackageSequence {
    fn default() -> Self {
        Self::Initial
    }
}
