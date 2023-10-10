use coap::{CoAPClient, Server};
use coap_lite::RequestType;
use tokio::runtime::Runtime;

fn main() {
    let addr = "127.0.0.1:5683";
    Runtime::new().unwrap().block_on(async move {
        let mut server = Server::new(addr).unwrap();
        println!("Server up on {}", addr);

        server
            .run(|request| async {
                match request.get_method() {
                    &RequestType::Get => println!("request by get {}", request.get_path()),
                    &RequestType::Post => println!(
                        "request by post {}",
                        String::from_utf8(request.message.payload).unwrap()
                    ),
                    &RequestType::Put => println!(
                        "request by put {}",
                        String::from_utf8(request.message.payload).unwrap()
                    ),
                    _ => println!("request by other method"),
                };
                return match request.response {
                    Some(mut message) => {
                        message.message.payload = b"OK".to_vec();
                        Some(message)
                    }
                    _ => None,
                };
            })
            .await
            .unwrap();
    });
}

fn client() {
    let url = "coap://127.0.0.1:5683/Rust";
    println!("Client request: {}", url);

    let response = CoAPClient::get(url).unwrap();
    println!(
        "Server reply: {}",
        String::from_utf8(response.message.payload).unwrap()
    );
}
