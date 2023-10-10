mod multicast_discovery;

pub use multicast_discovery::new_sender;

use once_cell::sync::Lazy;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

pub const PORT: u16 = 7645;
pub static IPV4: Lazy<IpAddr> = Lazy::new(|| Ipv4Addr::new(224, 0, 0, 123).into());
pub static IPV6: Lazy<IpAddr> =
    Lazy::new(|| Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123).into());

#[derive(Debug, Clone)]
pub struct MulticastDiscoveryConfig {
    join_multicast_timeout: Option<u32>,
}

impl MulticastDiscoveryConfig {
    pub fn new() -> MulticastDiscoveryConfig {
        MulticastDiscoveryConfig {
            join_multicast_timeout: None,
        }
    }
}

pub struct MulticastContext {
    knows_host: Vec<SocketAddr>,
    config: MulticastDiscoveryConfig,
}

#[cfg(test)]
pub mod test {
    use crate::multicast::{MulticastContext, MulticastDiscoveryConfig, IPV4, IPV6};
    use std::io;
    use std::mem::MaybeUninit;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_ipv4_multicast() {
        assert!(IPV4.is_multicast());
    }

    #[test]
    fn test_ipv6_multicast() {
        assert!(IPV6.is_multicast());
    }

    #[test]
    fn test_server() -> io::Result<()> {
        let config = MulticastDiscoveryConfig {
            join_multicast_timeout: None,
        };
        let context = MulticastContext::init_with_config(config);
        let listener = context.start_listener()?;
        loop {
            let mut buffer = Vec::with_capacity(4096);

            match listener.recv_from(buffer.spare_capacity_mut()) {
                Ok((len, remote)) => {
                    println!("{}-{:?}: {:?}", len, remote, buffer);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    break;
                }
            }
        }
        sleep(Duration::from_millis(1000));
        Ok(())
    }
}
