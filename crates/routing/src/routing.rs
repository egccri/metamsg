use crate::node::NodeInfo;

pub type RouterId = u64;

pub struct Router {
    router_id: RouterId,
    peer_nodes: Vec<NodeInfo>,
}
