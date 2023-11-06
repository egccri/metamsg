use crate::device::DeviceInfo;

// 节点信息，包含设备信息，网络，认证等信息
pub struct NodeInfo {
    node_id: u64, // every router has a unique router id
    itself: DeviceInfo,
    know_devices: Vec<DeviceInfo>
}
