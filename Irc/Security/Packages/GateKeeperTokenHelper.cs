using System.Runtime.InteropServices;
using NLog;

public class GateKeeperTokenHelper
{
    private static readonly Logger Log = LogManager.GetCurrentClassLogger();

    public static GateKeeperToken InitializeFromBytes(byte[] Data)
    {
        var AuthToken = new GateKeeperToken();
        var pBuf = Marshal.AllocHGlobal(Marshal.SizeOf(AuthToken));
        try
        {
            Marshal.Copy(Data, 0, pBuf, Marshal.SizeOf(AuthToken));
            AuthToken = Marshal.PtrToStructure<GateKeeperToken>(pBuf);
        }
        catch (Exception ex)
        {
            Log.Error(ex, "GateKeeperTokenHelper: Failed to deserialize GateKeeperToken from bytes. Data length: {0}", Data?.Length ?? 0);
            throw;
        }
        finally
        {
            Marshal.FreeHGlobal(pBuf);
        }

        return AuthToken;
    }
}