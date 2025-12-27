//! Access level enumerations

use serde::{Deserialize, Serialize};

/// Access level for channel access lists
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(i8)]
pub enum AccessLevel {
    None = -2,
    Deny = -1,
    Grant = 0,
    Voice = 1,
    Host = 2,
    Owner = 3,
    All = 4,
}

impl Default for AccessLevel {
    fn default() -> Self {
        Self::None
    }
}

/// Access operation result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessResult {
    Success,
    Failure,
    NoChange,
    NotFound,
    AlreadyExists,
}

/// Access operation error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessError {
    NoAccess,
    Denied,
    InvalidMask,
    InvalidLevel,
    ListFull,
}

/// Access operation types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessOperator {
    Add,
    Delete,
    Clear,
    List,
}
