mod xpub;

// 发送策略
pub enum Policy {
    Any, // 任意一个channel
    All, // 所有channel
    AllDevice, // 所有设备，同一设备只发送一份
}

