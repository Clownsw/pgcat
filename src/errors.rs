#[derive(Debug, PartialEq)]
pub enum Error {
    SocketError,
    // ClientDisconnected,
    ClientBadStartup,
    ProtocolSyncError,
    ServerError,
    // ServerTimeout,
    // DirtyServer,
    BadConfig,
    BadUserList,
    AllServersDown,
    AuthenticationError
}
