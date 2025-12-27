//! Command-related enumerations

use serde::{Deserialize, Serialize};

/// Command data type requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandDataType {
    /// No special data requirements
    None,
    /// Standard IRC command
    Standard,
    /// Extended IRC command
    Extended,
    /// Data contains raw bytes
    Raw,
}

impl Default for CommandDataType {
    fn default() -> Self {
        Self::None
    }
}

/// IRC error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum IrcError {
    /// No error
    Ok = 0,
    /// No such nick
    ErrNoSuchNick = 401,
    /// No such server
    ErrNoSuchServer = 402,
    /// No such channel
    ErrNoSuchChannel = 403,
    /// Cannot send to channel
    ErrCannotSendToChan = 404,
    /// Too many channels
    ErrTooManyChannels = 405,
    /// Was no such nick
    ErrWasNoSuchNick = 406,
    /// Too many targets
    ErrTooManyTargets = 407,
    /// No origin
    ErrNoOrigin = 409,
    /// No recipient
    ErrNoRecipient = 411,
    /// No text to send
    ErrNoTextToSend = 412,
    /// No top level
    ErrNoTopLevel = 413,
    /// Wild top level
    ErrWildTopLevel = 414,
    /// Unknown command
    ErrUnknownCommand = 421,
    /// No MOTD
    ErrNoMotd = 422,
    /// No admin info
    ErrNoAdminInfo = 423,
    /// File error
    ErrFileError = 424,
    /// No nickname given
    ErrNoNicknameGiven = 431,
    /// Erroneous nickname
    ErrErroneousNickname = 432,
    /// Nickname in use
    ErrNicknameInUse = 433,
    /// Nick collision
    ErrNickCollision = 436,
    /// User not in channel
    ErrUserNotInChannel = 441,
    /// Not on channel
    ErrNotOnChannel = 442,
    /// User on channel
    ErrUserOnChannel = 443,
    /// No login
    ErrNoLogin = 444,
    /// Summon disabled
    ErrSummonDisabled = 445,
    /// Users disabled
    ErrUsersDisabled = 446,
    /// Not registered
    ErrNotRegistered = 451,
    /// Need more params
    ErrNeedMoreParams = 461,
    /// Already registered
    ErrAlreadyRegistered = 462,
    /// No permission for host
    ErrNoPermForHost = 463,
    /// Password mismatch
    ErrPasswdMismatch = 464,
    /// You're banned creep
    ErrYoureBannedCreep = 465,
    /// Key set
    ErrKeySet = 467,
    /// Channel is full
    ErrChannelIsFull = 471,
    /// Unknown mode
    ErrUnknownMode = 472,
    /// Invite only channel
    ErrInviteOnlyChan = 473,
    /// Banned from channel
    ErrBannedFromChan = 474,
    /// Bad channel key
    ErrBadChannelKey = 475,
    /// No privileges
    ErrNoPrivileges = 481,
    /// Channel op privs needed
    ErrChanOpPrivsNeeded = 482,
    /// Cannot kill server
    ErrCannotKillServer = 483,
    /// Channel owner privs needed (IRCX)
    ErrChanQPrivsNeeded = 485,
    /// No oper host
    ErrNoOperHost = 491,
    /// Unknown mode flag
    ErrUnknownModeFlag = 501,
    /// Users don't match
    ErrUsersDontMatch = 502,
    /// No IRC op
    ErrNoIrcOp = 550,
    /// No permissions
    ErrNoPerms = 908,
    /// Cannot set for other
    ErrCannotSetForOther = 909,
}

impl Default for IrcError {
    fn default() -> Self {
        Self::Ok
    }
}
