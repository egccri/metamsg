//! Discovery all area devices.
//!
//! Sometime `Slave` device is a publisher, use `Main` device scan slave devices, like `coap`, `multicast`.
//!
//! Sometime `Salve` devices actively connect to `Main` device that configured addr, like `static`.

mod ble;
mod coap;
pub mod config;
pub mod manager;
mod manual;
mod multicast;

#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("{} server is not config.")]
    ConfigurationNotEnableError(String),
}

pub fn main_loop() {
    loop {
        tokio::task::spawn(async {});
    }
}
