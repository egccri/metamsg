use crate::manager::ConnectionId;
use std::net::SocketAddr;

mod tcp_connection_tx;
pub mod tcp_server;

pub struct TcpConnectOptions {}

#[derive(Debug, Copy, Clone)]
pub struct TcpConnectionInfo {
    conn_id: ConnectionId,
    remote_addr: SocketAddr,
}
