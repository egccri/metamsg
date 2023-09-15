use crate::manager::conn_manager::ListenerCallback;

pub struct P2PServer {

}

pub struct P2PConnectionInfo {}

pub struct P2PListenerCallback {}

impl ListenerCallback for P2PListenerCallback {
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