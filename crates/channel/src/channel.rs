use connection::manager::connection_manager::Connection;

pub struct Channel {
    inner: Option<Connection>,
    config: ChannelConfig,
}

#[derive(Debug)]
pub struct ChannelConfig {
    re_connect: bool,
    qos: QoS,
}

pub enum QoS {
    Low, Medium, High
}

impl Default for ChannelConfig {
    fn default() -> Self {
        ChannelConfig {
            re_connect: true,
            qos: QoS::High,
        }
    }
}


impl Channel {

}

