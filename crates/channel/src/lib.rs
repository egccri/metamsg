//! `Channel` like a pipe, cache messages or bytes from connection, support some control on the channel,
//!  like Qos, retry, timeout and use hooks or services.
//!
//! `Channel` also is a can-search channel, support req/resp pattern, user need offer a fn that
//! match the request, this fn is call after codec.
mod channel;
mod client;
mod codec;
mod message;

pub use channel::{Channel, ChannelId};
