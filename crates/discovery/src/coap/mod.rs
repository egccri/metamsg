pub mod coap_callback;

use crate::coap::coap_callback::CoapDiscoveryCallback;
use crate::manager::{Discovery, DiscoveryCallback};
use coap::Server;

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
        let mut server = Server::new(self.config.addr.clone()).unwrap();
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
