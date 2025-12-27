//! Server trait definitions

use crate::enums::{ProtocolType, UserAccessLevel};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

use super::{Channel, Connection, CredentialProvider, DataStore, Protocol, SecurityManager, User};

/// Represents an IRC server
#[async_trait]
pub trait Server: Send + Sync {
    /// Get the server's unique ID
    fn id(&self) -> Uuid;

    /// Get the short ID
    fn short_id(&self) -> String;

    /// Get the server name
    fn name(&self) -> &str;

    /// Set the server name
    fn set_name(&mut self, name: String);

    /// Get the server title
    fn title(&self) -> &str;

    /// Get the server creation date
    fn creation_date(&self) -> chrono::DateTime<chrono::Utc>;

    /// Check if anonymous connections are allowed
    fn anonymous_allowed(&self) -> bool;

    /// Get the channel count
    fn channel_count(&self) -> usize;

    /// Get server info lines
    fn info(&self) -> &[String];

    /// Get maximum message length
    fn max_message_length(&self) -> usize;

    /// Get maximum input bytes
    fn max_input_bytes(&self) -> usize;

    /// Get maximum output bytes
    fn max_output_bytes(&self) -> usize;

    /// Get ping interval in seconds
    fn ping_interval(&self) -> u64;

    /// Get ping attempts before disconnect
    fn ping_attempts(&self) -> u32;

    /// Get maximum channels per user
    fn max_channels(&self) -> usize;

    /// Get maximum connections
    fn max_connections(&self) -> usize;

    /// Get maximum authenticated connections
    fn max_authenticated_connections(&self) -> usize;

    /// Get maximum anonymous connections
    fn max_anonymous_connections(&self) -> usize;

    /// Get maximum guest connections
    fn max_guest_connections(&self) -> usize;

    /// Check if basic authentication is enabled
    fn basic_authentication(&self) -> bool;

    /// Check if anonymous connections are enabled
    fn anonymous_connections(&self) -> bool;

    /// Get count of invisible users on the network
    fn net_invisible_count(&self) -> usize;

    /// Get server count on the network
    fn net_server_count(&self) -> usize;

    /// Get user count on the network
    fn net_user_count(&self) -> usize;

    /// Get security packages string
    fn security_packages(&self) -> &str;

    /// Get sysop count
    fn sysop_count(&self) -> usize;

    /// Get unknown connection count
    fn unknown_connection_count(&self) -> usize;

    /// Get remote IP
    fn remote_ip(&self) -> &str;

    /// Set remote IP
    fn set_remote_ip(&mut self, ip: String);

    /// Check if guest mode is disabled
    fn guest_mode_disabled(&self) -> bool;

    /// Set guest mode disabled
    fn set_guest_mode_disabled(&mut self, disabled: bool);

    /// Check if user registration is disabled
    fn user_registration_disabled(&self) -> bool;

    /// Set user registration disabled
    fn set_user_registration_disabled(&mut self, disabled: bool);

    /// Check if this is a directory server
    fn is_directory_server(&self) -> bool;

    /// Get server version
    fn server_version(&self) -> &str;

    /// Set server version
    fn set_server_version(&mut self, version: String);

    /// Add a user to the server
    fn add_user(&mut self, user: Arc<dyn User>);

    /// Remove a user from the server
    fn remove_user(&mut self, user: &dyn User);

    /// Add a channel to the server
    fn add_channel(&mut self, channel: Arc<dyn Channel>);

    /// Remove a channel from the server
    fn remove_channel(&mut self, channel: &dyn Channel);

    /// Create a new channel
    fn create_channel(&self, name: &str) -> Arc<dyn Channel>;

    /// Create a new channel with creator and key
    fn create_channel_with_key(&self, creator: &dyn User, name: &str, key: Option<&str>) -> Arc<dyn Channel>;

    /// Create a new user for a connection
    fn create_user(&self, connection: Arc<dyn Connection>) -> Arc<dyn User>;

    /// Get all users
    fn get_users(&self) -> Vec<Arc<dyn User>>;

    /// Get a user by nickname
    fn get_user_by_nickname(&self, nickname: &str) -> Option<Arc<dyn User>>;

    /// Get a user by nickname relative to current user
    fn get_user_by_nickname_relative(&self, nickname: &str, current_user: &dyn User) -> Option<Arc<dyn User>>;

    /// Get users by a list of nicknames
    fn get_users_by_list(&self, nicknames: &[String]) -> Vec<Arc<dyn User>>;

    /// Get all channels
    fn get_channels(&self) -> Vec<Arc<dyn Channel>>;

    /// Get supported channel modes
    fn get_supported_channel_modes(&self) -> &str;

    /// Get supported user modes
    fn get_supported_user_modes(&self) -> &str;

    /// Get available protocols
    fn get_protocols(&self) -> Vec<(ProtocolType, Arc<dyn Protocol>)>;

    /// Get the data store
    fn get_data_store(&self) -> Arc<dyn DataStore>;

    /// Get a channel by name
    fn get_channel_by_name(&self, name: &str) -> Option<Arc<dyn Channel>>;

    /// Get a protocol by type
    fn get_protocol(&self, protocol_type: ProtocolType) -> Option<Arc<dyn Protocol>>;

    /// Get the security manager
    fn get_security_manager(&self) -> Arc<dyn SecurityManager>;

    /// Get the credential provider
    fn get_credential_provider(&self) -> Option<Arc<dyn CredentialProvider>>;

    /// Shutdown the server
    fn shutdown(&mut self);

    /// Get the MOTD
    fn get_motd(&self) -> &[String];

    /// Set the MOTD
    fn set_motd(&mut self, motd: Vec<String>);

    /// Process a cookie
    fn process_cookie(&self, user: &dyn User, name: &str, value: &str);
}
