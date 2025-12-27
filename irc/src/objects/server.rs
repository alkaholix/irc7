//! Server implementation

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use parking_lot::RwLock;
use uuid::Uuid;

use crate::enums::ProtocolType;
use crate::objects::Channel;
use crate::traits::{
    Channel as ChannelTrait, Connection, CredentialProvider, DataStore, Protocol,
    SecurityManager, Server as ServerTrait, User,
};

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub name: String,
    pub title: String,
    pub max_message_length: usize,
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub ping_interval: u64,
    pub ping_attempts: u32,
    pub max_channels: usize,
    pub max_connections: usize,
    pub max_authenticated_connections: usize,
    pub max_anonymous_connections: usize,
    pub max_guest_connections: usize,
    pub anonymous_allowed: bool,
    pub basic_authentication: bool,
    pub guest_mode_disabled: bool,
    pub user_registration_disabled: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "irc7".to_string(),
            title: "IRC7 Server".to_string(),
            max_message_length: 512,
            max_input_bytes: 1024,
            max_output_bytes: 4096,
            ping_interval: 60,
            ping_attempts: 3,
            max_channels: 100,
            max_connections: 10000,
            max_authenticated_connections: 10000,
            max_anonymous_connections: 100,
            max_guest_connections: 1000,
            anonymous_allowed: true,
            basic_authentication: true,
            guest_mode_disabled: false,
            user_registration_disabled: false,
        }
    }
}

/// Internal server state
struct ServerState {
    remote_ip: String,
    server_version: String,
    motd: Vec<String>,
    info: Vec<String>,
}

/// IRC Server implementation
pub struct Server {
    id: Uuid,
    creation_date: DateTime<Utc>,
    config: RwLock<ServerConfig>,
    state: RwLock<ServerState>,
    users: DashMap<Uuid, Arc<dyn User>>,
    channels: DashMap<Uuid, Arc<dyn ChannelTrait>>,
    protocols: DashMap<ProtocolType, Arc<dyn Protocol>>,
    security_manager: Arc<dyn SecurityManager>,
    credential_provider: Option<Arc<dyn CredentialProvider>>,
    data_store: Arc<dyn DataStore>,
    is_directory_server: bool,
}

impl Server {
    /// Create a new server
    pub fn new(
        config: ServerConfig,
        security_manager: Arc<dyn SecurityManager>,
        data_store: Arc<dyn DataStore>,
        credential_provider: Option<Arc<dyn CredentialProvider>>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            creation_date: Utc::now(),
            config: RwLock::new(config),
            state: RwLock::new(ServerState {
                remote_ip: "localhost".to_string(),
                server_version: env!("CARGO_PKG_VERSION").to_string(),
                motd: vec!["Welcome to IRC7".to_string()],
                info: Vec::new(),
            }),
            users: DashMap::new(),
            channels: DashMap::new(),
            protocols: DashMap::new(),
            security_manager,
            credential_provider,
            data_store,
            is_directory_server: false,
        }
    }

    /// Create a new directory server
    pub fn new_directory_server(
        config: ServerConfig,
        security_manager: Arc<dyn SecurityManager>,
        data_store: Arc<dyn DataStore>,
        credential_provider: Option<Arc<dyn CredentialProvider>>,
    ) -> Self {
        let mut server = Self::new(config, security_manager, data_store, credential_provider);
        server.is_directory_server = true;
        server
    }

    /// Add a protocol
    pub fn add_protocol(&self, protocol_type: ProtocolType, protocol: Arc<dyn Protocol>) {
        self.protocols.insert(protocol_type, protocol);
    }
}

#[async_trait]
impl ServerTrait for Server {
    fn id(&self) -> Uuid {
        self.id
    }

    fn short_id(&self) -> String {
        self.id.to_string()[..8].to_string()
    }

    fn name(&self) -> &str {
        // Can't return reference from RwLock
        ""
    }

    fn set_name(&mut self, name: String) {
        self.config.write().name = name;
    }

    fn title(&self) -> &str {
        ""
    }

    fn creation_date(&self) -> DateTime<Utc> {
        self.creation_date
    }

    fn anonymous_allowed(&self) -> bool {
        self.config.read().anonymous_allowed
    }

    fn channel_count(&self) -> usize {
        self.channels.len()
    }

    fn info(&self) -> &[String] {
        &[]
    }

    fn max_message_length(&self) -> usize {
        self.config.read().max_message_length
    }

    fn max_input_bytes(&self) -> usize {
        self.config.read().max_input_bytes
    }

    fn max_output_bytes(&self) -> usize {
        self.config.read().max_output_bytes
    }

    fn ping_interval(&self) -> u64 {
        self.config.read().ping_interval
    }

    fn ping_attempts(&self) -> u32 {
        self.config.read().ping_attempts
    }

    fn max_channels(&self) -> usize {
        self.config.read().max_channels
    }

    fn max_connections(&self) -> usize {
        self.config.read().max_connections
    }

    fn max_authenticated_connections(&self) -> usize {
        self.config.read().max_authenticated_connections
    }

    fn max_anonymous_connections(&self) -> usize {
        self.config.read().max_anonymous_connections
    }

    fn max_guest_connections(&self) -> usize {
        self.config.read().max_guest_connections
    }

    fn basic_authentication(&self) -> bool {
        self.config.read().basic_authentication
    }

    fn anonymous_connections(&self) -> bool {
        self.config.read().anonymous_allowed
    }

    fn net_invisible_count(&self) -> usize {
        0
    }

    fn net_server_count(&self) -> usize {
        1
    }

    fn net_user_count(&self) -> usize {
        self.users.len()
    }

    fn security_packages(&self) -> &str {
        ""
    }

    fn sysop_count(&self) -> usize {
        self.users
            .iter()
            .filter(|u| u.is_sysop())
            .count()
    }

    fn unknown_connection_count(&self) -> usize {
        self.users
            .iter()
            .filter(|u| !u.is_registered())
            .count()
    }

    fn remote_ip(&self) -> &str {
        ""
    }

    fn set_remote_ip(&mut self, ip: String) {
        self.state.write().remote_ip = ip;
    }

    fn guest_mode_disabled(&self) -> bool {
        self.config.read().guest_mode_disabled
    }

    fn set_guest_mode_disabled(&mut self, disabled: bool) {
        self.config.write().guest_mode_disabled = disabled;
    }

    fn user_registration_disabled(&self) -> bool {
        self.config.read().user_registration_disabled
    }

    fn set_user_registration_disabled(&mut self, disabled: bool) {
        self.config.write().user_registration_disabled = disabled;
    }

    fn is_directory_server(&self) -> bool {
        self.is_directory_server
    }

    fn server_version(&self) -> &str {
        ""
    }

    fn set_server_version(&mut self, version: String) {
        self.state.write().server_version = version;
    }

    fn add_user(&mut self, user: Arc<dyn User>) {
        self.users.insert(user.id(), user);
    }

    fn remove_user(&mut self, user: &dyn User) {
        self.users.remove(&user.id());
    }

    fn add_channel(&mut self, channel: Arc<dyn ChannelTrait>) {
        self.channels.insert(channel.id(), channel);
    }

    fn remove_channel(&mut self, channel: &dyn ChannelTrait) {
        self.channels.remove(&channel.id());
    }

    fn create_channel(&self, name: &str) -> Arc<dyn ChannelTrait> {
        Arc::new(Channel::new(name))
    }

    fn create_channel_with_key(
        &self,
        _creator: &dyn User,
        name: &str,
        _key: Option<&str>,
    ) -> Arc<dyn ChannelTrait> {
        Arc::new(Channel::new(name))
    }

    fn create_user(&self, _connection: Arc<dyn Connection>) -> Arc<dyn User> {
        unimplemented!("create_user requires full dependency injection")
    }

    fn get_users(&self) -> Vec<Arc<dyn User>> {
        self.users.iter().map(|r| r.value().clone()).collect()
    }

    fn get_user_by_nickname(&self, nickname: &str) -> Option<Arc<dyn User>> {
        let nick_lower = nickname.to_lowercase();
        self.users
            .iter()
            .find(|r| r.value().nickname().to_lowercase() == nick_lower)
            .map(|r| r.value().clone())
    }

    fn get_user_by_nickname_relative(
        &self,
        nickname: &str,
        _current_user: &dyn User,
    ) -> Option<Arc<dyn User>> {
        self.get_user_by_nickname(nickname)
    }

    fn get_users_by_list(&self, nicknames: &[String]) -> Vec<Arc<dyn User>> {
        nicknames
            .iter()
            .filter_map(|n| self.get_user_by_nickname(n))
            .collect()
    }

    fn get_channels(&self) -> Vec<Arc<dyn ChannelTrait>> {
        self.channels.iter().map(|r| r.value().clone()).collect()
    }

    fn get_supported_channel_modes(&self) -> &str {
        "imnpstklb"
    }

    fn get_supported_user_modes(&self) -> &str {
        "iowax"
    }

    fn get_protocols(&self) -> Vec<(ProtocolType, Arc<dyn Protocol>)> {
        self.protocols
            .iter()
            .map(|r| (*r.key(), r.value().clone()))
            .collect()
    }

    fn get_data_store(&self) -> Arc<dyn DataStore> {
        self.data_store.clone()
    }

    fn get_channel_by_name(&self, name: &str) -> Option<Arc<dyn ChannelTrait>> {
        let name_lower = name.to_lowercase();
        self.channels
            .iter()
            .find(|r| r.value().name().to_lowercase() == name_lower)
            .map(|r| r.value().clone())
    }

    fn get_protocol(&self, protocol_type: ProtocolType) -> Option<Arc<dyn Protocol>> {
        self.protocols.get(&protocol_type).map(|r| r.value().clone())
    }

    fn get_security_manager(&self) -> Arc<dyn SecurityManager> {
        self.security_manager.clone()
    }

    fn get_credential_provider(&self) -> Option<Arc<dyn CredentialProvider>> {
        self.credential_provider.clone()
    }

    fn shutdown(&mut self) {
        tracing::info!("Server shutting down...");
        // Disconnect all users
        for user in self.users.iter() {
            // user.value().disconnect("Server shutting down");
        }
        self.users.clear();
        self.channels.clear();
    }

    fn get_motd(&self) -> &[String] {
        &[]
    }

    fn set_motd(&mut self, motd: Vec<String>) {
        self.state.write().motd = motd;
    }

    fn process_cookie(&self, _user: &dyn User, _name: &str, _value: &str) {
        // Process authentication cookies
    }
}
