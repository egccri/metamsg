// ProtocolContext is a "context" for a protocol, which contains the
// various stateful operations such as timers, etc. necessary for
// running the protocol.  This is separable from the protocol itself
// as the protocol may permit the creation of multiple contexts.

use crate::Transport;
use channel::Channel;
use tokio::runtime::Runtime;
use connection::tcp::tcp_server::TcpServer;

// todo Engine is self executable,
// 基于Engine实现Layer，过滤所有channel
// todo 是否可以合并 client 和 Server
pub struct Engine {
    transport: Transport,
    server: TcpServer,
    client: Channel,
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

impl Engine {
    pub fn add_channel() {}

    pub fn remove_channel() {}
}
