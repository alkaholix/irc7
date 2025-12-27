//! Channel member implementation

use std::sync::Arc;
use parking_lot::RwLock;

use crate::enums::ChannelAccessLevel;
use crate::traits::{ChannelMember as ChannelMemberTrait, MemberModes, User};

/// Mode flags for a channel member
#[derive(Debug, Clone, Default)]
pub struct MemberModeFlags {
    /// Owner mode (.)
    pub owner: bool,
    /// Operator mode (@)
    pub operator: bool,
    /// Voice mode (+)
    pub voice: bool,
}

impl MemberModes for MemberModeFlags {
    fn is_owner(&self) -> bool {
        self.owner
    }

    fn set_owner(&mut self, value: bool) {
        self.owner = value;
    }

    fn is_operator(&self) -> bool {
        self.operator
    }

    fn set_operator(&mut self, value: bool) {
        self.operator = value;
    }

    fn is_voice(&self) -> bool {
        self.voice
    }

    fn set_voice(&mut self, value: bool) {
        self.voice = value;
    }

    fn has_any(&self) -> bool {
        self.owner || self.operator || self.voice
    }

    fn prefix_char(&self) -> Option<char> {
        if self.owner {
            Some('.')
        } else if self.operator {
            Some('@')
        } else if self.voice {
            Some('+')
        } else {
            None
        }
    }
}

/// A member of a channel (user + channel-specific state)
pub struct Member {
    /// The user
    user: Arc<dyn User>,
    /// Mode flags
    modes: RwLock<MemberModeFlags>,
}

impl Member {
    /// Create a new member
    pub fn new(user: Arc<dyn User>) -> Self {
        Self {
            user,
            modes: RwLock::new(MemberModeFlags::default()),
        }
    }

    /// Create a new member with owner status
    pub fn new_owner(user: Arc<dyn User>) -> Self {
        let member = Self::new(user);
        member.modes.write().owner = true;
        member
    }

    /// Create a new member with operator status
    pub fn new_operator(user: Arc<dyn User>) -> Self {
        let member = Self::new(user);
        member.modes.write().operator = true;
        member
    }

    /// Create a new member with voice status
    pub fn new_voice(user: Arc<dyn User>) -> Self {
        let member = Self::new(user);
        member.modes.write().voice = true;
        member
    }
}

impl ChannelMemberTrait for Member {
    fn get_user(&self) -> Arc<dyn User> {
        self.user.clone()
    }

    fn get_level(&self) -> ChannelAccessLevel {
        let modes = self.modes.read();
        if modes.owner {
            ChannelAccessLevel::ChatOwner
        } else if modes.operator {
            ChannelAccessLevel::ChatHost
        } else if modes.voice {
            ChannelAccessLevel::ChatVoice
        } else {
            ChannelAccessLevel::ChatMember
        }
    }

    fn is_owner(&self) -> bool {
        self.modes.read().owner
    }

    fn set_owner(&mut self, value: bool) {
        self.modes.write().owner = value;
    }

    fn is_operator(&self) -> bool {
        self.modes.read().operator
    }

    fn set_operator(&mut self, value: bool) {
        self.modes.write().operator = value;
    }

    fn is_voice(&self) -> bool {
        self.modes.read().voice
    }

    fn set_voice(&mut self, value: bool) {
        self.modes.write().voice = value;
    }

    fn has_modes(&self) -> bool {
        self.modes.read().has_any()
    }

    fn mode_char(&self) -> Option<char> {
        self.modes.read().prefix_char()
    }
}
