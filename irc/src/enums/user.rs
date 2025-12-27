//! User-related enumerations

use serde::{Deserialize, Serialize};

/// User access level (global permissions)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(i8)]
pub enum UserAccessLevel {
    /// No access (disconnected/banned)
    NoAccess = -1,
    /// Default user level
    None = 0,
    /// Authenticated member
    Member = 1,
    /// Guide/Helper
    Guide = 2,
    /// System operator
    Sysop = 3,
    /// Server administrator
    Administrator = 4,
    /// Service account
    Service = 5,
}

impl Default for UserAccessLevel {
    fn default() -> Self {
        Self::None
    }
}

impl UserAccessLevel {
    /// Check if user is an operator (guide or higher)
    pub fn is_oper(&self) -> bool {
        *self >= Self::Guide
    }

    /// Check if user is a sysop or higher
    pub fn is_sysop(&self) -> bool {
        *self >= Self::Sysop
    }

    /// Check if user is an administrator or higher
    pub fn is_admin(&self) -> bool {
        *self >= Self::Administrator
    }
}
