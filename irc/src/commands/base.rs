//! Base command infrastructure

use async_trait::async_trait;
use std::sync::Arc;

use crate::enums::CommandDataType;
use crate::traits::{ChatFrame, ChatMessage, Command, Server, User};

/// A concrete chat frame implementation
pub struct ChatFrameImpl {
    sequence_id: u64,
    message: Box<dyn ChatMessage>,
    server: Arc<dyn Server>,
    user: Arc<dyn User>,
}

impl ChatFrameImpl {
    pub fn new(
        sequence_id: u64,
        message: Box<dyn ChatMessage>,
        server: Arc<dyn Server>,
        user: Arc<dyn User>,
    ) -> Self {
        Self {
            sequence_id,
            message,
            server,
            user,
        }
    }
}

impl ChatFrame for ChatFrameImpl {
    fn sequence_id(&self) -> u64 {
        self.sequence_id
    }

    fn set_sequence_id(&mut self, id: u64) {
        self.sequence_id = id;
    }

    fn chat_message(&self) -> &dyn ChatMessage {
        self.message.as_ref()
    }

    fn server(&self) -> Arc<dyn Server> {
        self.server.clone()
    }

    fn user(&self) -> Arc<dyn User> {
        self.user.clone()
    }
}

/// A parsed chat message implementation
pub struct ChatMessageImpl {
    original: String,
    prefix: Option<String>,
    command: String,
    parameters: Vec<String>,
}

impl ChatMessageImpl {
    /// Parse an IRC message
    pub fn parse(input: &str) -> Self {
        let input = input.trim();
        let mut remaining = input;
        let mut prefix = None;

        // Parse prefix
        if remaining.starts_with(':') {
            if let Some(space_idx) = remaining.find(' ') {
                prefix = Some(remaining[1..space_idx].to_string());
                remaining = &remaining[space_idx + 1..];
            }
        }

        // Parse command and parameters
        let mut parts: Vec<&str> = Vec::new();
        let mut trailing = None;

        if let Some(colon_idx) = remaining.find(" :") {
            trailing = Some(&remaining[colon_idx + 2..]);
            remaining = &remaining[..colon_idx];
        }

        parts.extend(remaining.split_whitespace());

        let command = parts.first().map(|s| s.to_uppercase()).unwrap_or_default();
        let mut parameters: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        if let Some(trail) = trailing {
            parameters.push(trail.to_string());
        }

        Self {
            original: input.to_string(),
            prefix,
            command,
            parameters,
        }
    }
}

impl ChatMessage for ChatMessageImpl {
    fn parameters(&self) -> &[String] {
        &self.parameters
    }

    fn original_text(&self) -> &str {
        &self.original
    }

    fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    fn has_command(&self) -> bool {
        !self.command.is_empty()
    }

    fn command_name(&self) -> &str {
        &self.command
    }
}

/// Base command with common functionality
pub struct BaseCommand {
    name: String,
    data_type: CommandDataType,
    min_params: usize,
    requires_registration: bool,
}

impl BaseCommand {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_uppercase(),
            data_type: CommandDataType::Standard,
            min_params: 0,
            requires_registration: true,
        }
    }

    pub fn with_min_params(mut self, min: usize) -> Self {
        self.min_params = min;
        self
    }

    pub fn registration_required(mut self, required: bool) -> Self {
        self.requires_registration = required;
        self
    }

    pub fn with_data_type(mut self, data_type: CommandDataType) -> Self {
        self.data_type = data_type;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let msg = ChatMessageImpl::parse("NICK test");
        assert_eq!(msg.command_name(), "NICK");
        assert_eq!(msg.parameters(), &["test"]);
    }

    #[test]
    fn test_parse_with_prefix() {
        let msg = ChatMessageImpl::parse(":nick!user@host PRIVMSG #channel :Hello world");
        assert_eq!(msg.prefix(), Some("nick!user@host"));
        assert_eq!(msg.command_name(), "PRIVMSG");
        assert_eq!(msg.parameters(), &["#channel", "Hello world"]);
    }

    #[test]
    fn test_parse_no_trailing() {
        let msg = ChatMessageImpl::parse("MODE #channel +o nick");
        assert_eq!(msg.command_name(), "MODE");
        assert_eq!(msg.parameters(), &["#channel", "+o", "nick"]);
    }
}
