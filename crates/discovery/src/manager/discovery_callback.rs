use routing::device::{DeviceInfo, DeviceManager};
use std::sync::Arc;

/// Discovery callback, update routing-link
pub trait DiscoveryCallback {
    /// `impl Discovery` can use a new struct like `DeviceInfo` that real need by callback function.
    fn on_device_found(&self) -> DeviceInfo;

    fn _on_device_found(&self, device_manager: Arc<DeviceManager>) {
        let device_info = self.on_device_found();
        let _ = device_manager.add_device_info(device_info);
    }

    fn discovery_success(&self);
}
