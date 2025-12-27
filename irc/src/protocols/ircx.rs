//! IRCX protocol (Microsoft extension)

use std::sync::Arc;

use crate::commands::*;
use crate::enums::ProtocolType;
use crate::protocols::ProtocolBase;
use crate::traits::Protocol;

/// Create an IRCX protocol with all commands including extensions
pub fn create_ircx_protocol() -> Box<dyn Protocol> {
    let mut protocol = ProtocolBase::new(ProtocolType::IrcX);

    // All IRC commands
    protocol.add_command(Arc::new(NickCommand));
    protocol.add_command(Arc::new(UserCommand));
    protocol.add_command(Arc::new(PassCommand));
    protocol.add_command(Arc::new(QuitCommand));
    protocol.add_command(Arc::new(PingCommand));
    protocol.add_command(Arc::new(PongCommand));
    protocol.add_command(Arc::new(JoinCommand));
    protocol.add_command(Arc::new(PartCommand));
    protocol.add_command(Arc::new(KickCommand));
    protocol.add_command(Arc::new(TopicCommand));
    protocol.add_command(Arc::new(NamesCommand));
    protocol.add_command(Arc::new(ListCommand));
    protocol.add_command(Arc::new(ModeCommand));
    protocol.add_command(Arc::new(PrivmsgCommand));
    protocol.add_command(Arc::new(NoticeCommand));
    protocol.add_command(Arc::new(WhoisCommand));
    protocol.add_command(Arc::new(WhoCommand));
    protocol.add_command(Arc::new(AwayCommand));
    protocol.add_command(Arc::new(InviteCommand));

    // IRCX extensions
    protocol.add_command(Arc::new(WhisperCommand));
    // Additional IRCX commands would be added here:
    // AUTH, IRCX, ISIRCX, ACCESS, PROP, CREATE, LISTX, etc.

    Box::new(protocol)
}
