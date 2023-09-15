use crate::manager::connection_manager::ConnectionInfo;
use crate::manager::{ConnManagerError, ConnectionId};
use crate::tcp::tcp_server::TcpListenerOptions;
use crate::tcp::TcpConnectOptions;

pub enum ConnectOptions {
    Tcp(TcpConnectOptions),
}

pub enum ListenerOptions {
    Tcp(TcpListenerOptions),
}

pub trait ConnectionService {
    fn get_connection_info(connection_id: ConnectionId)
        -> Result<ConnectionInfo, ConnManagerError>;

    /// Connect remote device with [ConnectionId] and [ConnectOptions].
    fn connection_connect_device(connection_id: ConnectionId, options: ConnectOptions);

    /// Disconnect from remote device use [ConnectionId].
    fn connection_disconnect_device(connection_id: ConnectionId);
}

pub trait ListenerService {
    /// Create server when manager init.
    fn connection_server_init();

    fn start_listener(options: ListenerOptions) -> Result<(), ConnManagerError>;
}
