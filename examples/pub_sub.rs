use bytes::BytesMut;
use metamsg::{Metamsg, Protocol, Transport};

#[tokio::main]
async fn main() {}

async fn sub_server() {
    // codec
    // 如何提供不同的接口？不提供，只在协议层提供连接管理，协议解析等，broker需要用户调用metamsg的统一接口然后实现
    let metamsg = Metamsg::new()
        .linker(Box::new("127.0.0.1:8080"))
        .linker(Box::new("127.0.0.1:8090".to_string()))
        .enable_auto_link()
        .transport(Transport::TCP)
        .proto(Protocol::Sub)
        .build();
    loop {
        let mut buf = BytesMut::with_capacity(64);
        metamsg.recv(buf).await;
        println!("{}", buf.into_vec());
    }
}

async fn pub_client() {}
