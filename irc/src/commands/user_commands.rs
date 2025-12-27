//! User-related commands (WHOIS, WHO, AWAY, MODE, INVITE)

use async_trait::async_trait;

use crate::constants::raws;
use crate::enums::CommandDataType;
use crate::traits::{ChatFrame, Command};

/// WHOIS command - Get information about a user
pub struct WhoisCommand;

#[async_trait]
impl Command for WhoisCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "WHOIS"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() {
            user.send(&raws::err_nonicknamegiven(&*server, &*user));
            return;
        }

        let target_nick = &params[0];

        match server.get_user_by_nickname(target_nick) {
            Some(target) => {
                // 311 - RPL_WHOISUSER
                user.send(&format!(
                    ":{} 311 {} {} {} {} * :{}",
                    server.name(),
                    user.nickname(),
                    target.nickname(),
                    target.get_address().user,
                    target.get_address().host,
                    target.name()
                ));

                // 312 - RPL_WHOISSERVER
                user.send(&format!(
                    ":{} 312 {} {} {} :{}",
                    server.name(),
                    user.nickname(),
                    target.nickname(),
                    server.name(),
                    server.title()
                ));

                // 317 - RPL_WHOISIDLE
                let idle_secs = (chrono::Utc::now() - target.last_idle()).num_seconds();
                user.send(&format!(
                    ":{} 317 {} {} {} {} :seconds idle, signon time",
                    server.name(),
                    user.nickname(),
                    target.nickname(),
                    idle_secs,
                    target.logged_on().timestamp()
                ));

                // 319 - RPL_WHOISCHANNELS
                let channels: Vec<String> = target
                    .get_channels()
                    .values()
                    .map(|(ch, m)| {
                        let prefix = m.mode_char().map(|c| c.to_string()).unwrap_or_default();
                        format!("{}{}", prefix, ch.name())
                    })
                    .collect();

                if !channels.is_empty() {
                    user.send(&format!(
                        ":{} 319 {} {} :{}",
                        server.name(),
                        user.nickname(),
                        target.nickname(),
                        channels.join(" ")
                    ));
                }

                // 318 - RPL_ENDOFWHOIS
                user.send(&format!(
                    ":{} 318 {} {} :End of /WHOIS list",
                    server.name(),
                    user.nickname(),
                    target.nickname()
                ));
            }
            None => {
                user.send(&raws::err_nosuchnick(&*server, &*user, target_nick));
            }
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

/// WHO command - List users matching a mask
pub struct WhoCommand;

#[async_trait]
impl Command for WhoCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "WHO"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        let mask = params.first().map(|s| s.as_str()).unwrap_or("*");

        // Check if mask is a channel
        if mask.starts_with('#') || mask.starts_with('&') || mask.starts_with('%') {
            if let Some(channel) = server.get_channel_by_name(mask) {
                for member in channel.get_members() {
                    let target = member.get_user();
                    let flags = format!(
                        "H{}",
                        member.mode_char().map(|c| c.to_string()).unwrap_or_default()
                    );

                    user.send(&format!(
                        ":{} 352 {} {} {} {} {} {} {} :0 {}",
                        server.name(),
                        user.nickname(),
                        channel.name(),
                        target.get_address().user,
                        target.get_address().host,
                        server.name(),
                        target.nickname(),
                        flags,
                        target.name()
                    ));
                }
            }
        } else {
            // List all visible users matching mask
            for target in server.get_users() {
                if mask == "*" || target.nickname().to_lowercase().contains(&mask.to_lowercase()) {
                    let flags = if target.is_away() { "G" } else { "H" };
                    user.send(&format!(
                        ":{} 352 {} * {} {} {} {} {} :0 {}",
                        server.name(),
                        user.nickname(),
                        target.get_address().user,
                        target.get_address().host,
                        server.name(),
                        target.nickname(),
                        flags,
                        target.name()
                    ));
                }
            }
        }

        user.send(&format!(
            ":{} 315 {} {} :End of /WHO list",
            server.name(),
            user.nickname(),
            mask
        ));
    }

    fn parameters_are_valid(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }
}

/// AWAY command - Set/unset away message
pub struct AwayCommand;

#[async_trait]
impl Command for AwayCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "AWAY"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() || params[0].is_empty() {
            // Unset away
            user.send(&raws::rpl_unaway(&*server, &*user));
        } else {
            // Set away with message
            user.send(&raws::rpl_nowaway(&*server, &*user));
        }
    }

    fn parameters_are_valid(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }
}

/// INVITE command - Invite a user to a channel
pub struct InviteCommand;

#[async_trait]
impl Command for InviteCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "INVITE"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.len() < 2 {
            user.send(&raws::err_needmoreparams(&*server, &*user, "INVITE"));
            return;
        }

        let target_nick = &params[0];
        let channel_name = &params[1];

        // Find target user
        let target = match server.get_user_by_nickname(target_nick) {
            Some(u) => u,
            None => {
                user.send(&raws::err_nosuchnick(&*server, &*user, target_nick));
                return;
            }
        };

        // Find channel
        let channel = match server.get_channel_by_name(channel_name) {
            Some(ch) => ch,
            None => {
                user.send(&raws::err_nosuchchannel(&*server, &*user, channel_name));
                return;
            }
        };

        // Check if source is on channel
        let source_member = match channel.get_member(&*user) {
            Some(m) => m,
            None => {
                user.send(&raws::err_notonchannel(&*server, &*user, channel_name));
                return;
            }
        };

        // Check if source has permission to invite
        if !source_member.is_operator() && !source_member.is_owner() {
            user.send(&raws::err_chanoprivsneeded(&*server, &*user, channel_name));
            return;
        }

        // Send invite
        // channel.invite_member(&*target);

        // Notify target
        target.send(&format!(
            ":{}!{}@{} INVITE {} :{}",
            user.nickname(),
            user.get_address().user,
            user.get_address().host,
            target.nickname(),
            channel_name
        ));

        // Confirm to source
        user.send(&format!(
            ":{} 341 {} {} {}",
            server.name(),
            user.nickname(),
            target.nickname(),
            channel_name
        ));
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

/// MODE command - Get or set modes
pub struct ModeCommand;

#[async_trait]
impl Command for ModeCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "MODE"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() {
            user.send(&raws::err_needmoreparams(&*server, &*user, "MODE"));
            return;
        }

        let target = &params[0];

        // Check if target is a channel
        if target.starts_with('#') || target.starts_with('&') || target.starts_with('%') {
            // Channel mode
            let _channel = match server.get_channel_by_name(target) {
                Some(ch) => ch,
                None => {
                    user.send(&raws::err_nosuchchannel(&*server, &*user, target));
                    return;
                }
            };

            if params.len() == 1 {
                // Get modes
                // Send current channel modes
            } else {
                // Set modes
                // Parse and apply mode changes
            }
        } else {
            // User mode
            if target.to_lowercase() != user.nickname().to_lowercase() {
                // Can't change other user's modes
                return;
            }

            if params.len() == 1 {
                // Get modes
                // Send current user modes
            } else {
                // Set modes
                // Parse and apply mode changes
            }
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
