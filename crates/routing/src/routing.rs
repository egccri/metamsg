use crate::device::DeviceId;
use crate::node::NodeInfo;

pub type RouterId = u64;

pub type ChannelId = u64;
pub type NodeId = (DeviceId, ChannelId);

pub struct Router {
    router_id: RouterId,
    peer_nodes: Vec<NodeInfo>,
}
