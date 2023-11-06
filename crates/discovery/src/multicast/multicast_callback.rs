use crate::manager::DiscoveryCallback;
use routing_link::device::DeviceInfo;

pub struct MulticastCallback {}

impl DiscoveryCallback for MulticastCallback {
    fn on_device_found(&self) -> DeviceInfo {
        todo!()
    }

    fn discovery_success(&self) {
        todo!()
    }
}
