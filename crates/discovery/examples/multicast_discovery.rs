use discovery::multicast::{
    new_sender, MulticastContext, MulticastDiscoveryConfig, IPV4, IPV6, PORT,
};
use std::io;
use std::net::SocketAddr;
use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    let config = MulticastDiscoveryConfig::new();
    let context = MulticastContext::init_with_config(config);

    let listener = context.start_listener()?;

    let addr = SocketAddr::new(*IPV4, PORT);
    let message = b"Hello from client!";

    let socket = new_sender(&addr).expect("could not create sender!");

    loop {
        let mut buffer = Vec::with_capacity(4096);

        let result = socket.send_to(message, &addr);
        match result {
            Ok(size) => {
                println!("{}", size)
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        }

        match listener.recv_from(buffer.spare_capacity_mut()) {
            Ok((len, remote)) => {
                println!("{}-{:?}: {:?}", len, remote, buffer);
            }
            Err(err) => {
                eprintln!("{}", err);
                break;
            }
        }
        sleep(Duration::from_millis(1000));
    }
    sleep(Duration::from_millis(1000));
    Ok(())
}
