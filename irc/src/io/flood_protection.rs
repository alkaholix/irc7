//! Flood protection implementation

use std::collections::HashMap;
use std::time::Instant;

use parking_lot::RwLock;

use crate::enums::FloodProtectionLevel;
use crate::traits::{
    FloodProtectionManager as FloodProtectionManagerTrait,
    FloodProtectionProfile as FloodProtectionProfileTrait,
};

/// Flood protection profile configuration
#[derive(Debug, Clone)]
pub struct FloodProfileConfig {
    pub max_messages_per_second: u32,
    pub max_bytes_per_second: usize,
    pub max_queue_depth: usize,
    pub message_delay_ms: u64,
}

impl FloodProfileConfig {
    /// Create profile for a given protection level
    pub fn for_level(level: FloodProtectionLevel) -> Self {
        match level {
            FloodProtectionLevel::None => Self {
                max_messages_per_second: 1000,
                max_bytes_per_second: 1048576,
                max_queue_depth: 1000,
                message_delay_ms: 0,
            },
            FloodProtectionLevel::Low => Self {
                max_messages_per_second: 20,
                max_bytes_per_second: 8192,
                max_queue_depth: 100,
                message_delay_ms: 50,
            },
            FloodProtectionLevel::Medium => Self {
                max_messages_per_second: 10,
                max_bytes_per_second: 4096,
                max_queue_depth: 50,
                message_delay_ms: 100,
            },
            FloodProtectionLevel::High => Self {
                max_messages_per_second: 5,
                max_bytes_per_second: 2048,
                max_queue_depth: 25,
                message_delay_ms: 200,
            },
            FloodProtectionLevel::Maximum => Self {
                max_messages_per_second: 2,
                max_bytes_per_second: 1024,
                max_queue_depth: 10,
                message_delay_ms: 500,
            },
        }
    }
}

/// Flood protection profile implementation
pub struct FloodProtectionProfile {
    config: FloodProfileConfig,
    messages_this_second: RwLock<u32>,
    bytes_this_second: RwLock<usize>,
    last_reset: RwLock<Instant>,
    last_message: RwLock<Instant>,
}

impl FloodProtectionProfile {
    /// Create a new profile
    pub fn new(config: FloodProfileConfig) -> Self {
        let now = Instant::now();
        Self {
            config,
            messages_this_second: RwLock::new(0),
            bytes_this_second: RwLock::new(0),
            last_reset: RwLock::new(now),
            last_message: RwLock::new(now),
        }
    }

    /// Create with a protection level
    pub fn with_level(level: FloodProtectionLevel) -> Self {
        Self::new(FloodProfileConfig::for_level(level))
    }

    fn check_reset(&self) {
        let now = Instant::now();
        let last = *self.last_reset.read();
        if now.duration_since(last).as_secs() >= 1 {
            *self.messages_this_second.write() = 0;
            *self.bytes_this_second.write() = 0;
            *self.last_reset.write() = now;
        }
    }
}

impl FloodProtectionProfileTrait for FloodProtectionProfile {
    fn max_messages_per_second(&self) -> u32 {
        self.config.max_messages_per_second
    }

    fn max_bytes_per_second(&self) -> usize {
        self.config.max_bytes_per_second
    }

    fn max_queue_depth(&self) -> usize {
        self.config.max_queue_depth
    }

    fn message_delay_ms(&self) -> u64 {
        self.config.message_delay_ms
    }

    fn should_delay(&self) -> bool {
        self.check_reset();
        let messages = *self.messages_this_second.read();
        messages >= self.config.max_messages_per_second
    }

    fn record_message(&mut self) {
        self.check_reset();
        *self.messages_this_second.write() += 1;
        *self.last_message.write() = Instant::now();
    }

    fn reset(&mut self) {
        *self.messages_this_second.write() = 0;
        *self.bytes_this_second.write() = 0;
        *self.last_reset.write() = Instant::now();
    }
}

/// Flood protection manager
pub struct FloodProtectionManager {
    profiles: RwLock<HashMap<String, FloodProfileConfig>>,
    default_level: FloodProtectionLevel,
}

impl Default for FloodProtectionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl FloodProtectionManager {
    /// Create a new manager
    pub fn new() -> Self {
        Self {
            profiles: RwLock::new(HashMap::new()),
            default_level: FloodProtectionLevel::Medium,
        }
    }

    /// Set the default protection level
    pub fn set_default_level(&mut self, level: FloodProtectionLevel) {
        self.default_level = level;
    }
}

impl FloodProtectionManagerTrait for FloodProtectionManager {
    fn default_profile(&self) -> Box<dyn FloodProtectionProfileTrait> {
        Box::new(FloodProtectionProfile::with_level(self.default_level))
    }

    fn get_profile(&self, name: &str) -> Option<Box<dyn FloodProtectionProfileTrait>> {
        self.profiles
            .read()
            .get(name)
            .map(|config| Box::new(FloodProtectionProfile::new(config.clone())) as Box<dyn FloodProtectionProfileTrait>)
    }

    fn add_profile(&mut self, name: String, profile: Box<dyn FloodProtectionProfileTrait>) {
        let config = FloodProfileConfig {
            max_messages_per_second: profile.max_messages_per_second(),
            max_bytes_per_second: profile.max_bytes_per_second(),
            max_queue_depth: profile.max_queue_depth(),
            message_delay_ms: profile.message_delay_ms(),
        };
        self.profiles.write().insert(name, config);
    }
}
