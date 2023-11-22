use bytes::BytesMut;
use metamsg::{Metamsg, Protocol};

#[tokio::main]
async fn main() {}

async fn sub_server() {
    // codec
    let metamsg = Metamsg::new()
        .linker(Box::new("127.0.0.1:8080"))
        .linker(Box::new("127.0.0.1:8090".to_string()))
        .enable_auto_link()
        .proto(Protocol::Sub)
        .build();
    loop {
        let mut buf = BytesMut::with_capacity(64);
        metamsg.recv(buf).await;
        println!("{}", buf.into_vec());
    }
}

async fn pub_client() {}
