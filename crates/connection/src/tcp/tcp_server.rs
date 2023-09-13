use crate::manager::conn_manager::ListenerCallback;

pub struct TcpServer {

}

pub struct TcpConnectionInfo {}

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
