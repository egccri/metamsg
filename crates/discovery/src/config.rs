use crate::coap::CoapDiscoveryConfig;
use crate::manual::ManualDiscoveryConfig;
use crate::multicast::MulticastDiscoveryConfig;

#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    timeout: Option<u64>,
    delay: Option<u64>,
    pub manual: Option<ManualDiscoveryConfig>,
    pub coap: Option<CoapDiscoveryConfig>,
    pub multicast: Option<MulticastDiscoveryConfig>,
}
