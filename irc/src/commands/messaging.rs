//! Messaging commands (PRIVMSG, NOTICE, WHISPER)

use async_trait::async_trait;

use crate::constants::raws;
use crate::enums::CommandDataType;
use crate::traits::{ChatFrame, Command};

/// PRIVMSG command - Send a message to user or channel
pub struct PrivmsgCommand;

#[async_trait]
impl Command for PrivmsgCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "PRIVMSG"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.len() < 2 {
            user.send(&raws::err_needmoreparams(&*server, &*user, "PRIVMSG"));
            return;
        }

        let target = &params[0];
        let message = &params[1];

        // Check if target is a channel
        if target.starts_with('#') || target.starts_with('&') || target.starts_with('%') {
            // Send to channel
            match server.get_channel_by_name(target) {
                Some(channel) => {
                    if !channel.has_user(&*user) {
                        user.send(&raws::err_cannotsendtochan(&*server, &*user, target));
                        return;
                    }
                    channel.send_message(&*user, message);
                }
                None => {
                    user.send(&raws::err_nosuchchannel(&*server, &*user, target));
                }
            }
        } else {
            // Send to user
            match server.get_user_by_nickname(target) {
                Some(target_user) => {
                    let msg = raws::rpl_privmsg_user(&*user, &*target_user, message);
                    target_user.send(&msg);
                }
                None => {
                    user.send(&raws::err_nosuchnick(&*server, &*user, target));
                }
            }
        }
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

/// NOTICE command - Send a notice (no auto-reply)
pub struct NoticeCommand;

#[async_trait]
impl Command for NoticeCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "NOTICE"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.len() < 2 {
            // NOTICE doesn't send error replies
            return;
        }

        let target = &params[0];
        let message = &params[1];

        if target.starts_with('#') || target.starts_with('&') || target.starts_with('%') {
            if let Some(channel) = server.get_channel_by_name(target) {
                if channel.has_user(&*user) {
                    channel.send_notice(&*user, message);
                }
            }
        } else if let Some(target_user) = server.get_user_by_nickname(target) {
            let msg = raws::rpl_notice_user(&*user, &*target_user, message);
            target_user.send(&msg);
        }
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

/// WHISPER command (IRCX) - Send a private message within a channel context
pub struct WhisperCommand;

#[async_trait]
impl Command for WhisperCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Extended
    }

    fn name(&self) -> &str {
        "WHISPER"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.len() < 3 {
            user.send(&raws::err_needmoreparams(&*server, &*user, "WHISPER"));
            return;
        }

        let channel_name = &params[0];
        let target_nick = &params[1];
        let message = &params[2];

        // Find channel
        let channel = match server.get_channel_by_name(channel_name) {
            Some(ch) => ch,
            None => {
                user.send(&raws::err_nosuchchannel(&*server, &*user, channel_name));
                return;
            }
        };

        // Check if source is on channel
        if !channel.has_user(&*user) {
            user.send(&raws::err_notonchannel(&*server, &*user, channel_name));
            return;
        }

        // Find target
        match channel.get_member_by_nickname(target_nick) {
            Some(target_member) => {
                let target = target_member.get_user();
                // Format: :nick!user@host WHISPER #channel target :message
                let msg = format!(
                    ":{}!{}@{} WHISPER {} {} :{}",
                    user.nickname(),
                    user.get_address().user,
                    user.get_address().host,
                    channel_name,
                    target.nickname(),
                    message
                );
                target.send(&msg);
            }
            None => {
                user.send(&raws::err_nosuchnick(&*server, &*user, target_nick));
            }
        }
    }

    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool {
        frame.chat_message().parameters().len() >= 3
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn min_parameters(&self) -> usize {
        3
    }
}
