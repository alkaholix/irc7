//! Channel implementation

use std::collections::HashSet;
use std::sync::Arc;

use chrono::Utc;
use parking_lot::RwLock;
use regex::Regex;
use uuid::Uuid;

use crate::access::{AccessListImpl, ChannelAccess};
use crate::enums::{AccessLevel, ChannelAccessLevel, ChannelAccessResult, IrcError, UserAccessLevel};
use crate::modes::ChannelModesImpl;
use crate::objects::Member;
use crate::traits::{
    AccessEntry, AccessList, Channel as ChannelTrait, ChannelMember, ChannelModes, ChannelProps,
    Server, User,
};

/// Channel properties
#[derive(Debug, Clone, Default)]
pub struct ChannelPropsImpl {
    topic: String,
    onjoin: Option<String>,
    onpart: Option<String>,
    member_key: Option<String>,
    host_key: Option<String>,
    owner_key: Option<String>,
    name: String,
    creation: String,
}

impl ChannelProps for ChannelPropsImpl {
    fn topic(&self) -> &str {
        &self.topic
    }

    fn set_topic(&mut self, topic: String) {
        self.topic = topic;
    }

    fn onjoin(&self) -> Option<&str> {
        self.onjoin.as_deref()
    }

    fn set_onjoin(&mut self, message: Option<String>) {
        self.onjoin = message;
    }

    fn onpart(&self) -> Option<&str> {
        self.onpart.as_deref()
    }

    fn set_onpart(&mut self, message: Option<String>) {
        self.onpart = message;
    }

    fn get(&self, name: &str) -> Option<&str> {
        match name.to_uppercase().as_str() {
            "TOPIC" => Some(&self.topic),
            "NAME" => Some(&self.name),
            "CREATION" => Some(&self.creation),
            "ONJOIN" => self.onjoin.as_deref(),
            "ONPART" => self.onpart.as_deref(),
            "MEMBERKEY" => self.member_key.as_deref(),
            "HOSTKEY" => self.host_key.as_deref(),
            "OWNERKEY" => self.owner_key.as_deref(),
            _ => None,
        }
    }

    fn set(&mut self, name: &str, value: String) {
        match name.to_uppercase().as_str() {
            "TOPIC" => self.topic = value,
            "NAME" => self.name = value,
            "CREATION" => self.creation = value,
            "ONJOIN" => self.onjoin = Some(value),
            "ONPART" => self.onpart = Some(value),
            "MEMBERKEY" => self.member_key = Some(value),
            "HOSTKEY" => self.host_key = Some(value),
            "OWNERKEY" => self.owner_key = Some(value),
            _ => {}
        }
    }

    fn member_key(&self) -> Option<&str> {
        self.member_key.as_deref()
    }

    fn set_member_key(&mut self, key: Option<String>) {
        self.member_key = key;
    }

    fn host_key(&self) -> Option<&str> {
        self.host_key.as_deref()
    }

    fn set_host_key(&mut self, key: Option<String>) {
        self.host_key = key;
    }

    fn owner_key(&self) -> Option<&str> {
        self.owner_key.as_deref()
    }

    fn set_owner_key(&mut self, key: Option<String>) {
        self.owner_key = key;
    }
}

/// Internal channel state
struct ChannelState {
    members: Vec<Arc<dyn ChannelMember>>,
    invite_list: HashSet<String>,
    topic_changed: i64,
}

/// IRC Channel implementation
pub struct Channel {
    id: Uuid,
    name: String,
    creation: i64,
    modes: RwLock<ChannelModesImpl>,
    props: RwLock<ChannelPropsImpl>,
    access: RwLock<ChannelAccess>,
    state: RwLock<ChannelState>,
}

impl Channel {
    /// Create a new channel
    pub fn new(name: &str) -> Self {
        let now = Utc::now().timestamp();
        let mut props = ChannelPropsImpl::default();
        props.name = name.to_string();
        props.creation = now.to_string();

        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            creation: now,
            modes: RwLock::new(ChannelModesImpl::new()),
            props: RwLock::new(props),
            access: RwLock::new(ChannelAccess::new()),
            state: RwLock::new(ChannelState {
                members: Vec::new(),
                invite_list: HashSet::new(),
                topic_changed: now,
            }),
        }
    }

    /// Validate a channel name
    pub fn valid_name(name: &str) -> bool {
        // IRCX channel regex pattern
        let regex = Regex::new(r"^[%#&][^\x00\x07\x0a\x0d :,]+$").unwrap();
        regex.is_match(name)
    }

    /// Add a member to the channel
    fn add_member(&self, user: Arc<dyn User>, access_result: ChannelAccessResult) -> Arc<dyn ChannelMember> {
        let member: Arc<dyn ChannelMember> = match access_result {
            ChannelAccessResult::SuccessOwner => Arc::new(Member::new_owner(user)),
            ChannelAccessResult::SuccessHost => Arc::new(Member::new_operator(user)),
            ChannelAccessResult::SuccessVoice => Arc::new(Member::new_voice(user)),
            _ => Arc::new(Member::new(user)),
        };
        self.state.write().members.push(member.clone());
        member
    }

    /// Remove a member from the channel
    fn remove_member(&self, user: &dyn User) {
        let user_id = user.id();
        self.state.write().members.retain(|m| m.get_user().id() != user_id);
    }

    /// Check host/owner key
    fn check_host_key(&self, key: Option<&str>) -> ChannelAccessResult {
        let key = match key {
            Some(k) if !k.is_empty() => k,
            _ => return ChannelAccessResult::None,
        };

        let props = self.props.read();
        if props.owner_key.as_deref() == Some(key) {
            return ChannelAccessResult::SuccessOwner;
        }
        if props.host_key.as_deref() == Some(key) {
            return ChannelAccessResult::SuccessHost;
        }
        ChannelAccessResult::None
    }

    /// Check member key
    fn check_member_key(&self, key: Option<&str>) -> ChannelAccessResult {
        let modes = self.modes.read();
        if modes.has_key() {
            let props = self.props.read();
            if props.member_key.as_deref() == key {
                ChannelAccessResult::SuccessMember
            } else {
                ChannelAccessResult::ErrBadChannelKey
            }
        } else {
            ChannelAccessResult::None
        }
    }

    /// Check invite only
    fn check_invite_only(&self, user: &dyn User) -> ChannelAccessResult {
        if self.modes.read().is_invite_only() {
            let address = user.get_address().address();
            if self.state.read().invite_list.contains(&address) {
                ChannelAccessResult::SuccessMember
            } else {
                ChannelAccessResult::ErrInviteOnlyChan
            }
        } else {
            ChannelAccessResult::None
        }
    }

    /// Check user limit
    fn check_user_limit(&self, is_goto: bool) -> ChannelAccessResult {
        let limit = self.modes.read().user_limit();
        if limit == 0 {
            return ChannelAccessResult::None;
        }

        let effective_limit = if is_goto {
            (limit as f64 * 1.2).ceil() as usize
        } else {
            limit
        };

        if self.state.read().members.len() >= effective_limit {
            ChannelAccessResult::ErrChannelIsFull
        } else {
            ChannelAccessResult::None
        }
    }

    /// Check oper status
    fn check_oper(&self, user: &dyn User) -> ChannelAccessResult {
        if user.get_level() >= UserAccessLevel::Guide {
            ChannelAccessResult::SuccessOwner
        } else {
            ChannelAccessResult::None
        }
    }

    /// Get channel access level for user
    fn get_channel_access(&self, user: &dyn User) -> AccessLevel {
        let address = user.get_address().full_address();
        self.access.read().check(&address)
    }
}

impl ChannelTrait for Channel {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn creation(&self) -> i64 {
        self.creation
    }

    fn topic_changed(&self) -> i64 {
        self.state.read().topic_changed
    }

    fn set_topic_changed(&mut self, timestamp: i64) {
        self.state.write().topic_changed = timestamp;
    }

    fn access(&self) -> Arc<dyn AccessList> {
        Arc::new(AccessListImpl::new()) // Return a new access list - in practice would share
    }

    fn get_member(&self, user: &dyn User) -> Option<Arc<dyn ChannelMember>> {
        let user_id = user.id();
        self.state
            .read()
            .members
            .iter()
            .find(|m| m.get_user().id() == user_id)
            .cloned()
    }

    fn get_member_by_nickname(&self, nickname: &str) -> Option<Arc<dyn ChannelMember>> {
        let nick_lower = nickname.to_lowercase();
        self.state
            .read()
            .members
            .iter()
            .find(|m| m.get_user().nickname().to_lowercase() == nick_lower)
            .cloned()
    }

    fn has_user(&self, user: &dyn User) -> bool {
        let user_id = user.id();
        self.state
            .read()
            .members
            .iter()
            .any(|m| m.get_user().id() == user_id)
    }

    fn send(&self, message: &str) {
        for member in self.state.read().members.iter() {
            member.get_user().send(message);
        }
    }

    fn send_except(&self, message: &str, exclude: &dyn User) {
        let exclude_id = exclude.id();
        for member in self.state.read().members.iter() {
            if member.get_user().id() != exclude_id {
                member.get_user().send(message);
            }
        }
    }

    fn send_with_level(&self, message: &str, access_level: ChannelAccessLevel) {
        for member in self.state.read().members.iter() {
            if member.get_level() >= access_level {
                member.get_user().send(message);
            }
        }
    }

    fn join(
        &mut self,
        user: Arc<dyn User>,
        access_result: ChannelAccessResult,
    ) -> Result<Arc<dyn ChannelMember>, IrcError> {
        let member = self.add_member(user, access_result);
        Ok(member)
    }

    fn part(&mut self, user: &dyn User) {
        self.remove_member(user);
    }

    fn quit(&mut self, user: &dyn User) {
        self.remove_member(user);
    }

    fn kick(&mut self, _source: &dyn User, target: &dyn User, _reason: &str) {
        self.remove_member(target);
    }

    fn send_message(&self, user: &dyn User, message: &str) {
        let msg = format!(":{}!{}@{} PRIVMSG {} :{}",
            user.nickname(),
            user.get_address().user,
            user.get_address().host,
            self.name,
            message
        );
        self.send_except(&msg, user);
    }

    fn send_notice(&self, user: &dyn User, message: &str) {
        let msg = format!(":{}!{}@{} NOTICE {} :{}",
            user.nickname(),
            user.get_address().user,
            user.get_address().host,
            self.name,
            message
        );
        self.send_except(&msg, user);
    }

    fn get_members(&self) -> Vec<Arc<dyn ChannelMember>> {
        self.state.read().members.clone()
    }

    fn member_count(&self) -> usize {
        self.state.read().members.len()
    }

    fn can_be_modified_by(&self, source: &dyn User) -> bool {
        self.has_user(source) || source.get_level() >= UserAccessLevel::Sysop
    }

    fn can_modify_member(
        &self,
        source: &dyn ChannelMember,
        target: &dyn ChannelMember,
        required_level: ChannelAccessLevel,
    ) -> Result<(), IrcError> {
        // Oper check
        let target_level = target.get_user().get_level();
        let source_level = source.get_user().get_level();

        if target_level >= UserAccessLevel::Guide {
            if source_level < UserAccessLevel::Guide {
                return Err(IrcError::ErrNoIrcOp);
            }
            if source_level < UserAccessLevel::Sysop && source_level < target_level {
                return Err(IrcError::ErrNoPerms);
            }
        }

        if source.get_level() >= required_level && source.get_level() >= target.get_level() {
            return Ok(());
        }

        if !source.is_owner()
            && (required_level >= ChannelAccessLevel::ChatOwner
                || target.get_level() >= ChannelAccessLevel::ChatOwner)
        {
            return Err(IrcError::ErrChanQPrivsNeeded);
        }

        Err(IrcError::ErrChanOpPrivsNeeded)
    }

    fn process_channel_error(
        &self,
        _error: IrcError,
        _server: &dyn Server,
        _source: &dyn User,
        _target_name: &str,
        _data: &str,
    ) {
        // Error processing would send appropriate IRC error messages
    }

    fn send_topic(&self, user: &dyn User) {
        let topic = self.props.read().topic.clone();
        if topic.is_empty() {
            user.send(&format!("331 {} {} :No topic is set", user.nickname(), self.name));
        } else {
            user.send(&format!("332 {} {} :{}", user.nickname(), self.name, topic));
        }
    }

    fn send_topic_all(&self) {
        for member in self.state.read().members.iter() {
            self.send_topic(&*member.get_user());
        }
    }

    fn send_names(&self, user: &dyn User) {
        let members = self.state.read().members.clone();
        let names: Vec<String> = members
            .iter()
            .map(|m| {
                let prefix = m.mode_char().map(|c| c.to_string()).unwrap_or_default();
                format!("{}{}", prefix, m.get_user().nickname())
            })
            .collect();

        // Send in chunks to avoid message length limits
        for chunk in names.chunks(20) {
            user.send(&format!(
                "353 {} = {} :{}",
                user.nickname(),
                self.name,
                chunk.join(" ")
            ));
        }
        user.send(&format!("366 {} {} :End of /NAMES list", user.nickname(), self.name));
    }

    fn send_on_join_message(&self, user: &dyn User) {
        if let Some(msg) = &self.props.read().onjoin {
            for line in msg.split("\\n") {
                if !line.is_empty() {
                    user.send(&format!(
                        ":{} PRIVMSG {} :{}",
                        self.name, user.nickname(), line
                    ));
                }
            }
        }
    }

    fn send_on_part_message(&self, user: &dyn User) {
        if let Some(msg) = &self.props.read().onpart {
            for line in msg.split("\\n") {
                if !line.is_empty() {
                    user.send(&format!(
                        ":{} NOTICE {} :{}",
                        self.name, user.nickname(), line
                    ));
                }
            }
        }
    }

    fn allows(&self, user: &dyn User) -> bool {
        !self.has_user(user)
    }

    fn get_access(&self, user: &dyn User, key: Option<&str>, is_goto: bool) -> ChannelAccessResult {
        let host_key_check = self.check_host_key(key);
        let access_level = self.get_channel_access(user);

        let access_result = match access_level {
            AccessLevel::Owner => ChannelAccessResult::SuccessOwner,
            AccessLevel::Host => ChannelAccessResult::SuccessHost,
            AccessLevel::Voice => ChannelAccessResult::SuccessVoice,
            AccessLevel::Grant => ChannelAccessResult::SuccessMember,
            AccessLevel::Deny => ChannelAccessResult::ErrBannedFromChan,
            _ => ChannelAccessResult::None,
        };

        let oper_check = self.check_oper(user);
        let key_check = self.check_member_key(key);
        let invite_check = self.check_invite_only(user);
        let limit_check = self.check_user_limit(is_goto);

        // Return highest priority result
        let checks = [
            oper_check,
            host_key_check,
            key_check,
            invite_check,
            limit_check,
            access_result,
        ];

        checks
            .into_iter()
            .max()
            .unwrap_or(ChannelAccessResult::SuccessGuest)
    }

    fn invite_member(&mut self, user: &dyn User) -> bool {
        let address = user.get_address().address();
        self.state.write().invite_list.insert(address)
    }

    fn update_topic(&mut self, topic: &str) {
        self.props.write().topic = topic.to_string();
        self.state.write().topic_changed = Utc::now().timestamp();
    }

    fn topic(&self) -> &str {
        // This is a bit tricky with RwLock, we'd need to return a guard
        // For simplicity, we'll make this work differently
        ""
    }
}
