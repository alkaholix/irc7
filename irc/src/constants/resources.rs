//! Resource constants

use chrono::Utc;

/// IRCX channel name regex pattern
pub const IRCX_CHANNEL_REGEX: &str = r"^[%#&][^\x00\x07\x0a\x0d :,]+$";

/// Maximum nickname length
pub const MAX_NICKNAME_LENGTH: usize = 30;

/// Maximum channel name length
pub const MAX_CHANNEL_NAME_LENGTH: usize = 200;

/// Maximum topic length
pub const MAX_TOPIC_LENGTH: usize = 512;

/// Maximum away message length
pub const MAX_AWAY_LENGTH: usize = 160;

/// Maximum kick reason length
pub const MAX_KICK_REASON_LENGTH: usize = 160;

/// User mode characters
pub mod user_modes {
    pub const OPER: char = 'o';
    pub const ADMIN: char = 'a';
    pub const INVISIBLE: char = 'i';
    pub const WALLOPS: char = 'w';
    pub const SECURE: char = 'x';
}

/// Channel mode characters
pub mod channel_modes {
    pub const INVITE_ONLY: char = 'i';
    pub const MODERATED: char = 'm';
    pub const NO_EXTERNAL: char = 'n';
    pub const PRIVATE: char = 'p';
    pub const SECRET: char = 's';
    pub const TOPIC_LOCK: char = 't';
    pub const KEY: char = 'k';
    pub const LIMIT: char = 'l';
    pub const BAN: char = 'b';
}

/// Member mode characters
pub mod member_modes {
    pub const OWNER: char = 'q';
    pub const HOST: char = 'o';
    pub const VOICE: char = 'v';
}

/// Get current epoch time in seconds
pub fn get_epoch_now() -> i64 {
    Utc::now().timestamp()
}

/// Mask an IP address
pub fn mask_ip(ip: &str) -> String {
    if let Some(last_dot) = ip.rfind('.') {
        format!("{}.x", &ip[..last_dot])
    } else {
        ip.to_string()
    }
}
