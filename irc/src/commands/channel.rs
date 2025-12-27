//! Channel-related commands (JOIN, PART, KICK, TOPIC, MODE, NAMES, LIST)

use async_trait::async_trait;

use crate::constants::raws;
use crate::enums::{ChannelAccessResult, CommandDataType};
use crate::objects::Channel;
use crate::traits::{ChatFrame, Command};

/// JOIN command - Join a channel
pub struct JoinCommand;

#[async_trait]
impl Command for JoinCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "JOIN"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() {
            user.send(&raws::err_needmoreparams(&*server, &*user, "JOIN"));
            return;
        }

        let channel_name = &params[0];
        let key = params.get(1).map(|s| s.as_str());

        // Validate channel name
        if !Channel::valid_name(channel_name) {
            user.send(&raws::err_nosuchchannel(&*server, &*user, channel_name));
            return;
        }

        // Get or create channel
        let channel = match server.get_channel_by_name(channel_name) {
            Some(ch) => ch,
            None => {
                // Create new channel
                server.create_channel(channel_name)
            }
        };

        // Check access
        let access = channel.get_access(&*user, key, false);
        if access.is_error() {
            match access {
                ChannelAccessResult::ErrChannelIsFull => {
                    user.send(&raws::err_channelisfull(&*server, &*user, channel_name));
                }
                ChannelAccessResult::ErrInviteOnlyChan => {
                    user.send(&raws::err_inviteonlychan(&*server, &*user, channel_name));
                }
                ChannelAccessResult::ErrBadChannelKey => {
                    user.send(&raws::err_badchannelkey(&*server, &*user, channel_name));
                }
                ChannelAccessResult::ErrBannedFromChan => {
                    user.send(&raws::err_bannedfromchan(&*server, &*user, channel_name));
                }
                _ => {}
            }
            return;
        }

        // Join the channel
        // channel.join(user.clone(), access);
        // channel.send_topic(&*user);
        // channel.send_names(&*user);
    }

    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool {
        !frame.chat_message().parameters().is_empty()
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn min_parameters(&self) -> usize {
        1
    }
}

/// PART command - Leave a channel
pub struct PartCommand;

#[async_trait]
impl Command for PartCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "PART"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() {
            user.send(&raws::err_needmoreparams(&*server, &*user, "PART"));
            return;
        }

        let channel_name = &params[0];
        let _reason = params.get(1).map(|s| s.as_str()).unwrap_or("");

        let channel = match server.get_channel_by_name(channel_name) {
            Some(ch) => ch,
            None => {
                user.send(&raws::err_nosuchchannel(&*server, &*user, channel_name));
                return;
            }
        };

        if !channel.has_user(&*user) {
            user.send(&raws::err_notonchannel(&*server, &*user, channel_name));
            return;
        }

        // Part the channel
        // channel.part(&*user);
    }

    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool {
        !frame.chat_message().parameters().is_empty()
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn min_parameters(&self) -> usize {
        1
    }
}

/// KICK command - Kick a user from a channel
pub struct KickCommand;

#[async_trait]
impl Command for KickCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "KICK"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.len() < 2 {
            user.send(&raws::err_needmoreparams(&*server, &*user, "KICK"));
            return;
        }

        let channel_name = &params[0];
        let target_nick = &params[1];
        let _reason = params.get(2).map(|s| s.as_str()).unwrap_or(user.nickname());

        let channel = match server.get_channel_by_name(channel_name) {
            Some(ch) => ch,
            None => {
                user.send(&raws::err_nosuchchannel(&*server, &*user, channel_name));
                return;
            }
        };

        // Check if source is on channel and has permission
        let source_member = match channel.get_member(&*user) {
            Some(m) => m,
            None => {
                user.send(&raws::err_notonchannel(&*server, &*user, channel_name));
                return;
            }
        };

        if !source_member.is_operator() && !source_member.is_owner() {
            user.send(&raws::err_chanoprivsneeded(&*server, &*user, channel_name));
            return;
        }

        // Find target
        let _target = match channel.get_member_by_nickname(target_nick) {
            Some(m) => m,
            None => {
                user.send(&raws::err_nosuchnick(&*server, &*user, target_nick));
                return;
            }
        };

        // Kick the user
        // channel.kick(&*user, &*target.get_user(), reason);
    }

    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool {
        frame.chat_message().parameters().len() >= 2
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn min_parameters(&self) -> usize {
        2
    }
}

/// TOPIC command - Get or set channel topic
pub struct TopicCommand;

#[async_trait]
impl Command for TopicCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "TOPIC"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() {
            user.send(&raws::err_needmoreparams(&*server, &*user, "TOPIC"));
            return;
        }

        let channel_name = &params[0];

        let channel = match server.get_channel_by_name(channel_name) {
            Some(ch) => ch,
            None => {
                user.send(&raws::err_nosuchchannel(&*server, &*user, channel_name));
                return;
            }
        };

        if params.len() == 1 {
            // Get topic
            channel.send_topic(&*user);
        } else {
            // Set topic
            let member = match channel.get_member(&*user) {
                Some(m) => m,
                None => {
                    user.send(&raws::err_notonchannel(&*server, &*user, channel_name));
                    return;
                }
            };

            // Check if topic is locked and user has permission
            if !member.is_operator() && !member.is_owner() {
                // Check if topic lock mode is set
                user.send(&raws::err_chanoprivsneeded(&*server, &*user, channel_name));
                return;
            }

            // Set topic
            // let new_topic = &params[1];
            // channel.update_topic(new_topic);
            // channel.send_topic_all();
        }
    }

    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool {
        !frame.chat_message().parameters().is_empty()
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn min_parameters(&self) -> usize {
        1
    }
}

/// NAMES command - List users in a channel
pub struct NamesCommand;

#[async_trait]
impl Command for NamesCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "NAMES"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() {
            // List all visible channels
            for channel in server.get_channels() {
                channel.send_names(&*user);
            }
            return;
        }

        let channel_name = &params[0];
        match server.get_channel_by_name(channel_name) {
            Some(channel) => channel.send_names(&*user),
            None => {
                user.send(&raws::err_nosuchchannel(&*server, &*user, channel_name));
            }
        }
    }

    fn parameters_are_valid(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }
}

/// LIST command - List channels
pub struct ListCommand;

#[async_trait]
impl Command for ListCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "LIST"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let server = frame.server();
        let user = frame.user();

        // Send channel list header
        user.send(&format!(
            ":{} 321 {} Channel :Users  Name",
            server.name(),
            user.nickname()
        ));

        // List channels
        for channel in server.get_channels() {
            user.send(&format!(
                ":{} 322 {} {} {} :{}",
                server.name(),
                user.nickname(),
                channel.name(),
                channel.member_count(),
                channel.topic()
            ));
        }

        // End of list
        user.send(&format!(
            ":{} 323 {} :End of /LIST",
            server.name(),
            user.nickname()
        ));
    }

    fn parameters_are_valid(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }
}
