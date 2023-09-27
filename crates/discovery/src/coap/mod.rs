pub mod coap_callback;

use crate::coap::coap_callback::CoapDiscoveryCallback;
use crate::manager::{Discovery, DiscoveryCallback};

pub struct CoapDiscoveryConfig {}

#[derive(Debug)]
pub struct CoapDiscoveryServer {}

impl CoapDiscoveryServer {
    pub fn new() -> CoapDiscoveryServer {
        CoapDiscoveryServer {}
    }
}

impl Discovery<CoapDiscoveryCallback> for CoapDiscoveryServer {
    fn start_scan(&self) {
        todo!()
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
