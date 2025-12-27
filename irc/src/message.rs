//! IRC message parsing according to RFC 1459
//!
//! Message format:
//! <message>  ::= [':' <prefix> <SPACE> ] <command> <params> <crlf>
//! <prefix>   ::= <servername> | <nick> [ '!' <user> ] [ '@' <host> ]
//! <command>  ::= <letter> { <letter> } | <number> <number> <number>
//! <params>   ::= <SPACE> [ ':' <trailing> | <middle> <params> ]

use std::sync::Arc;

use crate::traits::ChatMessage as ChatMessageTrait;

/// Parsed IRC message
#[derive(Debug, Clone)]
pub struct IrcMessage {
    /// Original raw message text
    original_text: String,
    /// Message prefix (if any)
    prefix: Option<String>,
    /// Command name (uppercase)
    command: String,
    /// Command parameters
    parameters: Vec<String>,
    /// Whether the message has a valid command
    has_command: bool,
}

impl IrcMessage {
    /// Parse an IRC message from raw text
    pub fn parse(text: &str) -> Self {
        let trimmed = text.trim();

        if trimmed.is_empty() {
            return Self {
                original_text: text.to_string(),
                prefix: None,
                command: String::new(),
                parameters: Vec::new(),
                has_command: false,
            };
        }

        let mut prefix = None;
        let mut rest = trimmed;

        // Check for prefix (starts with ':')
        if rest.starts_with(':') {
            if let Some(space_idx) = rest.find(' ') {
                prefix = Some(rest[1..space_idx].to_string());
                rest = rest[space_idx..].trim_start();
            } else {
                // Only a prefix, no command
                return Self {
                    original_text: text.to_string(),
                    prefix: Some(rest[1..].to_string()),
                    command: String::new(),
                    parameters: Vec::new(),
                    has_command: false,
                };
            }
        }

        // Parse command
        let (command, params_str) = if let Some(space_idx) = rest.find(' ') {
            (rest[..space_idx].to_uppercase(), &rest[space_idx + 1..])
        } else {
            (rest.to_uppercase(), "")
        };

        // Parse parameters
        let parameters = Self::parse_parameters(params_str);

        Self {
            original_text: text.to_string(),
            prefix,
            command,
            parameters,
            has_command: true,
        }
    }

    /// Parse IRC message parameters
    /// Parameters are space-separated, except the last one which can start with ':'
    /// to include spaces
    fn parse_parameters(params_str: &str) -> Vec<String> {
        let mut params = Vec::new();
        let mut rest = params_str;

        while !rest.is_empty() {
            // Skip leading spaces
            rest = rest.trim_start();
            if rest.is_empty() {
                break;
            }

            // Check for trailing parameter (starts with ':')
            if rest.starts_with(':') {
                // Everything after ':' is the trailing parameter
                params.push(rest[1..].to_string());
                break;
            }

            // Regular parameter (until next space)
            if let Some(space_idx) = rest.find(' ') {
                let param = &rest[..space_idx];
                if !param.is_empty() {
                    params.push(param.to_string());
                }
                rest = &rest[space_idx + 1..];
            } else {
                // Last parameter
                if !rest.is_empty() {
                    params.push(rest.to_string());
                }
                break;
            }
        }

        params
    }

    /// Get the command name
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Check if the message has a valid command
    pub fn has_command(&self) -> bool {
        self.has_command && !self.command.is_empty()
    }

    /// Get the original text
    pub fn original_text(&self) -> &str {
        &self.original_text
    }

    /// Get the message prefix
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Get the parameters
    pub fn parameters(&self) -> &[String] {
        &self.parameters
    }

    /// Get the command name (alias for command())
    pub fn command_name(&self) -> &str {
        &self.command
    }
}

impl ChatMessageTrait for IrcMessage {
    fn parameters(&self) -> &[String] {
        &self.parameters
    }

    fn original_text(&self) -> &str {
        &self.original_text
    }

    fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    fn has_command(&self) -> bool {
        self.has_command && !self.command.is_empty()
    }

    fn command_name(&self) -> &str {
        &self.command
    }
}

/// Chat frame containing message context
pub struct ChatFrameImpl {
    sequence_id: u64,
    message: IrcMessage,
    server: Arc<dyn crate::traits::Server>,
    user: Arc<dyn crate::traits::User>,
}

impl ChatFrameImpl {
    /// Create a new chat frame
    pub fn new(
        sequence_id: u64,
        message: IrcMessage,
        server: Arc<dyn crate::traits::Server>,
        user: Arc<dyn crate::traits::User>,
    ) -> Self {
        Self {
            sequence_id,
            message,
            server,
            user,
        }
    }
}

impl crate::traits::ChatFrame for ChatFrameImpl {
    fn sequence_id(&self) -> u64 {
        self.sequence_id
    }

    fn set_sequence_id(&mut self, id: u64) {
        self.sequence_id = id;
    }

    fn chat_message(&self) -> &dyn ChatMessageTrait {
        &self.message
    }

    fn server(&self) -> Arc<dyn crate::traits::Server> {
        self.server.clone()
    }

    fn user(&self) -> Arc<dyn crate::traits::User> {
        self.user.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        let msg = IrcMessage::parse("NICK test");
        assert!(msg.has_command());
        assert_eq!(msg.command_name(), "NICK");
        assert_eq!(msg.parameters(), &["test"]);
    }

    #[test]
    fn test_command_with_prefix() {
        let msg = IrcMessage::parse(":nick!user@host PRIVMSG #channel :Hello World");
        assert!(msg.has_command());
        assert_eq!(msg.prefix(), Some("nick!user@host"));
        assert_eq!(msg.command_name(), "PRIVMSG");
        assert_eq!(msg.parameters(), &["#channel", "Hello World"]);
    }

    #[test]
    fn test_user_command() {
        let msg = IrcMessage::parse("USER guest 0 * :Real Name");
        assert!(msg.has_command());
        assert_eq!(msg.command_name(), "USER");
        assert_eq!(msg.parameters(), &["guest", "0", "*", "Real Name"]);
    }

    #[test]
    fn test_ping() {
        let msg = IrcMessage::parse("PING :server.example.com");
        assert!(msg.has_command());
        assert_eq!(msg.command_name(), "PING");
        assert_eq!(msg.parameters(), &["server.example.com"]);
    }

    #[test]
    fn test_join() {
        let msg = IrcMessage::parse("JOIN #channel key");
        assert!(msg.has_command());
        assert_eq!(msg.command_name(), "JOIN");
        assert_eq!(msg.parameters(), &["#channel", "key"]);
    }

    #[test]
    fn test_empty_trailing() {
        let msg = IrcMessage::parse("PRIVMSG #channel :");
        assert!(msg.has_command());
        assert_eq!(msg.command_name(), "PRIVMSG");
        assert_eq!(msg.parameters(), &["#channel", ""]);
    }

    #[test]
    fn test_case_insensitive_command() {
        let msg = IrcMessage::parse("nick TEST");
        assert!(msg.has_command());
        assert_eq!(msg.command_name(), "NICK");
    }

    #[test]
    fn test_empty_message() {
        let msg = IrcMessage::parse("");
        assert!(!msg.has_command());
    }

    #[test]
    fn test_whitespace_only() {
        let msg = IrcMessage::parse("   ");
        assert!(!msg.has_command());
    }
}
