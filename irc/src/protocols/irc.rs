//! Standard IRC protocol (RFC 1459)

use std::sync::Arc;

use crate::commands::*;
use crate::enums::ProtocolType;
use crate::protocols::ProtocolBase;
use crate::traits::Protocol;

/// Create a standard IRC protocol with all commands
pub fn create_irc_protocol() -> Box<dyn Protocol> {
    let mut protocol = ProtocolBase::new(ProtocolType::Irc);

    // Connection commands
    protocol.add_command(Arc::new(NickCommand));
    protocol.add_command(Arc::new(UserCommand));
    protocol.add_command(Arc::new(PassCommand));
    protocol.add_command(Arc::new(QuitCommand));
    protocol.add_command(Arc::new(PingCommand));
    protocol.add_command(Arc::new(PongCommand));

    // Channel commands
    protocol.add_command(Arc::new(JoinCommand));
    protocol.add_command(Arc::new(PartCommand));
    protocol.add_command(Arc::new(KickCommand));
    protocol.add_command(Arc::new(TopicCommand));
    protocol.add_command(Arc::new(NamesCommand));
    protocol.add_command(Arc::new(ListCommand));
    protocol.add_command(Arc::new(ModeCommand));

    // Messaging commands
    protocol.add_command(Arc::new(PrivmsgCommand));
    protocol.add_command(Arc::new(NoticeCommand));

    // User commands
    protocol.add_command(Arc::new(WhoisCommand));
    protocol.add_command(Arc::new(WhoCommand));
    protocol.add_command(Arc::new(AwayCommand));
    protocol.add_command(Arc::new(InviteCommand));

    Box::new(protocol)
}
