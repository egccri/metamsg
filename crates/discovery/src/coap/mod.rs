pub mod coap_callback;
mod coap_client;
mod coap_discovery;

use crate::coap::coap_callback::CoapDiscoveryCallback;
use crate::manager::{Discovery, DiscoveryCallback};
use coap::Server;

pub const COAP_DEFAULT_SERVER_ADDR: &str = "0.0.0.0";
pub const COAP_DEFAULT_SERVER_PORT: &str = "5684";

#[derive(Debug, Clone)]
pub struct CoapDiscoveryConfig {
    pub addr: String,
}

#[derive(Debug)]
pub struct CoapDiscoveryServer {
    config: CoapDiscoveryConfig,
}

impl CoapDiscoveryServer {
    pub fn new(config: CoapDiscoveryConfig) -> CoapDiscoveryServer {
        CoapDiscoveryServer { config }
    }
}

impl Discovery<CoapDiscoveryCallback> for CoapDiscoveryServer {
    fn start_scan(&self) {
        // let mut server = Server::new(self.config.addr.clone()).unwrap();
    }

    fn stop_scan(&self) {
        todo!()
    }

    fn subscribe(&self) {
        todo!()
    }

    fn unsubscribe(&self) {
        todo!()
    }
}
