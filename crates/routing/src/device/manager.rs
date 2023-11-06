use crate::device::{DeviceError, DeviceInfo};

/// 提供接口供Egccri实现，Egccri实现通过时序数据库Meta接口，查询租户下所有设备，返回给Metamsg。
pub struct DeviceManager {}

impl DeviceManager {
    pub fn add_device_info(&self, device_info: DeviceInfo) -> Result<(), DeviceError> {
        todo!()
    }

    pub fn trust_device() {}
}
