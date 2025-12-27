//! Protocol trait definitions

use crate::enums::ProtocolType;
use async_trait::async_trait;
use std::sync::Arc;

use super::{ChannelMember, Command, User};

/// An IRC protocol implementation
#[async_trait]
pub trait Protocol: Send + Sync {
    /// Get the protocol type
    fn protocol_type(&self) -> ProtocolType;

    /// Get a command by name
    fn get_command(&self, name: &str) -> Option<Arc<dyn Command>>;

    /// Get all commands
    fn get_commands(&self) -> Vec<(String, Arc<dyn Command>)>;

    /// Add a command
    fn add_command(&mut self, command: Arc<dyn Command>);

    /// Add a command with a custom name
    fn add_command_with_name(&mut self, name: String, command: Arc<dyn Command>);

    /// Update a command
    fn update_command(&mut self, command: Arc<dyn Command>);

    /// Update a command with a custom name
    fn update_command_with_name(&mut self, name: String, command: Arc<dyn Command>);

    /// Flush all commands
    fn flush_commands(&mut self);

    /// Format a user for display in this protocol
    fn formatted_user(&self, member: &dyn ChannelMember) -> String;

    /// Get the format string for a user
    fn get_format(&self, user: &dyn User) -> String;
}
