mod discovery_listener;
mod discovery_manager;

use crate::coap::coap_callback::CoapDiscoveryCallback;
use crate::coap::CoapDiscoveryServer;
use routing_link::device::DeviceManager;
use std::sync::Arc;

pub(crate) use discovery_listener::DiscoveryCallback;
pub(crate) use discovery_manager::Discovery;

use crate::config::DiscoveryConfig;
use crate::manual::{ManualDiscoveryCallback, ManualDiscoveryServer};
pub use discovery_manager::{publish_service, start_discovery, stop_discovery, un_publish_service};

pub enum DiscoveryServer {
    Manual(ManualDiscoveryServer),
    Coap(CoapDiscoveryServer),
}

pub struct DiscoveryServers {
    inner: Vec<DiscoveryServer>,
    config: DiscoveryConfig,
    device_manager: Arc<DeviceManager>,
}

impl DiscoveryServers {
    pub fn init(config: DiscoveryConfig, device_manager: Arc<DeviceManager>) -> DiscoveryServers {
        let mut servers = Vec::new();
        let coap_server = CoapDiscoveryServer::new();
        servers.push(DiscoveryServer::Coap(coap_server));
        DiscoveryServers {
            inner: servers,
            config,
            device_manager,
        }
    }

    pub fn re_init(&self) -> DiscoveryServers {
        todo!()
    }
}

pub fn start_scan(discovery: DiscoveryServer, device_manager: Arc<DeviceManager>) {
    match discovery {
        DiscoveryServer::Manual(manual_server) => {
            manual_server._start_scan(ManualDiscoveryCallback::new(), device_manager.clone());
        }
        DiscoveryServer::Coap(coap_server) => {
            coap_server._start_scan(CoapDiscoveryCallback::new(), device_manager.clone());
        }
    }
}