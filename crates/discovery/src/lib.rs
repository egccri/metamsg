//! Discovery all area devices.
//!
//! Sometime `Slave` device is a publisher, use `Main` device scan slave devices, like `coap`, `multicast`.
//!
//! Sometime `Salve` devices actively connect to `Main` device that configured addr, like `static`.
mod coap;
mod config;
mod multicast;
mod r#static;
