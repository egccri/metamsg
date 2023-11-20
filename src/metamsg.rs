use crate::session::Session;
use crate::Protocol;
use bytes::BytesMut;
use routing::Linkable;
use std::sync::Arc;

/// `Metamsg` is the api user use. `Metamsg` can cheap clone, and thread safe.
#[derive(Clone)]
pub struct Metamsg {
    sender: Option<Sender>,
    receiver: Option<Receiver>,
    auto_link: bool,
    linker: Arc<Vec<Box<dyn Linkable>>>,
    session: Arc<Box<dyn Session>>,
}

pub struct MetamsgBuilder {
    sender: Option<Sender>,
    receiver: Option<Receiver>,
    auto_link: bool,
    linker: Arc<Vec<Box<dyn Linkable>>>,
    session: Option<Arc<Box<dyn Session>>>,
}

impl MetamsgBuilder {
    pub fn new() -> MetamsgBuilder {
        MetamsgBuilder {
            sender: None,
            receiver: None,
            auto_link: false,
            linker: Arc::new(vec![]),
            session: None,
        }
    }

    pub fn proto(mut self, protocol: Protocol) -> Self {
        self
    }

    /// Set linker, see `Linkable`, linker used to find device, if device has registry, user can
    /// send to the device by name.

    pub fn enable_auto_link(mut self) -> Self {
        self.auto_link = true;
        self
    }

    pub fn linker(mut self, linker: Box<dyn Linkable>) -> Self {
        self.linker.push(linker);
        self
    }
}

/// Support consume specify index message.
pub struct Receiver {}

pub struct Sender {}

impl Metamsg {
    pub fn send(byte: BytesMut) {}

    pub fn recv(byte: BytesMut) {}

    // todo connect 是否可以包含listen（accept）
    pub fn connect(endpoint: Endpoint) {}

    pub fn listen(endpoint: Endpoint) {}
}

/// Example: "tcp://127.0.0.1:9999"
pub struct Endpoint {
    prefix: String,
    address: String,
}
