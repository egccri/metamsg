use discovery::multicast::{
    new_sender, new_socket, MulticastContext, MulticastDiscoveryConfig, IPV4, PORT,
};
use std::io;
use std::mem::MaybeUninit;
use std::net::{SocketAddr, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

fn main() -> io::Result<()> {
    let handler = std::thread::spawn(|| start_multicast_server());

    // run a client
    let message = b"Hello from client!";
    let addr = SocketAddr::new(*IPV4, PORT);
    let socket = new_sender(&addr).expect("Could not create sender!");
    for _ in 1..10 {
        let result = socket.send_to(message, &addr);
        match result {
            Ok(size) => {
                println!("{}", size)
            }
            Err(err) => {
                eprintln!("{}", err)
            }
        }
        sleep(Duration::from_secs(1));
    }

    let _ = handler.join();
    Ok(())
}

fn start_multicast_server() -> io::Result<()> {
    let config = MulticastDiscoveryConfig::new();
    let context = MulticastContext::init_with_config(config);

    let listener = context.start_listener()?;

    loop {
        let mut buf = [MaybeUninit::uninit(); 1024];

        match listener.recv_from(&mut buf) {
            Ok((len, remote)) => {
                let result: String = buf[..len].iter()
                    .map(|&char_maybe_uninit| {
                        // Use assume_init to convert MaybeUninit<char> to char
                        let initialized_char: char =
                            unsafe { char_maybe_uninit.assume_init() as char } ;
                        initialized_char
                    })
                    .collect();

                println!("recv message from {:?}, len:{}: {:?}", &remote.as_socket(), len, result);

                let remote_addr = &remote.as_socket().unwrap();

                let responder: UdpSocket = new_socket(remote_addr)
                    .expect("failed to create responder")
                    .into();

                match responder.send_to(&vec![255, 255], remote_addr) {
                    Ok(size) => {
                        println!("Response size: {}", size);
                    }
                    Err(err) => {
                        eprintln!("{}", err);
                    }
                };
            }
            Err(err) => {
                // eprintln!("{}", err.kind());
                continue;
            }
        }
        sleep(Duration::from_millis(10));
    }
}
