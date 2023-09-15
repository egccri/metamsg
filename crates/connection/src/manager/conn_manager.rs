use crate::ble::ble_server::{BleListenerCallback, BleConnectionInfo};
use crate::p2p::p2p_server::{P2PConnectionInfo, P2PListenerCallback};
use crate::tcp::tcp_server::{TcpConnectionInfo, TcpListenerCallback};

pub struct Session {
    connections: Vec<Connection>
}

pub struct Connection {
    is_available: bool,
    is_server: bool,
    info: ConnectionInfo,
}

pub enum ConnectionInfo {
    Ble(BleConnectionInfo),
    Tcp(TcpConnectionInfo),
    P2P(P2PConnectionInfo),
}

pub enum ListenerCallbacks {
    Ble(BleListenerCallback),
    Tcp(TcpListenerCallback),
    P2P(P2PListenerCallback),
}

// manager里只使用方法，不使用Listener结构体，将Listener具体实现放到不同协议里
pub trait ListenerCallback {
    fn on_connected();
    fn on_disconnected();
    fn on_data_received();
}