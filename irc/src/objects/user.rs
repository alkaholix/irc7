//! User implementation

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use uuid::Uuid;

use crate::access::UserAccess;
use crate::enums::{ChannelAccessLevel, UserAccessLevel};
use crate::modes::{ModeOperation, UserModesImpl};
use crate::objects::UserAddress;
use crate::traits::{
    Channel, ChannelMember, ChatFrame, Connection, DataRegulator, FloodProtectionProfile,
    ModeCollection, Protocol, Server, SupportPackage, User as UserTrait, UserModes, UserProps,
};

/// User properties implementation
#[derive(Debug, Clone, Default)]
pub struct UserPropsImpl {
    oid: Option<String>,
    role: Option<String>,
    props: HashMap<String, String>,
}

impl UserProps for UserPropsImpl {
    fn get(&self, key: &str) -> Option<&str> {
        self.props.get(key).map(|s| s.as_str())
    }

    fn set(&mut self, key: &str, value: String) {
        self.props.insert(key.to_string(), value);
    }

    fn oid(&self) -> Option<&str> {
        self.oid.as_deref()
    }

    fn set_oid(&mut self, oid: String) {
        self.oid = Some(oid);
    }

    fn role(&self) -> Option<&str> {
        self.role.as_deref()
    }

    fn set_role(&mut self, role: String) {
        self.role = Some(role);
    }
}

/// Internal user state
struct UserState {
    name: String,
    client: String,
    pass: String,
    away: bool,
    utf8: bool,
    guest: bool,
    registered: bool,
    authenticated: bool,
    level: UserAccessLevel,
    last_idle: DateTime<Utc>,
    last_ping: DateTime<Utc>,
    ping_count: u32,
    command_sequence: u64,
    mode_operations: Vec<ModeOperation>,
    channels: HashMap<Uuid, (Arc<dyn Channel>, Arc<dyn ChannelMember>)>,
}

/// IRC User implementation
pub struct User {
    id: Uuid,
    logged_on: DateTime<Utc>,
    server: Arc<dyn Server>,
    connection: Arc<dyn Connection>,
    protocol: RwLock<Arc<dyn Protocol>>,
    data_regulator: Arc<dyn DataRegulator>,
    flood_profile: Arc<dyn FloodProtectionProfile>,
    support_package: RwLock<Arc<dyn SupportPackage>>,
    modes: RwLock<UserModesImpl>,
    props: RwLock<UserPropsImpl>,
    access: RwLock<UserAccess>,
    address: RwLock<UserAddress>,
    state: RwLock<UserState>,
}

impl User {
    /// Create a new user
    pub fn new(
        server: Arc<dyn Server>,
        connection: Arc<dyn Connection>,
        protocol: Arc<dyn Protocol>,
        data_regulator: Arc<dyn DataRegulator>,
        flood_profile: Arc<dyn FloodProtectionProfile>,
        support_package: Arc<dyn SupportPackage>,
    ) -> Self {
        let now = Utc::now();
        let mut address = UserAddress::new();
        address.set_ip(connection.get_ip());

        Self {
            id: Uuid::new_v4(),
            logged_on: now,
            server,
            connection,
            protocol: RwLock::new(protocol),
            data_regulator,
            flood_profile,
            support_package: RwLock::new(support_package),
            modes: RwLock::new(UserModesImpl::new()),
            props: RwLock::new(UserPropsImpl::default()),
            access: RwLock::new(UserAccess::new()),
            address: RwLock::new(address),
            state: RwLock::new(UserState {
                name: String::new(),
                client: String::new(),
                pass: String::new(),
                away: false,
                utf8: false,
                guest: false,
                registered: false,
                authenticated: false,
                level: UserAccessLevel::None,
                last_idle: now,
                last_ping: now,
                ping_count: 0,
                command_sequence: 0,
                mode_operations: Vec::new(),
                channels: HashMap::new(),
            }),
        }
    }
}

#[async_trait]
impl UserTrait for User {
    fn server(&self) -> Arc<dyn Server> {
        self.server.clone()
    }

    fn id(&self) -> Uuid {
        self.id
    }

    fn short_id(&self) -> String {
        self.id.to_string()[..8].to_string()
    }

    fn name(&self) -> &str {
        // Return empty string since we can't return a reference to RwLock content
        ""
    }

    fn set_name(&mut self, name: String) {
        self.state.write().name = name;
    }

    fn nickname(&self) -> &str {
        ""
    }

    fn set_nickname(&mut self, nickname: String) {
        self.state.write().name = nickname.clone();
        self.address.write().set_nickname(&nickname);
    }

    fn client(&self) -> &str {
        ""
    }

    fn set_client(&mut self, client: String) {
        self.state.write().client = client;
    }

    fn pass(&self) -> &str {
        ""
    }

    fn set_pass(&mut self, pass: String) {
        self.state.write().pass = pass;
    }

    fn is_away(&self) -> bool {
        self.state.read().away
    }

    fn set_away(&mut self, away: bool) {
        self.state.write().away = away;
    }

    fn last_idle(&self) -> DateTime<Utc> {
        self.state.read().last_idle
    }

    fn set_last_idle(&mut self, time: DateTime<Utc>) {
        self.state.write().last_idle = time;
    }

    fn logged_on(&self) -> DateTime<Utc> {
        self.logged_on
    }

    fn is_utf8(&self) -> bool {
        self.state.read().utf8
    }

    fn set_utf8(&mut self, utf8: bool) {
        self.state.write().utf8 = utf8;
    }

    fn get_next_frame(&mut self) -> Option<Box<dyn ChatFrame>> {
        // Would pop from data regulator and create frame
        None
    }

    fn change_nickname(&mut self, new_nick: &str, utf8_prefix: bool) {
        let nickname = if utf8_prefix {
            format!("'{}", new_nick)
        } else {
            new_nick.to_string()
        };
        self.set_nickname(nickname);
    }

    fn set_guest(&mut self, guest: bool) {
        if !self.server.guest_mode_disabled() {
            self.state.write().guest = guest;
        }
    }

    fn set_away_with_message(&mut self, _message: &str) {
        self.state.write().away = true;
        // Would broadcast to channels
    }

    fn set_back(&mut self) {
        self.state.write().away = false;
        // Would broadcast to channels
    }

    fn set_level(&mut self, level: UserAccessLevel) {
        self.state.write().level = level;
    }

    fn get_level(&self) -> UserAccessLevel {
        self.state.read().level
    }

    fn broadcast_to_channels(&self, data: &str, exclude_self: bool) {
        let channels = self.state.read().channels.clone();
        for (_, (channel, _)) in channels.iter() {
            if exclude_self {
                channel.send_except(data, self);
            } else {
                channel.send(data);
            }
        }
    }

    fn add_channel(&mut self, channel: Arc<dyn Channel>, member: Arc<dyn ChannelMember>) {
        self.state.write().channels.insert(channel.id(), (channel, member));
    }

    fn remove_channel(&mut self, channel: &dyn Channel) {
        self.state.write().channels.remove(&channel.id());
    }

    fn get_channel_member_info(&self, channel: &dyn Channel) -> Option<Arc<dyn ChannelMember>> {
        self.state
            .read()
            .channels
            .get(&channel.id())
            .map(|(_, m)| m.clone())
    }

    fn get_channel_info(&self, name: &str) -> Option<(Arc<dyn Channel>, Arc<dyn ChannelMember>)> {
        let name_lower = name.to_lowercase();
        self.state
            .read()
            .channels
            .values()
            .find(|(c, _)| c.name().to_lowercase() == name_lower)
            .cloned()
    }

    fn get_channels(&self) -> HashMap<Uuid, (Arc<dyn Channel>, Arc<dyn ChannelMember>)> {
        self.state.read().channels.clone()
    }

    fn send(&self, message: &str) {
        self.data_regulator.push_outgoing(message.to_string());
    }

    fn send_with_level(&self, message: &str, _access_level: ChannelAccessLevel) {
        self.send(message);
    }

    fn flush(&mut self) {
        // Would flush data regulator to connection
    }

    fn disconnect(&mut self, message: &str) {
        self.state.write().mode_operations.clear();
        // Would disconnect via connection
        tracing::info!("Disconnecting user: {}", message);
    }

    fn get_data_regulator(&self) -> Arc<dyn DataRegulator> {
        self.data_regulator.clone()
    }

    fn get_flood_protection_profile(&self) -> Arc<dyn FloodProtectionProfile> {
        self.flood_profile.clone()
    }

    fn get_support_package(&self) -> Arc<dyn SupportPackage> {
        self.support_package.read().clone()
    }

    fn set_support_package(&mut self, package: Arc<dyn SupportPackage>) {
        *self.support_package.write() = package;
    }

    fn set_protocol(&mut self, protocol: Arc<dyn Protocol>) {
        *self.protocol.write() = protocol;
    }

    fn get_protocol(&self) -> Arc<dyn Protocol> {
        self.protocol.read().clone()
    }

    fn get_connection(&self) -> Arc<dyn Connection> {
        self.connection.clone()
    }

    fn get_address(&self) -> &UserAddress {
        // This is tricky with RwLock - would need different approach
        // For now return empty
        unsafe { &*(&UserAddress::new() as *const UserAddress) }
    }

    fn is_guest(&self) -> bool {
        self.state.read().guest
    }

    fn is_registered(&self) -> bool {
        self.state.read().registered
    }

    fn is_authenticated(&self) -> bool {
        self.state.read().authenticated
    }

    fn is_anon(&self) -> bool {
        self.support_package.read().name() == "ANON"
    }

    fn is_sysop(&self) -> bool {
        self.modes.read().is_oper()
    }

    fn is_administrator(&self) -> bool {
        self.modes.read().is_admin()
    }

    fn is_on(&self, channel: &dyn Channel) -> bool {
        self.state.read().channels.contains_key(&channel.id())
    }

    fn promote_to_administrator(&mut self) {
        self.modes.write().set_admin(true);
        self.state.write().level = UserAccessLevel::Administrator;
    }

    fn promote_to_sysop(&mut self) {
        self.modes.write().set_oper(true);
        self.state.write().level = UserAccessLevel::Sysop;
    }

    fn promote_to_guide(&mut self) {
        self.modes.write().set_oper(true);
        self.state.write().level = UserAccessLevel::Guide;
    }

    fn disconnect_if_outgoing_threshold_exceeded(&mut self) -> bool {
        if self.data_regulator.is_outgoing_threshold_exceeded() {
            self.data_regulator.purge();
            self.disconnect("Output quota exceeded");
            return true;
        }
        false
    }

    fn disconnect_if_incoming_threshold_exceeded(&mut self) -> bool {
        if self.data_regulator.is_incoming_threshold_exceeded() {
            self.data_regulator.purge();
            self.disconnect("Input quota exceeded");
            return true;
        }
        false
    }

    fn register(&mut self) {
        let mut state = self.state.write();
        state.authenticated = true;
        state.registered = true;
    }

    fn authenticate(&mut self) {
        self.state.write().authenticated = true;
    }

    fn disconnect_if_inactive(&mut self) {
        let state = self.state.read();
        let seconds = (Utc::now() - state.last_ping).num_seconds() as u64;
        let ping_interval = self.server.ping_interval();
        let ping_attempts = self.server.ping_attempts();

        if seconds > (state.ping_count as u64 + 1) * ping_interval {
            drop(state);
            let mut state = self.state.write();
            if state.ping_count < ping_attempts {
                state.ping_count += 1;
                // Would send PING
            } else {
                drop(state);
                self.data_regulator.purge();
                self.disconnect("Ping timeout");
            }
        }
    }

    fn get_mode_operations(&self) -> &[ModeOperation] {
        // Can't return slice from RwLock
        &[]
    }

    fn add_mode_operation(&mut self, op: ModeOperation) {
        self.state.write().mode_operations.push(op);
    }

    fn clear_mode_operations(&mut self) {
        self.state.write().mode_operations.clear();
    }
}
