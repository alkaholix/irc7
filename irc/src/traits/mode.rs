//! Mode trait definitions

use async_trait::async_trait;

use super::User;

/// A mode that can be set on users or channels
pub trait Mode: Send + Sync {
    /// Get the mode character
    fn char(&self) -> char;

    /// Get the mode name
    fn name(&self) -> &str;

    /// Check if the mode requires a parameter
    fn requires_parameter(&self) -> bool;

    /// Check if the mode takes a parameter when being set
    fn takes_parameter_on_set(&self) -> bool;

    /// Check if the mode takes a parameter when being unset
    fn takes_parameter_on_unset(&self) -> bool;

    /// Validate a parameter value
    fn validate_parameter(&self, value: &str) -> bool;
}

/// A collection of modes
pub trait ModeCollection: Send + Sync {
    /// Get a mode by character
    fn get_mode(&self, char: char) -> Option<&dyn Mode>;

    /// Check if a mode is set
    fn has_mode(&self, char: char) -> bool;

    /// Get the mode value (for boolean modes)
    fn get_mode_value(&self, char: char) -> Option<bool>;

    /// Set the mode value
    fn set_mode_value(&mut self, char: char, value: bool);

    /// Get the mode string representation
    fn to_mode_string(&self) -> String;

    /// Get all mode characters
    fn all_chars(&self) -> Vec<char>;
}

/// User modes collection
pub trait UserModes: ModeCollection {
    /// Check if invisible mode is set
    fn is_invisible(&self) -> bool;

    /// Set invisible mode
    fn set_invisible(&mut self, value: bool);

    /// Check if operator mode is set
    fn is_oper(&self) -> bool;

    /// Set operator mode
    fn set_oper(&mut self, value: bool);

    /// Check if admin mode is set
    fn is_admin(&self) -> bool;

    /// Set admin mode
    fn set_admin(&mut self, value: bool);
}

/// Channel modes collection
pub trait ChannelModes: ModeCollection {
    /// Check if invite-only mode is set
    fn is_invite_only(&self) -> bool;

    /// Set invite-only mode
    fn set_invite_only(&mut self, value: bool);

    /// Check if moderated mode is set
    fn is_moderated(&self) -> bool;

    /// Set moderated mode
    fn set_moderated(&mut self, value: bool);

    /// Check if no external messages mode is set
    fn is_no_external(&self) -> bool;

    /// Set no external messages mode
    fn set_no_external(&mut self, value: bool);

    /// Check if private mode is set
    fn is_private(&self) -> bool;

    /// Set private mode
    fn set_private(&mut self, value: bool);

    /// Check if secret mode is set
    fn is_secret(&self) -> bool;

    /// Set secret mode
    fn set_secret(&mut self, value: bool);

    /// Check if topic lock mode is set
    fn is_topic_lock(&self) -> bool;

    /// Set topic lock mode
    fn set_topic_lock(&mut self, value: bool);

    /// Check if key mode is set
    fn has_key(&self) -> bool;

    /// Get the user limit (0 = no limit)
    fn user_limit(&self) -> usize;

    /// Set the user limit
    fn set_user_limit(&mut self, limit: usize);
}

/// Member modes (for a user in a channel)
pub trait MemberModes: Send + Sync {
    /// Check if owner mode is set
    fn is_owner(&self) -> bool;

    /// Set owner mode
    fn set_owner(&mut self, value: bool);

    /// Check if operator mode is set
    fn is_operator(&self) -> bool;

    /// Set operator mode
    fn set_operator(&mut self, value: bool);

    /// Check if voice mode is set
    fn is_voice(&self) -> bool;

    /// Set voice mode
    fn set_voice(&mut self, value: bool);

    /// Check if any mode is set
    fn has_any(&self) -> bool;

    /// Get the mode prefix character
    fn prefix_char(&self) -> Option<char>;
}

/// A mode rule for validation
#[async_trait]
pub trait ModeRule: Send + Sync {
    /// Validate a mode change
    fn validate(&self, user: &dyn User, mode: char, adding: bool, parameter: Option<&str>) -> bool;

    /// Get the required access level
    fn required_level(&self) -> crate::enums::ChannelAccessLevel;
}
