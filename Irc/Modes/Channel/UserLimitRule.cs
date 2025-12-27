using Irc.Constants;
using Irc.Enumerations;
using Irc.Interfaces;
using Irc.Objects.Channel;

namespace Irc.Modes.Channel;

public class UserLimitRule : ModeRuleChannel, IModeRule
{
    public UserLimitRule() : base(Resources.ChannelModeUserLimit, true)
    {
    }

    public new EnumIrcError Evaluate(IChatObject source, IChatObject target, bool flag, string parameter)
    {
        var result = base.Evaluate(source, target, flag, parameter);
        if (result != EnumIrcError.OK) return result;

        var user = (IUser)source;
        var channel = (IChannel)target;
        var isAdministrator = user.IsAdministrator();
        var channelModes = (ChannelModes)channel.Modes;

        if (flag == false)
        {
            // Allow any user with proper channel permissions to unset the limit
            channelModes.UserLimit.Value = 0;
            DispatchModeChange(source, target, false, string.Empty);
            return EnumIrcError.OK;
        }

        if (!int.TryParse(parameter, out var limit)) return EnumIrcError.ERR_NEEDMOREPARAMS;

        // Limit must be positive
        if (limit <= 0) return EnumIrcError.ERR_BADVALUE;

        // Non-administrators can only set limits up to 100
        if (limit > 100 && !isAdministrator) return EnumIrcError.ERR_BADVALUE;

        channelModes.UserLimit.Value = limit;
        DispatchModeChange(source, target, true, limit.ToString());

        return EnumIrcError.OK;
    }
}