//! `network-device-link` store a network topology between devices that contain two types.
//! Devices link by network channel.
//!
//! In the high level, topology has two model:
//! * peer to peer: `Main` devices
//! * client to router `Slave` devices and One `Main` device
//!
//! `network-device-link` use `device-id`, then fetch connection in `Connection`.
pub mod device;
mod link;
mod routing;
