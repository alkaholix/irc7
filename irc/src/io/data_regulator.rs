//! Data regulator for message queuing

use parking_lot::Mutex;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::message::IrcMessage;
use crate::traits::{ChatMessage, DataRegulator as DataRegulatorTrait};

/// Configuration for the data regulator
#[derive(Debug, Clone)]
pub struct DataRegulatorConfig {
    /// Maximum incoming queue depth
    pub max_incoming: usize,
    /// Maximum outgoing queue depth
    pub max_outgoing: usize,
    /// Maximum incoming bytes
    pub max_incoming_bytes: usize,
    /// Maximum outgoing bytes
    pub max_outgoing_bytes: usize,
}

impl Default for DataRegulatorConfig {
    fn default() -> Self {
        Self {
            max_incoming: 100,
            max_outgoing: 500,
            max_incoming_bytes: 512,
            max_outgoing_bytes: 16384,
        }
    }
}

/// Data regulator implementation
pub struct DataRegulator {
    config: DataRegulatorConfig,
    incoming: Mutex<VecDeque<IrcMessage>>,
    outgoing: Mutex<VecDeque<String>>,
    incoming_bytes: AtomicUsize,
    outgoing_bytes: AtomicUsize,
}

impl DataRegulator {
    /// Create a new data regulator
    pub fn new(config: DataRegulatorConfig) -> Self {
        Self {
            config,
            incoming: Mutex::new(VecDeque::new()),
            outgoing: Mutex::new(VecDeque::new()),
            incoming_bytes: AtomicUsize::new(0),
            outgoing_bytes: AtomicUsize::new(0),
        }
    }

    /// Create with default config
    pub fn default_config() -> Self {
        Self::new(DataRegulatorConfig::default())
    }

    /// Push an incoming IrcMessage directly
    pub fn push_incoming_message(&self, message: IrcMessage) {
        let mut queue = self.incoming.lock();
        if queue.len() < self.config.max_incoming {
            self.incoming_bytes.fetch_add(message.original_text().len(), Ordering::Relaxed);
            queue.push_back(message);
        }
    }

    /// Pop an incoming IrcMessage directly
    pub fn pop_incoming_message(&self) -> Option<IrcMessage> {
        let mut queue = self.incoming.lock();
        if let Some(msg) = queue.pop_front() {
            self.incoming_bytes.fetch_sub(msg.original_text().len(), Ordering::Relaxed);
            Some(msg)
        } else {
            None
        }
    }

    /// Peek at the next incoming IrcMessage without removing it
    pub fn peek_incoming_message(&self) -> Option<IrcMessage> {
        self.incoming.lock().front().cloned()
    }
}

impl DataRegulatorTrait for DataRegulator {
    fn push_incoming(&self, message: Box<dyn ChatMessage>) {
        // Parse the message if it's not already an IrcMessage
        let irc_msg = IrcMessage::parse(message.original_text());
        self.push_incoming_message(irc_msg);
    }

    fn pop_incoming(&self) -> Option<Box<dyn ChatMessage>> {
        self.pop_incoming_message().map(|m| Box::new(m) as Box<dyn ChatMessage>)
    }

    fn peek_incoming(&self) -> Option<Box<dyn ChatMessage>> {
        self.peek_incoming_message().map(|m| Box::new(m) as Box<dyn ChatMessage>)
    }

    fn push_outgoing(&self, message: String) {
        let mut queue = self.outgoing.lock();
        let current_bytes = self.outgoing_bytes.load(Ordering::Relaxed);

        if queue.len() < self.config.max_outgoing && current_bytes + message.len() < self.config.max_outgoing_bytes {
            self.outgoing_bytes.fetch_add(message.len(), Ordering::Relaxed);
            queue.push_back(message);
        }
    }

    fn pop_outgoing(&self) -> Option<String> {
        let mut queue = self.outgoing.lock();
        if let Some(msg) = queue.pop_front() {
            self.outgoing_bytes.fetch_sub(msg.len(), Ordering::Relaxed);
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

    fn incoming_bytes(&self) -> usize {
        self.incoming_bytes.load(Ordering::Relaxed)
    }

    fn outgoing_bytes(&self) -> usize {
        self.outgoing_bytes.load(Ordering::Relaxed)
    }

    fn is_incoming_threshold_exceeded(&self) -> bool {
        self.incoming_bytes.load(Ordering::Relaxed) >= self.config.max_incoming_bytes
    }

    fn is_outgoing_threshold_exceeded(&self) -> bool {
        self.outgoing_bytes.load(Ordering::Relaxed) >= self.config.max_outgoing_bytes
    }

    fn purge(&self) {
        self.incoming.lock().clear();
        self.outgoing.lock().clear();
        self.incoming_bytes.store(0, Ordering::Relaxed);
        self.outgoing_bytes.store(0, Ordering::Relaxed);
    }
}
