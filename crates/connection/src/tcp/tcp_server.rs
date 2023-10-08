use crate::manager::common_interface::ListenerOptions;
use crate::manager::common_listener::ListenerCallback;
use crate::manager::{ConnManagerError, ListenerService};
use bytes::BytesMut;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::broadcast;

pub struct TcpListenerOptions {
    ip_addr: String,
    port: u16,
}

pub struct TcpServer {
    listener: TcpListener,
}

impl TcpServer {
    pub async fn accept(listener: &TcpListener) {
        let a = listener.accept().await;
    }
}

#[async_trait::async_trait]
impl ListenerService for TcpServer {
    async fn connection_server_init() {
        todo!()
    }

    async fn start_listener(
        options: ListenerOptions,
        mut shutdown_signal: broadcast::Receiver<()>,
    ) {
        match options {
            ListenerOptions::Tcp(option) => {
                let addr = format!("{}:{}", option.ip_addr, option.port);
                let listener = TcpListener::bind(addr).await.unwrap();
                loop {
                    select! {
                        _ = shutdown_signal.recv() => {
                            tracing::info!("Stopping tcp server at {}", chrono::Local::now());
                            break;
                        }
                        _ = Self::accept(&listener) => {}
                    }
                }
            }
        };
        tracing::info!("Server broker has stopped!");
    }

    async fn shutdown_listener() -> Result<(), ConnManagerError> {
        todo!()
    }
}

pub struct TcpListenerCallback {}

#[async_trait::async_trait]
impl ListenerCallback for TcpListenerCallback {
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
