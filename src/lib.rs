#![feature(associated_type_defaults)]
//! Metamsg
mod config;
mod engine;
mod metamsg;
mod session;

pub use metamsg::{Metamsg, MetamsgBuilder};

pub enum Protocol {
    Pub,
    Sub,
}

pub enum Engine {
    TCP,
    UDP,
    BLE,
}
