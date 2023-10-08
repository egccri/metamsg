use crate::manager::connection_manager::ConnectionInfo;
use crate::manager::{ConnManagerError, ConnectionId};
use crate::tcp::tcp_server::TcpListenerOptions;
use crate::tcp::TcpConnectOptions;
use bytes::BytesMut;
use tokio::sync::broadcast;

pub enum ConnectOptions {
    Tcp(TcpConnectOptions),
}

pub enum ListenerOptions {
    Tcp(TcpListenerOptions),
}

#[async_trait::async_trait]
pub trait ConnectionService {
    async fn get_connection_info(
        connection_id: ConnectionId,
    ) -> Result<ConnectionInfo, ConnManagerError>;

    async fn connection_send_data(
        connection_id: ConnectionId,
        data: BytesMut,
    ) -> Result<(), ConnManagerError>;

    /// Connect remote device with [ConnectionId] and [ConnectOptions].
    async fn connection_connect_device(connection_id: ConnectionId, options: ConnectOptions);

    /// Disconnect from remote device use [ConnectionId].
    async fn connection_disconnect_device(connection_id: ConnectionId);
}

#[async_trait::async_trait]
pub trait ListenerService {
    /// Create server when manager init.
    async fn connection_server_init();

    // 传入一个server_conn
    async fn start_listener(options: ListenerOptions, shutdown_signal: broadcast::Receiver<()>);

    async fn shutdown_listener() -> Result<(), ConnManagerError>;
}
