use crate::multicast::multicast_discovery::new_socket;
use socket2::SockAddr;
use std::io;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

pub fn new_sender(addr: &SocketAddr) -> io::Result<UdpSocket> {
    let socket = new_socket(addr)?;

    if addr.is_ipv4() {
        // In order to control which interface multicast datagrams will be sent on,
        // the API provides the IP_MULTICAST_IF socket option. This option can be used to
        // set the interface for sending outbound multicast datagrams from the sockets application.
        // Multicast datagrams can be transmitted on only one interface at a time.
        socket.set_multicast_if_v4(&Ipv4Addr::new(0, 0, 0, 0))?;

        // IP_MULTICAST_LOOP 套接字选项来启用或禁用传出多播数据报的环回,默认已启用。
        // 此选项用于使系统上具有多个发送方和接收方的应用程序能够循环数据报，以便每个进程都能接收系统上其他发送方的传输。
        // socket.set_multicast_loop_v4(true);

        socket.bind(&SockAddr::from(SocketAddr::new(
            Ipv4Addr::new(0, 0, 0, 0).into(),
            35524,
        )))?;
    } else {
        // *WARNING* THIS IS SPECIFIC TO THE AUTHORS COMPUTER
        //   find the index of your IPv6 interface you'd like to test with.
        socket.set_multicast_if_v6(5)?;

        socket.bind(&SockAddr::from(SocketAddr::new(
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0).into(),
            0,
        )))?;
    }

    // convert to standard sockets...
    Ok(socket.into())
}
