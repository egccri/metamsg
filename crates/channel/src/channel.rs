use connection::manager::connection_manager::Connection;

mod adapter_conn;

use crate::channel::adapter_conn::AdapterConnection;
use crate::message::Message;
use tower::buffer::Buffer;

// Why either connection and svc, because of maybe some service on the connection, before it become
// a channel.
// type Svc = Either<Connection, BoxService<Request<BoxBody>, Response<hyper::Body>, crate::Error>>;

/// `Channel` serve multi message with `Service`s, it's link connection and `Service`s.
pub struct Inner<T> {
    svc: Buffer<AdapterConnection, Message<T>>,
}

pub struct Channel {
    inner: Inner<T>,
    config: ChannelConfig,
}

#[derive(Debug)]
pub struct ChannelConfig {
    re_connect: bool,
    qos: QoS,
}

pub enum QoS {
    Low,
    Medium,
    High,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        ChannelConfig {
            re_connect: true,
            qos: QoS::High,
        }
    }
}

impl Channel {}
