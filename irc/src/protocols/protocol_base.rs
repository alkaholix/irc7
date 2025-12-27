//! Base protocol implementation

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use parking_lot::RwLock;

use crate::enums::ProtocolType;
use crate::traits::{ChannelMember, Command, Protocol as ProtocolTrait, User};

/// Base protocol implementation
pub struct ProtocolBase {
    protocol_type: ProtocolType,
    commands: RwLock<HashMap<String, Arc<dyn Command>>>,
}

impl ProtocolBase {
    /// Create a new protocol
    pub fn new(protocol_type: ProtocolType) -> Self {
        Self {
            protocol_type,
            commands: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl ProtocolTrait for ProtocolBase {
    fn protocol_type(&self) -> ProtocolType {
        self.protocol_type
    }

    fn get_command(&self, name: &str) -> Option<Arc<dyn Command>> {
        self.commands.read().get(&name.to_uppercase()).cloned()
    }

    fn get_commands(&self) -> Vec<(String, Arc<dyn Command>)> {
        self.commands
            .read()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }

    fn add_command(&mut self, command: Arc<dyn Command>) {
        let name = command.name().to_uppercase();
        self.commands.write().insert(name, command);
    }

    fn add_command_with_name(&mut self, name: String, command: Arc<dyn Command>) {
        self.commands.write().insert(name.to_uppercase(), command);
    }

    fn update_command(&mut self, command: Arc<dyn Command>) {
        let name = command.name().to_uppercase();
        let mut commands = self.commands.write();
        if commands.contains_key(&name) {
            commands.insert(name, command);
        }
    }

    fn update_command_with_name(&mut self, name: String, command: Arc<dyn Command>) {
        let name = name.to_uppercase();
        let mut commands = self.commands.write();
        if commands.contains_key(&name) {
            commands.insert(name, command);
        }
    }

    fn flush_commands(&mut self) {
        self.commands.write().clear();
    }

    fn formatted_user(&self, member: &dyn ChannelMember) -> String {
        let prefix = member.mode_char().map(|c| c.to_string()).unwrap_or_default();
        format!("{}{}", prefix, member.get_user().nickname())
    }

    fn get_format(&self, user: &dyn User) -> String {
        user.get_address().full_address()
    }
}
