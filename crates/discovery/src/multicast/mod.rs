use once_cell::sync::Lazy;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

static IPV4: Lazy<IpAddr> = Lazy::new(|| Ipv4Addr::new(224, 0, 0, 123).into());
static IPV6: Lazy<IpAddr> = Lazy::new(|| Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123).into());

#[test]
fn test_ipv4_multicast() {
    assert!(IPV4.is_multicast());
}

#[test]
fn test_ipv6_multicast() {
    assert!(IPV6.is_multicast());
}
