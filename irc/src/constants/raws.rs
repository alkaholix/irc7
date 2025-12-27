//! IRC raw message templates

use crate::traits::{Channel, ChannelMember, Server, User};

/// Format a server prefix
pub fn server_prefix(server: &dyn Server) -> String {
    format!(":{}", server.name())
}

/// Format a user prefix
pub fn user_prefix(user: &dyn User) -> String {
    let addr = user.get_address();
    format!(":{}!{}@{}", user.nickname(), addr.user, addr.host)
}

// ============================================================================
// Connection/Registration Messages
// ============================================================================

/// RPL_WELCOME (001)
pub fn rpl_welcome(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 001 {} :Welcome to the {} IRC Network {}",
        server_prefix(server),
        user.nickname(),
        server.title(),
        user.get_address().full_address()
    )
}

/// RPL_YOURHOST (002)
pub fn rpl_yourhost(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 002 {} :Your host is {}, running version {}",
        server_prefix(server),
        user.nickname(),
        server.name(),
        server.server_version()
    )
}

/// RPL_CREATED (003)
pub fn rpl_created(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 003 {} :This server was created {}",
        server_prefix(server),
        user.nickname(),
        server.creation_date().format("%a %b %d %Y at %H:%M:%S %Z")
    )
}

/// RPL_MYINFO (004)
pub fn rpl_myinfo(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 004 {} {} {} {} {}",
        server_prefix(server),
        user.nickname(),
        server.name(),
        server.server_version(),
        server.get_supported_user_modes(),
        server.get_supported_channel_modes()
    )
}

/// RPL_LUSERCLIENT (251)
pub fn rpl_luserclient(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 251 {} :There are {} users and {} invisible on {} servers",
        server_prefix(server),
        user.nickname(),
        server.net_user_count(),
        server.net_invisible_count(),
        server.net_server_count()
    )
}

/// RPL_LUSEROP (252)
pub fn rpl_luserop(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 252 {} {} :operator(s) online",
        server_prefix(server),
        user.nickname(),
        server.sysop_count()
    )
}

/// RPL_LUSERCHANNELS (254)
pub fn rpl_luserchannels(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 254 {} {} :channels formed",
        server_prefix(server),
        user.nickname(),
        server.channel_count()
    )
}

/// RPL_LUSERME (255)
pub fn rpl_luserme(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 255 {} :I have {} clients and {} servers",
        server_prefix(server),
        user.nickname(),
        server.net_user_count(),
        1
    )
}

/// RPL_MOTDSTART (375)
pub fn rpl_motdstart(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 375 {} :- {} Message of the day -",
        server_prefix(server),
        user.nickname(),
        server.name()
    )
}

/// RPL_MOTD (372)
pub fn rpl_motd(server: &dyn Server, user: &dyn User, line: &str) -> String {
    format!(
        "{} 372 {} :- {}",
        server_prefix(server),
        user.nickname(),
        line
    )
}

/// RPL_ENDOFMOTD (376)
pub fn rpl_endofmotd(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 376 {} :End of /MOTD command",
        server_prefix(server),
        user.nickname()
    )
}

// ============================================================================
// Channel Messages
// ============================================================================

/// JOIN message
pub fn rpl_join(user: &dyn User, channel: &dyn Channel) -> String {
    format!("{} JOIN {}", user_prefix(user), channel.name())
}

/// PART message
pub fn rpl_part(user: &dyn User, channel: &dyn Channel) -> String {
    format!("{} PART {}", user_prefix(user), channel.name())
}

/// PART message with reason
pub fn rpl_part_reason(user: &dyn User, channel: &dyn Channel, reason: &str) -> String {
    format!("{} PART {} :{}", user_prefix(user), channel.name(), reason)
}

/// KICK message
pub fn rpl_kick(
    source: &dyn User,
    channel: &dyn Channel,
    target: &dyn User,
    reason: &str,
) -> String {
    format!(
        "{} KICK {} {} :{}",
        user_prefix(source),
        channel.name(),
        target.nickname(),
        reason
    )
}

/// PRIVMSG to channel
pub fn rpl_privmsg_channel(user: &dyn User, channel: &dyn Channel, message: &str) -> String {
    format!(
        "{} PRIVMSG {} :{}",
        user_prefix(user),
        channel.name(),
        message
    )
}

/// PRIVMSG to user
pub fn rpl_privmsg_user(source: &dyn User, target: &dyn User, message: &str) -> String {
    format!(
        "{} PRIVMSG {} :{}",
        user_prefix(source),
        target.nickname(),
        message
    )
}

/// NOTICE to channel
pub fn rpl_notice_channel(user: &dyn User, channel: &dyn Channel, message: &str) -> String {
    format!(
        "{} NOTICE {} :{}",
        user_prefix(user),
        channel.name(),
        message
    )
}

/// NOTICE to user
pub fn rpl_notice_user(source: &dyn User, target: &dyn User, message: &str) -> String {
    format!(
        "{} NOTICE {} :{}",
        user_prefix(source),
        target.nickname(),
        message
    )
}

/// RPL_TOPIC (332)
pub fn rpl_topic(server: &dyn Server, user: &dyn User, channel: &dyn Channel) -> String {
    format!(
        "{} 332 {} {} :{}",
        server_prefix(server),
        user.nickname(),
        channel.name(),
        channel.topic()
    )
}

/// RPL_NOTOPIC (331)
pub fn rpl_notopic(server: &dyn Server, user: &dyn User, channel: &dyn Channel) -> String {
    format!(
        "{} 331 {} {} :No topic is set",
        server_prefix(server),
        user.nickname(),
        channel.name()
    )
}

/// RPL_NAMREPLY (353)
pub fn rpl_namreply(
    server: &dyn Server,
    user: &dyn User,
    channel: &dyn Channel,
    names: &str,
) -> String {
    format!(
        "{} 353 {} = {} :{}",
        server_prefix(server),
        user.nickname(),
        channel.name(),
        names
    )
}

/// RPL_ENDOFNAMES (366)
pub fn rpl_endofnames(server: &dyn Server, user: &dyn User, channel: &dyn Channel) -> String {
    format!(
        "{} 366 {} {} :End of /NAMES list",
        server_prefix(server),
        user.nickname(),
        channel.name()
    )
}

/// NICK change
pub fn rpl_nick(source: &dyn User, new_nick: &str) -> String {
    format!("{} NICK {}", user_prefix(source), new_nick)
}

/// QUIT message
pub fn rpl_quit(user: &dyn User, message: &str) -> String {
    format!("{} QUIT :{}", user_prefix(user), message)
}

/// PING message
pub fn rpl_ping(server: &dyn Server, _user: &dyn User) -> String {
    format!("PING :{}", server.name())
}

/// PONG message
pub fn rpl_pong(server: &dyn Server, token: &str) -> String {
    format!("{} PONG {} :{}", server_prefix(server), server.name(), token)
}

// ============================================================================
// Error Messages
// ============================================================================

/// ERR_NOSUCHNICK (401)
pub fn err_nosuchnick(server: &dyn Server, user: &dyn User, nick: &str) -> String {
    format!(
        "{} 401 {} {} :No such nick/channel",
        server_prefix(server),
        user.nickname(),
        nick
    )
}

/// ERR_NOSUCHCHANNEL (403)
pub fn err_nosuchchannel(server: &dyn Server, user: &dyn User, channel: &str) -> String {
    format!(
        "{} 403 {} {} :No such channel",
        server_prefix(server),
        user.nickname(),
        channel
    )
}

/// ERR_CANNOTSENDTOCHAN (404)
pub fn err_cannotsendtochan(server: &dyn Server, user: &dyn User, channel: &str) -> String {
    format!(
        "{} 404 {} {} :Cannot send to channel",
        server_prefix(server),
        user.nickname(),
        channel
    )
}

/// ERR_UNKNOWNCOMMAND (421)
pub fn err_unknowncommand(server: &dyn Server, user: &dyn User, command: &str) -> String {
    format!(
        "{} 421 {} {} :Unknown command",
        server_prefix(server),
        user.nickname(),
        command
    )
}

/// ERR_NONICKNAMEGIVEN (431)
pub fn err_nonicknamegiven(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 431 {} :No nickname given",
        server_prefix(server),
        user.nickname()
    )
}

/// ERR_ERRONEUSNICKNAME (432)
pub fn err_erroneusnickname(server: &dyn Server, user: &dyn User, nick: &str) -> String {
    format!(
        "{} 432 {} {} :Erroneous nickname",
        server_prefix(server),
        user.nickname(),
        nick
    )
}

/// ERR_NICKNAMEINUSE (433)
pub fn err_nicknameinuse(server: &dyn Server, user: &dyn User, nick: &str) -> String {
    format!(
        "{} 433 {} {} :Nickname is already in use",
        server_prefix(server),
        user.nickname(),
        nick
    )
}

/// ERR_NOTONCHANNEL (442)
pub fn err_notonchannel(server: &dyn Server, user: &dyn User, channel: &str) -> String {
    format!(
        "{} 442 {} {} :You're not on that channel",
        server_prefix(server),
        user.nickname(),
        channel
    )
}

/// ERR_NOTREGISTERED (451)
pub fn err_notregistered(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 451 {} :You have not registered",
        server_prefix(server),
        user.nickname()
    )
}

/// ERR_NEEDMOREPARAMS (461)
pub fn err_needmoreparams(server: &dyn Server, user: &dyn User, command: &str) -> String {
    format!(
        "{} 461 {} {} :Not enough parameters",
        server_prefix(server),
        user.nickname(),
        command
    )
}

/// ERR_ALREADYREGISTERED (462)
pub fn err_alreadyregistered(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 462 {} :You may not reregister",
        server_prefix(server),
        user.nickname()
    )
}

/// ERR_CHANNELISFULL (471)
pub fn err_channelisfull(server: &dyn Server, user: &dyn User, channel: &str) -> String {
    format!(
        "{} 471 {} {} :Cannot join channel (+l)",
        server_prefix(server),
        user.nickname(),
        channel
    )
}

/// ERR_INVITEONLYCHAN (473)
pub fn err_inviteonlychan(server: &dyn Server, user: &dyn User, channel: &str) -> String {
    format!(
        "{} 473 {} {} :Cannot join channel (+i)",
        server_prefix(server),
        user.nickname(),
        channel
    )
}

/// ERR_BANNEDFROMCHAN (474)
pub fn err_bannedfromchan(server: &dyn Server, user: &dyn User, channel: &str) -> String {
    format!(
        "{} 474 {} {} :Cannot join channel (+b)",
        server_prefix(server),
        user.nickname(),
        channel
    )
}

/// ERR_BADCHANNELKEY (475)
pub fn err_badchannelkey(server: &dyn Server, user: &dyn User, channel: &str) -> String {
    format!(
        "{} 475 {} {} :Cannot join channel (+k)",
        server_prefix(server),
        user.nickname(),
        channel
    )
}

/// ERR_NOPRIVILEGES (481)
pub fn err_noprivileges(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 481 {} :Permission Denied- You're not an IRC operator",
        server_prefix(server),
        user.nickname()
    )
}

/// ERR_CHANOPRIVSNEEDED (482)
pub fn err_chanoprivsneeded(server: &dyn Server, user: &dyn User, channel: &str) -> String {
    format!(
        "{} 482 {} {} :You're not channel operator",
        server_prefix(server),
        user.nickname(),
        channel
    )
}

// ============================================================================
// IRCX Extensions
// ============================================================================

/// RPL_YOUREOPER (381)
pub fn rpl_youreoper(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 381 {} :You are now an IRC operator",
        server_prefix(server),
        user.nickname()
    )
}

/// RPL_YOUREADMIN (386) - IRCX
pub fn rpl_youreadmin(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 386 {} :You are now an IRC administrator",
        server_prefix(server),
        user.nickname()
    )
}

/// RPL_YOUREGUIDE (629) - IRCX
pub fn rpl_youreguide(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 629 {} :You are now a guide",
        server_prefix(server),
        user.nickname()
    )
}

/// RPL_NOWAWAY (306)
pub fn rpl_nowaway(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 306 {} :You have been marked as being away",
        server_prefix(server),
        user.nickname()
    )
}

/// RPL_UNAWAY (305)
pub fn rpl_unaway(server: &dyn Server, user: &dyn User) -> String {
    format!(
        "{} 305 {} :You are no longer marked as being away",
        server_prefix(server),
        user.nickname()
    )
}

/// Closing link message
pub fn closing_link(server: &dyn Server, user: &dyn User, reason: &str) -> String {
    format!(
        "ERROR :Closing Link: {}[{}] ({})",
        user.nickname(),
        user.get_address().ip,
        reason
    )
}
