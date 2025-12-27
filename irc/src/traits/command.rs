//! Command trait definitions

use crate::enums::CommandDataType;
use async_trait::async_trait;
use std::sync::Arc;

use super::{Server, User};

/// A chat frame containing message context
pub trait ChatFrame: Send + Sync {
    /// Get the sequence ID
    fn sequence_id(&self) -> u64;

    /// Set the sequence ID
    fn set_sequence_id(&mut self, id: u64);

    /// Get the chat message
    fn chat_message(&self) -> &dyn ChatMessage;

    /// Get the server
    fn server(&self) -> Arc<dyn Server>;

    /// Get the user
    fn user(&self) -> Arc<dyn User>;
}

/// A parsed chat message
pub trait ChatMessage: Send + Sync {
    /// Get message parameters
    fn parameters(&self) -> &[String];

    /// Get the original text
    fn original_text(&self) -> &str;

    /// Get the prefix (if any)
    fn prefix(&self) -> Option<&str>;

    /// Check if the message has a command
    fn has_command(&self) -> bool;

    /// Get the command name
    fn command_name(&self) -> &str;
}

/// An IRC command handler
#[async_trait]
pub trait Command: Send + Sync {
    /// Get the data type for this command
    fn data_type(&self) -> CommandDataType;

    /// Get the command name
    fn name(&self) -> &str;

    /// Execute the command
    async fn execute(&self, frame: &dyn ChatFrame);

    /// Check if parameters are valid
    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool;

    /// Check if registration is needed for this command
    fn registration_needed(&self, frame: &dyn ChatFrame) -> bool;

    /// Get minimum required parameters
    fn min_parameters(&self) -> usize {
        0
    }

    /// Get maximum allowed parameters (None = unlimited)
    fn max_parameters(&self) -> Option<usize> {
        None
    }
}
