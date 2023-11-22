//! `routing-link` store a network topology between devices that contain two types.
//! Devices link by network channel.
//!
//! In the high level, topology has two model:
//! * peer to peer: `Main` devices
//! * client to router `Slave` devices and One `Main` device
//!
//! `routing-link` use `device-id`, then fetch connection in `Connection`.
//!
//! For topper level, find device by a set, but in the `routing-link`, it will find the previous device,
//! then find real device, like following graph(device_1st call device_3rd).
//! ```text
//! |------------|
//! | device_1st |
//! | device_2nd |  => device_1st -> device_2nd -> device_3rd
//! | device_3rd |
//! |------------|
//! ```
//!
//! Depend on routing, you can easily build a distributed network, you only need focus on protocol itself.
pub mod device;
mod link;
mod message;
mod net_ledger;
mod node;
mod routing;

/// The main node that persists the routing graph is typically a router.
pub enum NodeType {
    Main,
    Slave,
}

/// `Linkable` is used by Metamsg linker method. It's maybe a static address, or a discovery.
pub trait Linkable {}

impl<T> Linkable for Box<T> {}

impl Linkable for String {}

impl Linkable for &str {}
