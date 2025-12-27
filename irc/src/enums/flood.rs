//! Flood protection enumerations

use serde::{Deserialize, Serialize};

/// Flood protection level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FloodProtectionLevel {
    /// No flood protection
    None = 0,
    /// Low flood protection
    Low = 1,
    /// Medium flood protection
    Medium = 2,
    /// High flood protection
    High = 3,
    /// Maximum flood protection
    Maximum = 4,
}

impl Default for FloodProtectionLevel {
    fn default() -> Self {
        Self::Medium
    }
}

/// Result of flood check
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloodResult {
    /// Operation allowed
    Ok,
    /// Rate limited, message delayed
    Delayed,
    /// Exceeded threshold, connection terminated
    Exceeded,
}

/// Result of data saturation check
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaturationResult {
    /// Below threshold
    Ok,
    /// Approaching threshold
    Warning,
    /// Exceeded threshold
    Exceeded,
}
