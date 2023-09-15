use crate::manager::common_listener::ListenerCallback;

pub struct BleServer {}

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
