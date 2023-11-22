// ProtocolContext is a "context" for a protocol, which contains the
// various stateful operations such as timers, etc. necessary for
// running the protocol.  This is separable from the protocol itself
// as the protocol may permit the creation of multiple contexts.

use crate::EngineType;
use channel::Channel;
use connection::manager::common_interface::ListenerOptions;
use tokio::runtime::Runtime;

// todo Engine is self executable,

pub struct Engine {
    engine_type: EngineType,
    channels: Vec<Channel>,
    runtime: Runtime,
}

// pub trait Engine {
//     type Listener;
//
//     type Codec;
//
//     fn start(option: ListenerOptions);
//
//     fn shutdown();
// }
