use tokio_util::bytes::Bytes;
use connection::manager::connection_manager::Connection;

mod adapter_conn;

use crate::channel::adapter_conn::AdapterConnection;
use crate::message::Message;
use tower::buffer::Buffer;

// Why either connection and svc, because of maybe some service on the connection, before it become
// a channel.
// type Svc = Either<Connection, BoxService<Request<BoxBody>, Response<hyper::Body>, crate::Error>>;

pub type ChannelId = u64;

pub struct Channel {
    channel_id: ChannelId,
    inner: Inner<Bytes>,
    context: ChannelContext,
    config: ChannelConfig,
}

/// `Channel` serve multi message with `Service`s, it's link connection and `Service`s.
// 将T延迟到方法级别，主要有byte和自定义codec
pub struct Inner<T> {
    svc: Buffer<AdapterConnection, Message<T>>,
}

// 存储channel里的数据该发送到哪里
pub struct ChannelContext {}

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

impl<T> Channel<T> {}
