fn main() {
    // use url::Url;
    //
    // use std::{
    //     net::{SocketAddr, UdpSocket},
    //     time::Duration,
    // };
    //
    // use libcoap_rs::{
    //     message::{CoapMessageCommon, CoapRequest, CoapResponse},
    //     protocol::{CoapMessageCode, CoapMessageType, CoapRequestCode, CoapResponseCode},
    //     session::{CoapClientSession, CoapSessionCommon},
    //     types::{CoapUri, CoapUriScheme},
    //     CoapContext, CoapRequestHandler, CoapResource,
    // };
    //
    // let server_address: SocketAddr = "[::1]:5683".parse().unwrap();
    //
    // // Create a new context.
    // let mut context = CoapContext::new().expect("Failed to create CoAP context");
    //
    // // Connect to the server at the specified address over UDP (plaintext CoAP)//!
    // let session = CoapClientSession::connect_udp(&mut context, server_address)
    //     .expect("Failed to create client-side session");
    //
    // // Create a new CoAP URI to request from.
    // let uri = CoapUri::try_from_url(Url::parse("coap://[::1]:5683/hello_world").unwrap()).unwrap();
    //
    // // Create a new request of type get with the specified URI.
    // let mut request = CoapRequest::new(CoapMessageType::Con, CoapRequestCode::Get).unwrap();
    // request.set_uri(Some(uri)).unwrap();
    //
    // // Send the request and wait for a response.
    // let req_handle = session
    //     .send_request(request)
    //     .expect("Unable to send request");
    // loop {
    //     context
    //         .do_io(Some(Duration::from_secs(10)))
    //         .expect("error during IO");
    //     // Poll for responses to a request using the request handle.
    //     for response in session.poll_handle(&req_handle) {
    //         assert_eq!(
    //             response.code(),
    //             CoapMessageCode::Response(CoapResponseCode::Content)
    //         );
    //         assert_eq!(response.data().unwrap().as_ref(), "Hello World!".as_bytes());
    //         return;
    //     }
    // }
}
