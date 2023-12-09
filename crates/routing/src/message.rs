use crate::NodeType;

pub enum Message {
    OAM(OAM),
    Transfer(TransferMessage),
}

/// The message that upgrade a remote node to router peers.
/// And every node has a know partner lists, p2p.
// 如果OAM自己信任的，或者拥有相同的namespace，建立连接
// 默认discovery发现的设备是client，收到OAM消息后升级为Peer或者Router
pub struct OAM {
    node_id: u64, // remote router id
    node_type: NodeType,
    region: u64,
}

pub struct TransferMessage {
    header: RoutingMessageHeader,
    payload: Vec<u8>,
}

// 透传数据时需要将数据属性放置于Header里，然后包装原始数据发送
pub struct RoutingMessageHeader {
    qos: u8,
}
