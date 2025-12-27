//! User trait definitions

use crate::enums::{ChannelAccessLevel, UserAccessLevel};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use super::{Channel, ChannelMember, ChatFrame, Connection, DataRegulator, FloodProtectionProfile, Protocol, Server, SupportPackage};
use crate::objects::UserAddress;
use crate::modes::ModeOperation;

/// Represents a connected IRC user
#[async_trait]
pub trait User: Send + Sync {
    /// Get the server this user is connected to
    fn server(&self) -> Arc<dyn Server>;

    /// Get the unique identifier for this user
    fn id(&self) -> Uuid;

    /// Get the short ID (first 8 chars of UUID)
    fn short_id(&self) -> String;

    /// Get the user's name
    fn name(&self) -> &str;

    /// Set the user's name
    fn set_name(&mut self, name: String);

    /// Get the user's nickname
    fn nickname(&self) -> &str;

    /// Set the user's nickname
    fn set_nickname(&mut self, nickname: String);

    /// Get the client identifier
    fn client(&self) -> &str;

    /// Set the client identifier
    fn set_client(&mut self, client: String);

    /// Get the password (for authentication)
    fn pass(&self) -> &str;

    /// Set the password
    fn set_pass(&mut self, pass: String);

    /// Check if user is away
    fn is_away(&self) -> bool;

    /// Set away status
    fn set_away(&mut self, away: bool);

    /// Get last idle time
    fn last_idle(&self) -> chrono::DateTime<chrono::Utc>;

    /// Set last idle time
    fn set_last_idle(&mut self, time: chrono::DateTime<chrono::Utc>);

    /// Get login time
    fn logged_on(&self) -> chrono::DateTime<chrono::Utc>;

    /// Check if UTF-8 mode is enabled
    fn is_utf8(&self) -> bool;

    /// Set UTF-8 mode
    fn set_utf8(&mut self, utf8: bool);

    /// Get the next chat frame from the incoming queue
    fn get_next_frame(&mut self) -> Option<Box<dyn ChatFrame>>;

    /// Change nickname with optional UTF-8 prefix
    fn change_nickname(&mut self, new_nick: &str, utf8_prefix: bool);

    /// Set guest status
    fn set_guest(&mut self, guest: bool);

    /// Set away status with message
    fn set_away_with_message(&mut self, message: &str);

    /// Set user as back (not away)
    fn set_back(&mut self);

    /// Set user access level
    fn set_level(&mut self, level: UserAccessLevel);

    /// Get user access level
    fn get_level(&self) -> UserAccessLevel;

    /// Broadcast message to all channels user is in
    fn broadcast_to_channels(&self, data: &str, exclude_self: bool);

    /// Add user to a channel
    fn add_channel(&mut self, channel: Arc<dyn Channel>, member: Arc<dyn ChannelMember>);

    /// Remove user from a channel
    fn remove_channel(&mut self, channel: &dyn Channel);

    /// Get channel member info
    fn get_channel_member_info(&self, channel: &dyn Channel) -> Option<Arc<dyn ChannelMember>>;

    /// Get channel info by name
    fn get_channel_info(&self, name: &str) -> Option<(Arc<dyn Channel>, Arc<dyn ChannelMember>)>;

    /// Get all channels the user is in
    fn get_channels(&self) -> HashMap<Uuid, (Arc<dyn Channel>, Arc<dyn ChannelMember>)>;

    /// Send a message to the user
    fn send(&self, message: &str);

    /// Send a message with access level requirement
    fn send_with_level(&self, message: &str, access_level: ChannelAccessLevel);

    /// Flush outgoing messages
    fn flush(&mut self);

    /// Disconnect the user with a message
    fn disconnect(&mut self, message: &str);

    /// Get the data regulator
    fn get_data_regulator(&self) -> Arc<dyn DataRegulator>;

    /// Get the flood protection profile
    fn get_flood_protection_profile(&self) -> Arc<dyn FloodProtectionProfile>;

    /// Get the support package (authentication)
    fn get_support_package(&self) -> Arc<dyn SupportPackage>;

    /// Set the support package
    fn set_support_package(&mut self, package: Arc<dyn SupportPackage>);

    /// Set the protocol
    fn set_protocol(&mut self, protocol: Arc<dyn Protocol>);

    /// Get the protocol
    fn get_protocol(&self) -> Arc<dyn Protocol>;

    /// Get the connection
    fn get_connection(&self) -> Arc<dyn Connection>;

    /// Get the user address
    fn get_address(&self) -> &UserAddress;

    /// Check if user is a guest
    fn is_guest(&self) -> bool;

    /// Check if user is registered
    fn is_registered(&self) -> bool;

    /// Check if user is authenticated
    fn is_authenticated(&self) -> bool;

    /// Check if user is anonymous
    fn is_anon(&self) -> bool;

    /// Check if user is a sysop
    fn is_sysop(&self) -> bool;

    /// Check if user is an administrator
    fn is_administrator(&self) -> bool;

    /// Check if user is on a channel
    fn is_on(&self, channel: &dyn Channel) -> bool;

    /// Promote user to administrator
    fn promote_to_administrator(&mut self);

    /// Promote user to sysop
    fn promote_to_sysop(&mut self);

    /// Promote user to guide
    fn promote_to_guide(&mut self);

    /// Check and disconnect if outgoing threshold exceeded
    fn disconnect_if_outgoing_threshold_exceeded(&mut self) -> bool;

    /// Check and disconnect if incoming threshold exceeded
    fn disconnect_if_incoming_threshold_exceeded(&mut self) -> bool;

    /// Register the user
    fn register(&mut self);

    /// Authenticate the user
    fn authenticate(&mut self);

    /// Check and disconnect if inactive
    fn disconnect_if_inactive(&mut self);

    /// Get pending mode operations
    fn get_mode_operations(&self) -> &[ModeOperation];

    /// Add a mode operation
    fn add_mode_operation(&mut self, op: ModeOperation);

    /// Clear mode operations
    fn clear_mode_operations(&mut self);
}

/// User properties
pub trait UserProps: Send + Sync {
    /// Get a property value
    fn get(&self, key: &str) -> Option<&str>;

    /// Set a property value
    fn set(&mut self, key: &str, value: String);

    /// Get the OID (object ID)
    fn oid(&self) -> Option<&str>;

    /// Set the OID
    fn set_oid(&mut self, oid: String);

    /// Get the role
    fn role(&self) -> Option<&str>;

    /// Set the role
    fn set_role(&mut self, role: String);
}
