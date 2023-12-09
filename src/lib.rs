#![feature(associated_type_defaults)]
//! Metamsg
mod config;
mod context;
mod engine;
pub mod metamsg;

pub use metamsg::{Metamsg, MetamsgBuilder};

pub enum Mode {
    Router,
    Client,
}

pub enum Protocol {
    Pub,
    Sub,
}

pub enum Transport {
    TCP,
    UDP,
    BLE,
}
