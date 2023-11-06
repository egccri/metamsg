use crate::coap::CoapDiscoveryServer;
use crate::manager::discovery_callback::DiscoveryCallback;
use crate::manual::ManualDiscoveryServer;
use routing_link::device::DeviceManager;
use std::sync::Arc;

pub enum DiscoveryServer {
    Manual(ManualDiscoveryServer),
    Multicast(),
    Coap(CoapDiscoveryServer),
}

pub trait Discovery<Callback: DiscoveryCallback> {
    fn start_scan(&self);

    fn _start_scan(&self, callback: Callback, device_manager: Arc<DeviceManager>) {
        self.start_scan();
        callback._on_device_found(device_manager)
    }

    fn stop_scan(&self);

    fn subscribe(&self);

    fn unsubscribe(&self);
}

pub fn start_discovery() {}

pub fn stop_discovery() {}

pub fn publish_service() {}

pub fn un_publish_service() {}
