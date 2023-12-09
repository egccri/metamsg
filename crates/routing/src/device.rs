mod manager;

pub use manager::DeviceManager;

#[derive(Debug, thiserror::Error)]
pub enum DeviceError {}

pub type DeviceId = String;

/// Device info, type `Main` devices are peer to peer, type `Slave` devices are connect to
/// one `Main` device.
pub struct DeviceInfo {
    device_id: DeviceId,
    device_type: DeviceType,
}

/// Router type contain server and peer.
pub enum DeviceType {
    Router = 0b001,
    Client = 0b010,
}


/// Device default is a `Client`, `Router` must be explicitly specified.
impl Default for DeviceType {
    fn default() -> Self {
        DeviceType::Client
    }
}
