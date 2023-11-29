use bytes::BytesMut;
use metamsg::{Metamsg, Mode, Protocol, Transport};

// Engine之间如何分发消息

#[tokio::main]
async fn main() {}

async fn router() {
    // codec
    // 如何提供不同的接口？不提供，只在协议层提供连接管理，协议解析等，broker需要用户调用metamsg的统一接口然后实现
    let metamsg = Metamsg::new()
        .mode(Mode::Router)
        .linker(Box::new("127.0.0.1:8080")) // export discovery address
        .enable_auto_link()
        .transport(vec![Transport::TCP, Transport::BLE])
        .proto(Protocol::Pub)
        .build();

}

// 默认情况下，pub的所有消息，复制发送给每一个sub
async fn sub_client_tcp() {
    let metamsg = Metamsg::new()
        .mode(Mode::Client)
        .linker(Box::new("127.0.0.1:8080")) // link to the router with address
        .transport(vec![Transport::TCP])
        .proto(Protocol::Sub)
        .build();
    loop {
        let mut buf = BytesMut::with_capacity(64);
        metamsg.recv(buf).await;
        println!("{}", buf.into_vec());
    }

}

async fn pub_client_ble() {
    let metamsg = Metamsg::new()
        .mode(Mode::Client)
        .linker(Box::new("127.0.0.1:8080")) // link to the router with address
        .transport(vec![Transport::BLE])
        .proto(Protocol::Pub)
        .build();
    let buf = BytesMut::with_capacity(64);
    metamsg.send(buf).await;
}