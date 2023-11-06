use crate::manager::common_listener::ListenerCallback;
use bytes::BytesMut;

pub struct BleServer {}

pub struct BleListenerCallback {}

#[async_trait::async_trait]
impl ListenerCallback for BleListenerCallback {
    async fn on_connected() {
        todo!()
    }

    async fn on_disconnected() {
        todo!()
    }

    async fn on_data_received(data: BytesMut) {
        todo!()
    }
}
