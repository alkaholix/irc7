//! Channel modes implementation

use crate::traits::{ChannelModes as ChannelModesTrait, Mode, ModeCollection};
use parking_lot::RwLock;
use std::collections::HashMap;

/// Channel mode characters
pub mod chars {
    pub const INVITE_ONLY: char = 'i';
    pub const MODERATED: char = 'm';
    pub const NO_EXTERNAL: char = 'n';
    pub const PRIVATE: char = 'p';
    pub const SECRET: char = 's';
    pub const TOPIC_LOCK: char = 't';
    pub const KEY: char = 'k';
    pub const LIMIT: char = 'l';
    pub const BAN: char = 'b';
    pub const REGISTERED: char = 'r';
    pub const AUDITORIUM: char = 'u';
    pub const SERVICE: char = 'S';
    pub const KNOCK: char = 'K';
    pub const NO_WHISPER: char = 'w';
    pub const CLONE: char = 'c';
    pub const NO_GUEST: char = 'g';
    pub const AUTH_ONLY: char = 'a';
}

/// A single channel mode
#[derive(Debug, Clone)]
pub struct ChannelMode {
    char: char,
    name: String,
    requires_param: bool,
    param_on_set: bool,
    param_on_unset: bool,
}

impl ChannelMode {
    pub fn new(char: char, name: &str, requires_param: bool) -> Self {
        Self {
            char,
            name: name.to_string(),
            requires_param,
            param_on_set: requires_param,
            param_on_unset: false,
        }
    }

    pub fn with_param_rules(
        char: char,
        name: &str,
        param_on_set: bool,
        param_on_unset: bool,
    ) -> Self {
        Self {
            char,
            name: name.to_string(),
            requires_param: param_on_set || param_on_unset,
            param_on_set,
            param_on_unset,
        }
    }
}

impl Mode for ChannelMode {
    fn char(&self) -> char {
        self.char
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn requires_parameter(&self) -> bool {
        self.requires_param
    }

    fn takes_parameter_on_set(&self) -> bool {
        self.param_on_set
    }

    fn takes_parameter_on_unset(&self) -> bool {
        self.param_on_unset
    }

    fn validate_parameter(&self, value: &str) -> bool {
        match self.char {
            chars::LIMIT => value.parse::<usize>().is_ok(),
            chars::KEY => !value.is_empty() && !value.contains(' '),
            _ => true,
        }
    }
}

/// Internal mode state
struct ChannelModeState {
    bool_modes: HashMap<char, bool>,
    user_limit: usize,
    key: Option<String>,
}

impl Default for ChannelModeState {
    fn default() -> Self {
        Self {
            bool_modes: HashMap::new(),
            user_limit: 0,
            key: None,
        }
    }
}

/// Collection of channel modes
pub struct ChannelModesImpl {
    state: RwLock<ChannelModeState>,
}

impl Default for ChannelModesImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelModesImpl {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(ChannelModeState::default()),
        }
    }

    /// Get the channel key
    pub fn get_key(&self) -> Option<String> {
        self.state.read().key.clone()
    }

    /// Set the channel key
    pub fn set_key(&mut self, key: Option<String>) {
        self.state.write().key = key;
    }
}

impl ModeCollection for ChannelModesImpl {
    fn get_mode(&self, _char: char) -> Option<&dyn Mode> {
        None
    }

    fn has_mode(&self, char: char) -> bool {
        self.state.read().bool_modes.get(&char).copied().unwrap_or(false)
    }

    fn get_mode_value(&self, char: char) -> Option<bool> {
        self.state.read().bool_modes.get(&char).copied()
    }

    fn set_mode_value(&mut self, char: char, value: bool) {
        self.state.write().bool_modes.insert(char, value);
    }

    fn to_mode_string(&self) -> String {
        let state = self.state.read();
        let mut chars: Vec<char> = state
            .bool_modes
            .iter()
            .filter(|(_, &v)| v)
            .map(|(&c, _)| c)
            .collect();

        // Add special modes
        if state.user_limit > 0 {
            chars.push(chars::LIMIT);
        }
        if state.key.is_some() {
            chars.push(chars::KEY);
        }

        chars.sort();

        if chars.is_empty() {
            return String::new();
        }

        let mut result = format!("+{}", chars.iter().collect::<String>());

        // Add parameters
        if state.user_limit > 0 {
            result.push_str(&format!(" {}", state.user_limit));
        }
        if let Some(ref key) = state.key {
            result.push_str(&format!(" {}", key));
        }

        result
    }

    fn all_chars(&self) -> Vec<char> {
        vec![
            chars::INVITE_ONLY,
            chars::MODERATED,
            chars::NO_EXTERNAL,
            chars::PRIVATE,
            chars::SECRET,
            chars::TOPIC_LOCK,
            chars::KEY,
            chars::LIMIT,
            chars::BAN,
            chars::REGISTERED,
            chars::AUDITORIUM,
            chars::SERVICE,
            chars::KNOCK,
            chars::NO_WHISPER,
            chars::CLONE,
            chars::NO_GUEST,
            chars::AUTH_ONLY,
        ]
    }
}

impl ChannelModesTrait for ChannelModesImpl {
    fn is_invite_only(&self) -> bool {
        self.has_mode(chars::INVITE_ONLY)
    }

    fn set_invite_only(&mut self, value: bool) {
        self.set_mode_value(chars::INVITE_ONLY, value);
    }

    fn is_moderated(&self) -> bool {
        self.has_mode(chars::MODERATED)
    }

    fn set_moderated(&mut self, value: bool) {
        self.set_mode_value(chars::MODERATED, value);
    }

    fn is_no_external(&self) -> bool {
        self.has_mode(chars::NO_EXTERNAL)
    }

    fn set_no_external(&mut self, value: bool) {
        self.set_mode_value(chars::NO_EXTERNAL, value);
    }

    fn is_private(&self) -> bool {
        self.has_mode(chars::PRIVATE)
    }

    fn set_private(&mut self, value: bool) {
        self.set_mode_value(chars::PRIVATE, value);
    }

    fn is_secret(&self) -> bool {
        self.has_mode(chars::SECRET)
    }

    fn set_secret(&mut self, value: bool) {
        self.set_mode_value(chars::SECRET, value);
    }

    fn is_topic_lock(&self) -> bool {
        self.has_mode(chars::TOPIC_LOCK)
    }

    fn set_topic_lock(&mut self, value: bool) {
        self.set_mode_value(chars::TOPIC_LOCK, value);
    }

    fn has_key(&self) -> bool {
        self.state.read().key.is_some()
    }

    fn user_limit(&self) -> usize {
        self.state.read().user_limit
    }

    fn set_user_limit(&mut self, limit: usize) {
        self.state.write().user_limit = limit;
    }
}
