//! `Channel` like a pipe, cache messages or bytes from connection, support some control on the channel,
//!  like Qos, retry, timeout and use hooks or services.
mod channel;
mod client;
mod codec;
mod message;

pub use channel::{Channel, ChannelId};
