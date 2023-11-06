use crate::ble::ble_server::BleListenerCallback;
use crate::p2p::p2p_server::P2PListenerCallback;
use crate::tcp::tcp_server::TcpListenerCallback;
use bytes::BytesMut;

pub enum ListenerModule {
    Proxy,
    Auth,
    Direct,
}

pub enum ListenerOption {}

pub enum ListenerCallbacks {
    Ble(BleListenerCallback),
    Tcp(TcpListenerCallback),
    P2P(P2PListenerCallback),
}

// manager里只使用方法，不使用Listener结构体，将Listener具体实现放到不同协议里
#[async_trait::async_trait]
pub trait ListenerCallback {
    async fn on_connected();
    async fn on_disconnected();
    async fn on_data_received(data: BytesMut);
}
