pub mod ble_server;

pub struct BleConnectOptions {}

pub struct BleConnectionInfo {
    ble_mac: String,
}

impl BleConnectionInfo {
    pub fn ble_mac(&self) -> &str {
        &self.ble_mac
    }
}
