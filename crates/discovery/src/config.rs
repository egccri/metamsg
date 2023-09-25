use crate::multicast::DiscoveryMulticastConfig;
use crate::r#static::DiscoveryStaticConfig;

pub struct DiscoveryConfig {
    timeout: Option<u64>,
    delay: Option<u64>,
    pub r#static: DiscoveryStaticConfig,
    pub multicast: DiscoveryMulticastConfig,
}
