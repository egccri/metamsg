use crate::engine::Engine;
use crate::Protocol;
use channel::ChannelId;
use connection::manager::connection_manager::Connection;
use connection::manager::ConnectionId;
use routing::device::DeviceInfo;
use std::collections::HashMap;

// ProtocolBase provides the protocol-specific handling for sockets.
// This is the new style API for sockets, and is how protocols provide
// their specific handling.

// todo MetamsgContext 包含各种类型的 Engine，并根据 channel_id 发送消息
// todo MetamsgContext 包含 多个Engine，多个Engine之间可以互传消息
// todo dispatch Message(Bytes, MaybeUnit) to the Engine or Channel

// metamsg 发送来的消息，如何分发到不同的engine？

// 消息先发给Protocol，然后做一些单独处理逻辑，然后根据channel_id发送给Engine
pub struct MetamsgContext {
    protocol: Protocol,
    dispatch: HashMap<ChannelId, Engine>,
    engines: Vec<Engine>,
}

impl<T> MetamsgContext<T> {
    pub fn new() -> MetamsgContext<T> {
        MetamsgContext {
            protocol: Protocol::Pub,
            engines: vec![],
            dispatch: Default::default(),
        }
    }
}
