//! Data management trait definitions

use async_trait::async_trait;
use serde_json::Value;

use super::ChatMessage;

/// Data regulator for managing incoming/outgoing message queues
pub trait DataRegulator: Send + Sync {
    /// Push an incoming message
    fn push_incoming(&self, message: Box<dyn ChatMessage>);

    /// Pop an incoming message
    fn pop_incoming(&self) -> Option<Box<dyn ChatMessage>>;

    /// Push an outgoing message
    fn push_outgoing(&self, message: String);

    /// Pop an outgoing message
    fn pop_outgoing(&self) -> Option<String>;

    /// Get incoming queue length
    fn incoming_queue_length(&self) -> usize;

    /// Get outgoing queue length
    fn outgoing_queue_length(&self) -> usize;

    /// Get outgoing bytes count
    fn outgoing_bytes(&self) -> usize;

    /// Check if incoming threshold is exceeded
    fn is_incoming_threshold_exceeded(&self) -> bool;

    /// Check if outgoing threshold is exceeded
    fn is_outgoing_threshold_exceeded(&self) -> bool;

    /// Purge all queues
    fn purge(&self);
}

/// Flood protection profile
pub trait FloodProtectionProfile: Send + Sync {
    /// Get maximum messages per second
    fn max_messages_per_second(&self) -> u32;

    /// Get maximum bytes per second
    fn max_bytes_per_second(&self) -> usize;

    /// Get maximum queue depth
    fn max_queue_depth(&self) -> usize;

    /// Get the delay between messages (in milliseconds)
    fn message_delay_ms(&self) -> u64;

    /// Check if a message should be delayed
    fn should_delay(&self) -> bool;

    /// Record a message sent
    fn record_message(&mut self);

    /// Reset counters
    fn reset(&mut self);
}

/// Flood protection manager
pub trait FloodProtectionManager: Send + Sync {
    /// Get the default profile
    fn default_profile(&self) -> Box<dyn FloodProtectionProfile>;

    /// Get a profile by name
    fn get_profile(&self, name: &str) -> Option<Box<dyn FloodProtectionProfile>>;

    /// Add a profile
    fn add_profile(&mut self, name: String, profile: Box<dyn FloodProtectionProfile>);
}

/// Data store for configuration and state
#[async_trait]
pub trait DataStore: Send + Sync {
    /// Get a value by key
    fn get(&self, key: &str) -> Option<Value>;

    /// Get a string value
    fn get_string(&self, key: &str) -> Option<String>;

    /// Get an integer value
    fn get_int(&self, key: &str) -> Option<i64>;

    /// Get a boolean value
    fn get_bool(&self, key: &str) -> Option<bool>;

    /// Set a value
    fn set(&mut self, key: &str, value: Value);

    /// Remove a value
    fn remove(&mut self, key: &str) -> Option<Value>;

    /// Check if a key exists
    fn contains(&self, key: &str) -> bool;

    /// Load from file
    async fn load(&mut self, path: &str) -> Result<(), std::io::Error>;

    /// Save to file
    async fn save(&self, path: &str) -> Result<(), std::io::Error>;
}
