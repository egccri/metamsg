use crate::manager::common_listener::ListenerCallback;
use std::net::IpAddr;

pub struct TcpListenerOptions {
    ip_addr: IpAddr,
    port: u16,
}

pub struct TcpServer {}

pub struct TcpListenerCallback {}

impl ListenerCallback for TcpListenerCallback {
    fn on_connected() {
        todo!()
    }

    fn on_disconnected() {
        todo!()
    }

    fn on_data_received() {
        todo!()
    }
}
