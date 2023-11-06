pub enum Message {
    OAM(OAM)
}

/// The message that upgrade a remote node to router peers.
/// And every node has a know partner lists, p2p.
pub struct OAM {

}