/// Use static addr to connect, include tcp, udp etc.
use crate::manager::{Discovery, DiscoveryCallback};
use routing_link::device::DeviceInfo;

pub struct ManualDiscoveryConfig {}

pub struct ManualDiscoveryServer {}

pub struct ManualDiscoveryCallback {}

impl ManualDiscoveryServer {
    pub fn new() -> ManualDiscoveryServer {
        ManualDiscoveryServer {}
    }
}

impl Discovery<ManualDiscoveryCallback> for ManualDiscoveryServer {
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

impl ManualDiscoveryCallback {
    pub fn new() -> ManualDiscoveryCallback {
        ManualDiscoveryCallback {}
    }
}

impl DiscoveryCallback for ManualDiscoveryCallback {
    fn on_device_found(&self) -> DeviceInfo {
        todo!()
    }

    fn discovery_success(&self) {
        todo!()
    }
}
