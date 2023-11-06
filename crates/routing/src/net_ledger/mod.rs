mod remote_meta_ledger;
/// The store of network topology.
mod remote_net_ledger;

use crate::node::NodeInfo;

pub struct LocalNetLedger {
    node_info: NodeInfo,
    status: LocalLedgerStatus,
}

pub enum LocalLedgerStatus {
    LlInitUnknown = 0,
    LlInitFail,
    LlInitSuccess,
}
