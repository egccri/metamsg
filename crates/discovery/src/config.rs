use crate::coap::CoapDiscoveryConfig;
use crate::manual::ManualDiscoveryConfig;

pub struct DiscoveryConfig {
    timeout: Option<u64>,
    delay: Option<u64>,
    pub manual: Option<ManualDiscoveryConfig>,
    pub coap: Option<CoapDiscoveryConfig>,
}
