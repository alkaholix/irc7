using Irc.Constants;
using Irc.Enumerations;
using Irc.Interfaces;

namespace Irc.Commands;

internal class Kill : Command, ICommand
{
    public Kill() : base(2)
    {
    }

    public new EnumCommandDataType GetDataType()
    {
        return EnumCommandDataType.Standard;
    }

    public new void Execute(IChatFrame chatFrame)
    {
        var server = chatFrame.Server;
        var user = chatFrame.User;
        var target = chatFrame.ChatMessage.Parameters.First();
        var reason = chatFrame.ChatMessage.Parameters[1];

        if (user.GetLevel() < EnumUserAccessLevel.Sysop)
        {
            chatFrame.User.Send(Raws.IRCX_ERR_SECURITY_908(server, user));
            return;
        }

        var targetUser = server.GetUserByNickname(target);

        if (targetUser == null)
        {
            chatFrame.User.Send(Raws.IRCX_ERR_NOSUCHNICK_401(server, user, target));
            return;
        }

        if (targetUser.GetLevel() > user.GetLevel())
        {
            chatFrame.User.Send(Raws.IRCX_ERR_SECURITY_908(server, user));
            return;
        }

        // Notify all channels the target user is in about the kill
        var targetChannels = targetUser.GetChannels();
        foreach (var channelKvp in targetChannels)
        {
            var channel = channelKvp.Key;
            channel.Send(Raws.RPL_KILL_IRC(user, targetUser, reason));
        }

        targetUser.Disconnect(
            Raws.IRCX_CLOSINGLINK_007_SYSTEMKILL(server, targetUser, targetUser.GetAddress().RemoteIp));
    }
}