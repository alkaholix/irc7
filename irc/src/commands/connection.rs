//! Connection-related commands (NICK, USER, PASS, QUIT, PING, PONG)

use async_trait::async_trait;
use std::sync::Arc;

use crate::constants::raws;
use crate::enums::CommandDataType;
use crate::traits::{ChatFrame, Command};

/// NICK command - Set or change nickname
pub struct NickCommand;

#[async_trait]
impl Command for NickCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "NICK"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() {
            user.send(&raws::err_nonicknamegiven(&*server, &*user));
            return;
        }

        let new_nick = &params[0];

        // Validate nickname
        if !is_valid_nickname(new_nick) {
            user.send(&raws::err_erroneusnickname(&*server, &*user, new_nick));
            return;
        }

        // Check if nickname is in use
        if server.get_user_by_nickname(new_nick).is_some() {
            user.send(&raws::err_nicknameinuse(&*server, &*user, new_nick));
            return;
        }

        // Change nickname
        // user.change_nickname(new_nick, false);
    }

    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool {
        !frame.chat_message().parameters().is_empty()
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        false
    }

    fn min_parameters(&self) -> usize {
        1
    }
}

/// USER command - Set username and realname
pub struct UserCommand;

#[async_trait]
impl Command for UserCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "USER"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.len() < 4 {
            user.send(&raws::err_needmoreparams(&*server, &*user, "USER"));
            return;
        }

        if user.is_registered() {
            user.send(&raws::err_alreadyregistered(&*server, &*user));
            return;
        }

        // Set user info
        // let username = &params[0];
        // let realname = &params[3];
        // user.set_username(username);
        // user.set_realname(realname);
    }

    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool {
        frame.chat_message().parameters().len() >= 4
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        false
    }

    fn min_parameters(&self) -> usize {
        4
    }
}

/// PASS command - Set connection password
pub struct PassCommand;

#[async_trait]
impl Command for PassCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "PASS"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        if params.is_empty() {
            user.send(&raws::err_needmoreparams(&*server, &*user, "PASS"));
            return;
        }

        if user.is_registered() {
            user.send(&raws::err_alreadyregistered(&*server, &*user));
            return;
        }

        // Store password for later authentication
        // user.set_pass(&params[0]);
    }

    fn parameters_are_valid(&self, frame: &dyn ChatFrame) -> bool {
        !frame.chat_message().parameters().is_empty()
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        false
    }

    fn min_parameters(&self) -> usize {
        1
    }
}

/// QUIT command - Disconnect from server
pub struct QuitCommand;

#[async_trait]
impl Command for QuitCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "QUIT"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let user = frame.user();

        let message = params.first().map(|s| s.as_str()).unwrap_or("Quit");

        // Broadcast quit to all channels
        let quit_msg = raws::rpl_quit(&*user, message);
        user.broadcast_to_channels(&quit_msg, true);

        // Disconnect
        // user.disconnect(message);
    }

    fn parameters_are_valid(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        false
    }
}

/// PING command - Test connection
pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "PING"
    }

    async fn execute(&self, frame: &dyn ChatFrame) {
        let params = frame.chat_message().parameters();
        let server = frame.server();
        let user = frame.user();

        let token = params.first().map(|s| s.as_str()).unwrap_or("");
        user.send(&raws::rpl_pong(&*server, token));
    }

    fn parameters_are_valid(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        false
    }
}

/// PONG command - Response to PING
pub struct PongCommand;

#[async_trait]
impl Command for PongCommand {
    fn data_type(&self) -> CommandDataType {
        CommandDataType::Standard
    }

    fn name(&self) -> &str {
        "PONG"
    }

    async fn execute(&self, _frame: &dyn ChatFrame) {
        // PONG is just acknowledged, no response needed
        // Updates last ping time in user object
    }

    fn parameters_are_valid(&self, _frame: &dyn ChatFrame) -> bool {
        true
    }

    fn registration_needed(&self, _frame: &dyn ChatFrame) -> bool {
        false
    }
}

/// Validate a nickname
fn is_valid_nickname(nick: &str) -> bool {
    if nick.is_empty() || nick.len() > 30 {
        return false;
    }

    let first = nick.chars().next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' && first != '\'' {
        return false;
    }

    nick.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '\'' || c == '`' || c == '^'
    })
}
