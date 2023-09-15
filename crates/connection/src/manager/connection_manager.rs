use crate::ble::BleConnectionInfo;
use crate::p2p::P2PConnectionInfo;
use crate::tcp::TcpConnectionInfo;

pub struct ConnectionStore {
    connections: Vec<Connection>,
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

impl ConnectionInfo {
    pub fn compare_connection_info(self: &Self, other: &ConnectionInfo) -> bool {
        return match (self, other) {
            (ConnectionInfo::Ble(ble), ConnectionInfo::Ble(other)) => {
                ble.ble_mac().eq(other.ble_mac())
            }
            (ConnectionInfo::Tcp(tcp), ConnectionInfo::Tcp(other)) => false,
            (ConnectionInfo::P2P(p2p), ConnectionInfo::P2P(other)) => false,
            _ => false,
        };
    }
}
