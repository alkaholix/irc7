//! User modes implementation

use crate::traits::{Mode, ModeCollection, UserModes as UserModesTrait};
use parking_lot::RwLock;
use std::collections::HashMap;

/// User mode characters
pub mod chars {
    pub const INVISIBLE: char = 'i';
    pub const OPER: char = 'o';
    pub const ADMIN: char = 'a';
    pub const WALLOPS: char = 'w';
    pub const SECURE: char = 'x';
    pub const HOST: char = 'h';
    pub const GAG: char = 'z';
}

/// A single user mode
#[derive(Debug, Clone)]
pub struct UserMode {
    char: char,
    name: String,
    value: bool,
}

impl UserMode {
    pub fn new(char: char, name: &str) -> Self {
        Self {
            char,
            name: name.to_string(),
            value: false,
        }
    }
}

impl Mode for UserMode {
    fn char(&self) -> char {
        self.char
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn requires_parameter(&self) -> bool {
        false
    }

    fn takes_parameter_on_set(&self) -> bool {
        false
    }

    fn takes_parameter_on_unset(&self) -> bool {
        false
    }

    fn validate_parameter(&self, _value: &str) -> bool {
        true
    }
}

/// Collection of user modes
pub struct UserModesImpl {
    modes: RwLock<HashMap<char, bool>>,
}

impl Default for UserModesImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl UserModesImpl {
    pub fn new() -> Self {
        Self {
            modes: RwLock::new(HashMap::new()),
        }
    }
}

impl ModeCollection for UserModesImpl {
    fn get_mode(&self, char: char) -> Option<&dyn Mode> {
        // We don't store Mode objects, just values
        None
    }

    fn has_mode(&self, char: char) -> bool {
        self.modes.read().get(&char).copied().unwrap_or(false)
    }

    fn get_mode_value(&self, char: char) -> Option<bool> {
        self.modes.read().get(&char).copied()
    }

    fn set_mode_value(&mut self, char: char, value: bool) {
        self.modes.write().insert(char, value);
    }

    fn to_mode_string(&self) -> String {
        let modes = self.modes.read();
        let mut chars: Vec<char> = modes
            .iter()
            .filter(|(_, &v)| v)
            .map(|(&c, _)| c)
            .collect();
        chars.sort();
        if chars.is_empty() {
            String::new()
        } else {
            format!("+{}", chars.iter().collect::<String>())
        }
    }

    fn all_chars(&self) -> Vec<char> {
        vec![
            chars::INVISIBLE,
            chars::OPER,
            chars::ADMIN,
            chars::WALLOPS,
            chars::SECURE,
            chars::HOST,
            chars::GAG,
        ]
    }
}

impl UserModesTrait for UserModesImpl {
    fn is_invisible(&self) -> bool {
        self.has_mode(chars::INVISIBLE)
    }

    fn set_invisible(&mut self, value: bool) {
        self.set_mode_value(chars::INVISIBLE, value);
    }

    fn is_oper(&self) -> bool {
        self.has_mode(chars::OPER)
    }

    fn set_oper(&mut self, value: bool) {
        self.set_mode_value(chars::OPER, value);
    }

    fn is_admin(&self) -> bool {
        self.has_mode(chars::ADMIN)
    }

    fn set_admin(&mut self, value: bool) {
        self.set_mode_value(chars::ADMIN, value);
    }
}
