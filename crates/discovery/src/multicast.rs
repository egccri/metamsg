use std::net::SocketAddr;

#[derive(Debug)]
pub struct DiscoveryMulticastConfig {
    /// Whether multicast scouting is enabled or not. If left empty, `zenohd` will set it according to the presence of the `--no-multicast-scouting` argument.
    enabled: Option<bool>,
    /// The socket which should be used for multicast scouting. `zenohd` will use `224.0.0.224:7446` by default if none is provided.
    address: Option<SocketAddr>,
    /// The network interface which should be used for multicast scouting. `zenohd` will automatically select an interface if none is provided.
    interface: Option<String>,
    // /// Which type of Zenoh instances to automatically establish sessions with upon discovery through UDP multicast.
    // #[serde(deserialize_with = "treat_error_as_none")]
    // auto_connect: Option<ModeDependentValue<WhatAmIMatcher>>,
    // /// Whether or not to listen for scout messages on UDP multicast and reply to them.
    // listen: Option<ModeDependentValue<bool>>,
}
