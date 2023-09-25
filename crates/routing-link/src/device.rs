/// Device info, type `Main` devices are peer to peer, type `Slave` devices are connect to
/// one `Main` device.
pub struct DeviceInfo {
    device_id: String,
    device_type: DeviceType,
}

pub enum DeviceType {
    Router,
    Client,
}
