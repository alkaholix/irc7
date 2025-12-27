//! Channel-related enumerations

use serde::{Deserialize, Serialize};

/// Channel access level for members
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(i8)]
pub enum ChannelAccessLevel {
    None = 0,
    ChatMember = 1,
    ChatVoice = 2,
    ChatHost = 3,
    ChatOwner = 4,
}

impl Default for ChannelAccessLevel {
    fn default() -> Self {
        Self::None
    }
}

/// Result of channel access check
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i16)]
pub enum ChannelAccessResult {
    /// Default/no result
    None = 0,
    /// Successfully joined as guest
    SuccessGuest = 1,
    /// Successfully joined as member
    SuccessMember = 2,
    /// Successfully joined as voice
    SuccessVoice = 3,
    /// Successfully joined as host/operator
    SuccessHost = 4,
    /// Successfully joined as owner
    SuccessOwner = 5,
    /// Error: Channel is full
    ErrChannelIsFull = -1,
    /// Error: Invite only channel
    ErrInviteOnlyChan = -2,
    /// Error: Bad channel key
    ErrBadChannelKey = -3,
    /// Error: Banned from channel
    ErrBannedFromChan = -4,
    /// Error: No such channel
    ErrNoSuchChannel = -5,
}

impl Default for ChannelAccessResult {
    fn default() -> Self {
        Self::None
    }
}

impl ChannelAccessResult {
    /// Check if the result indicates success
    pub fn is_success(&self) -> bool {
        matches!(
            self,
            Self::SuccessGuest
                | Self::SuccessMember
                | Self::SuccessVoice
                | Self::SuccessHost
                | Self::SuccessOwner
        )
    }

    /// Check if the result indicates an error
    pub fn is_error(&self) -> bool {
        matches!(
            self,
            Self::ErrChannelIsFull
                | Self::ErrInviteOnlyChan
                | Self::ErrBadChannelKey
                | Self::ErrBannedFromChan
                | Self::ErrNoSuchChannel
        )
    }
}

/// Join reply type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinReply {
    /// Standard join
    Join,
    /// Create new channel
    Create,
    /// Error occurred
    Error,
}
