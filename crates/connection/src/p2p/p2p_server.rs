use crate::manager::common_listener::ListenerCallback;
use bytes::BytesMut;

pub struct P2PServer {}

pub struct P2PListenerCallback {}

#[async_trait::async_trait]
impl ListenerCallback for P2PListenerCallback {
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
