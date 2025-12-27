//! Channel trait definitions

use crate::enums::{AccessLevel, ChannelAccessLevel, ChannelAccessResult, IrcError};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use super::{Server, User};

/// Represents an IRC channel
#[async_trait]
pub trait Channel: Send + Sync {
    /// Get the channel's unique ID
    fn id(&self) -> Uuid;

    /// Get the channel name
    fn name(&self) -> &str;

    /// Get channel creation timestamp (epoch seconds)
    fn creation(&self) -> i64;

    /// Get topic changed timestamp (epoch seconds)
    fn topic_changed(&self) -> i64;

    /// Set topic changed timestamp
    fn set_topic_changed(&mut self, timestamp: i64);

    /// Get the access list
    fn access(&self) -> Arc<dyn AccessList>;

    /// Get a channel member by user
    fn get_member(&self, user: &dyn User) -> Option<Arc<dyn ChannelMember>>;

    /// Get a channel member by nickname
    fn get_member_by_nickname(&self, nickname: &str) -> Option<Arc<dyn ChannelMember>>;

    /// Check if a user is in this channel
    fn has_user(&self, user: &dyn User) -> bool;

    /// Send a message to all channel members
    fn send(&self, message: &str);

    /// Send a message to all members except one
    fn send_except(&self, message: &str, exclude: &dyn User);

    /// Send a message to members with a minimum access level
    fn send_with_level(&self, message: &str, access_level: ChannelAccessLevel);

    /// Join a user to the channel
    fn join(&mut self, user: Arc<dyn User>, access_result: ChannelAccessResult) -> Result<Arc<dyn ChannelMember>, IrcError>;

    /// Part a user from the channel
    fn part(&mut self, user: &dyn User);

    /// Quit a user from the channel (disconnect)
    fn quit(&mut self, user: &dyn User);

    /// Kick a user from the channel
    fn kick(&mut self, source: &dyn User, target: &dyn User, reason: &str);

    /// Send a message to the channel from a user
    fn send_message(&self, user: &dyn User, message: &str);

    /// Send a notice to the channel from a user
    fn send_notice(&self, user: &dyn User, message: &str);

    /// Get all channel members
    fn get_members(&self) -> Vec<Arc<dyn ChannelMember>>;

    /// Get member count
    fn member_count(&self) -> usize;

    /// Check if a source can modify this channel
    fn can_be_modified_by(&self, source: &dyn User) -> bool;

    /// Check if a source member can modify a target member
    fn can_modify_member(
        &self,
        source: &dyn ChannelMember,
        target: &dyn ChannelMember,
        required_level: ChannelAccessLevel,
    ) -> Result<(), IrcError>;

    /// Process a channel error
    fn process_channel_error(
        &self,
        error: IrcError,
        server: &dyn Server,
        source: &dyn User,
        target_name: &str,
        data: &str,
    );

    /// Send the topic to a user
    fn send_topic(&self, user: &dyn User);

    /// Send the topic to all members
    fn send_topic_all(&self);

    /// Send the names list to a user
    fn send_names(&self, user: &dyn User);

    /// Send the on-join message to a user
    fn send_on_join_message(&self, user: &dyn User);

    /// Send the on-part message to a user
    fn send_on_part_message(&self, user: &dyn User);

    /// Check if a user is allowed to join
    fn allows(&self, user: &dyn User) -> bool;

    /// Get access result for a user
    fn get_access(&self, user: &dyn User, key: Option<&str>, is_goto: bool) -> ChannelAccessResult;

    /// Invite a member to the channel
    fn invite_member(&mut self, user: &dyn User) -> bool;

    /// Update the channel topic
    fn update_topic(&mut self, topic: &str);

    /// Get the channel topic
    fn topic(&self) -> &str;
}

/// Represents a member of a channel (user + channel-specific state)
pub trait ChannelMember: Send + Sync {
    /// Get the user
    fn get_user(&self) -> Arc<dyn User>;

    /// Get the channel access level
    fn get_level(&self) -> ChannelAccessLevel;

    /// Check if member has owner mode
    fn is_owner(&self) -> bool;

    /// Set owner mode
    fn set_owner(&mut self, value: bool);

    /// Check if member has operator mode
    fn is_operator(&self) -> bool;

    /// Set operator mode
    fn set_operator(&mut self, value: bool);

    /// Check if member has voice mode
    fn is_voice(&self) -> bool;

    /// Set voice mode
    fn set_voice(&mut self, value: bool);

    /// Check if member has any modes
    fn has_modes(&self) -> bool;

    /// Get the mode character for display
    fn mode_char(&self) -> Option<char>;
}

/// Channel properties
pub trait ChannelProps: Send + Sync {
    /// Get the channel topic
    fn topic(&self) -> &str;

    /// Set the channel topic
    fn set_topic(&mut self, topic: String);

    /// Get the on-join message
    fn onjoin(&self) -> Option<&str>;

    /// Set the on-join message
    fn set_onjoin(&mut self, message: Option<String>);

    /// Get the on-part message
    fn onpart(&self) -> Option<&str>;

    /// Set the on-part message
    fn set_onpart(&mut self, message: Option<String>);

    /// Get a property by name
    fn get(&self, name: &str) -> Option<&str>;

    /// Set a property by name
    fn set(&mut self, name: &str, value: String);

    /// Get the member key
    fn member_key(&self) -> Option<&str>;

    /// Set the member key
    fn set_member_key(&mut self, key: Option<String>);

    /// Get the host key
    fn host_key(&self) -> Option<&str>;

    /// Set the host key
    fn set_host_key(&mut self, key: Option<String>);

    /// Get the owner key
    fn owner_key(&self) -> Option<&str>;

    /// Set the owner key
    fn set_owner_key(&mut self, key: Option<String>);
}

/// Access list for channels
pub trait AccessList: Send + Sync {
    /// Get entries for a specific access level
    fn get_entries(&self, level: AccessLevel) -> Vec<AccessEntry>;

    /// Get all entries
    fn get_all_entries(&self) -> Vec<(AccessLevel, AccessEntry)>;

    /// Add an entry
    fn add(&mut self, level: AccessLevel, entry: AccessEntry) -> bool;

    /// Remove an entry
    fn remove(&mut self, level: AccessLevel, mask: &str) -> bool;

    /// Clear entries for a level
    fn clear(&mut self, level: AccessLevel);

    /// Clear all entries
    fn clear_all(&mut self);

    /// Check access for a user address
    fn check(&self, address: &str) -> AccessLevel;
}

/// An access list entry
#[derive(Debug, Clone)]
pub struct AccessEntry {
    /// The mask pattern (e.g., "*!*@*.example.com")
    pub mask: String,
    /// The reason for the entry
    pub reason: Option<String>,
    /// When the entry was created
    pub created: chrono::DateTime<chrono::Utc>,
    /// Who created the entry
    pub created_by: Option<String>,
    /// Duration in seconds (0 = permanent)
    pub duration: u64,
}

impl AccessEntry {
    /// Create a new access entry
    pub fn new(mask: String) -> Self {
        Self {
            mask,
            reason: None,
            created: chrono::Utc::now(),
            created_by: None,
            duration: 0,
        }
    }

    /// Check if the entry has expired
    pub fn is_expired(&self) -> bool {
        if self.duration == 0 {
            return false;
        }
        let elapsed = chrono::Utc::now()
            .signed_duration_since(self.created)
            .num_seconds() as u64;
        elapsed >= self.duration
    }
}
