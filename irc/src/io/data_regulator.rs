//! Data regulator for message queuing

use parking_lot::Mutex;
use std::collections::VecDeque;

use crate::traits::{ChatMessage, DataRegulator as DataRegulatorTrait};

/// Configuration for the data regulator
#[derive(Debug, Clone)]
pub struct DataRegulatorConfig {
    /// Maximum incoming queue depth
    pub max_incoming: usize,
    /// Maximum outgoing queue depth
    pub max_outgoing: usize,
    /// Maximum outgoing bytes
    pub max_outgoing_bytes: usize,
}

impl Default for DataRegulatorConfig {
    fn default() -> Self {
        Self {
            max_incoming: 100,
            max_outgoing: 500,
            max_outgoing_bytes: 16384,
        }
    }
}

/// Data regulator implementation
pub struct DataRegulator {
    config: DataRegulatorConfig,
    incoming: Mutex<VecDeque<Box<dyn ChatMessage>>>,
    outgoing: Mutex<VecDeque<String>>,
    outgoing_bytes: Mutex<usize>,
}

impl DataRegulator {
    /// Create a new data regulator
    pub fn new(config: DataRegulatorConfig) -> Self {
        Self {
            config,
            incoming: Mutex::new(VecDeque::new()),
            outgoing: Mutex::new(VecDeque::new()),
            outgoing_bytes: Mutex::new(0),
        }
    }

    /// Create with default config
    pub fn default_config() -> Self {
        Self::new(DataRegulatorConfig::default())
    }
}

impl DataRegulatorTrait for DataRegulator {
    fn push_incoming(&self, message: Box<dyn ChatMessage>) {
        let mut queue = self.incoming.lock();
        if queue.len() < self.config.max_incoming {
            queue.push_back(message);
        }
    }

    fn pop_incoming(&self) -> Option<Box<dyn ChatMessage>> {
        self.incoming.lock().pop_front()
    }

    fn push_outgoing(&self, message: String) {
        let mut queue = self.outgoing.lock();
        let mut bytes = self.outgoing_bytes.lock();

        if queue.len() < self.config.max_outgoing && *bytes + message.len() < self.config.max_outgoing_bytes {
            *bytes += message.len();
            queue.push_back(message);
        }
    }

    fn pop_outgoing(&self) -> Option<String> {
        let mut queue = self.outgoing.lock();
        if let Some(msg) = queue.pop_front() {
            let mut bytes = self.outgoing_bytes.lock();
            *bytes = bytes.saturating_sub(msg.len());
            Some(msg)
        } else {
            None
        }
    }

    fn incoming_queue_length(&self) -> usize {
        self.incoming.lock().len()
    }

    fn outgoing_queue_length(&self) -> usize {
        self.outgoing.lock().len()
    }

    fn outgoing_bytes(&self) -> usize {
        *self.outgoing_bytes.lock()
    }

    fn is_incoming_threshold_exceeded(&self) -> bool {
        self.incoming.lock().len() >= self.config.max_incoming
    }

    fn is_outgoing_threshold_exceeded(&self) -> bool {
        let queue = self.outgoing.lock();
        let bytes = self.outgoing_bytes.lock();
        queue.len() >= self.config.max_outgoing || *bytes >= self.config.max_outgoing_bytes
    }

    fn purge(&self) {
        self.incoming.lock().clear();
        self.outgoing.lock().clear();
        *self.outgoing_bytes.lock() = 0;
    }
}
