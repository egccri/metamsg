use crate::manager::conn_manager::ListenerCallback;

pub struct BleServer {

}

pub struct BleConnectionInfo {}

pub struct BleListenerCallback {}

impl ListenerCallback for BleListenerCallback {
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