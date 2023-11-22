use std::collections::HashMap;
use channel::{Channel, ChannelId};

pub struct Socket {
    sub_channels: HashMap<ChannelId, Channel>
}

