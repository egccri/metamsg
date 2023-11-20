pub enum Message {
    OAM(OAM),
}

/// The message that upgrade a remote node to router peers.
/// And every node has a know partner lists, p2p.
// 如果OAM自己信任的，或者拥有相同的namespace，建立连接
pub struct OAM {
    node_id: u64, // remote router id
}
