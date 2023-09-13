use crate::ble::ble_server::{BleListenerCallback, BleConnectionInfo};
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
}

pub enum ListenerCallbacks {
    Ble(BleListenerCallback),
    Tcp(TcpListenerCallback),
}

// manager里只使用方法，不使用Listener结构体，将Listener具体实现放到不同协议里
pub trait ListenerCallback {
    fn on_connected();
    fn on_disconnected();
    fn on_data_received();
}