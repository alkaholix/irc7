//! Mode operation representation

/// A pending mode operation
#[derive(Debug, Clone)]
pub struct ModeOperation {
    /// The mode character
    pub mode: char,
    /// Whether the mode is being added (true) or removed (false)
    pub adding: bool,
    /// Optional parameter for the mode
    pub parameter: Option<String>,
}

impl ModeOperation {
    /// Create a new mode operation
    pub fn new(mode: char, adding: bool, parameter: Option<String>) -> Self {
        Self {
            mode,
            adding,
            parameter,
        }
    }

    /// Create an add operation
    pub fn add(mode: char, parameter: Option<String>) -> Self {
        Self::new(mode, true, parameter)
    }

    /// Create a remove operation
    pub fn remove(mode: char, parameter: Option<String>) -> Self {
        Self::new(mode, false, parameter)
    }
}
