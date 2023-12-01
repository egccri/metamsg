use crate::manager::DiscoveryCallback;
use routing::device::DeviceInfo;

pub struct CoapDiscoveryCallback {}

impl CoapDiscoveryCallback {
    pub fn new() -> CoapDiscoveryCallback {
        CoapDiscoveryCallback {}
    }
}

impl DiscoveryCallback for CoapDiscoveryCallback {
    fn on_device_found(&self) -> DeviceInfo {
        todo!()
    }

    fn discovery_success(&self) {
        todo!()
    }
}
