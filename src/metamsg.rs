use crate::context::MetamsgContext;
use crate::Protocol;
use bytes::{Bytes, BytesMut};
use routing::Linkable;
use std::sync::Arc;

/// `Metamsg` is the api user use. `Metamsg` can cheap clone, and thread safe.
#[derive(Clone)]
pub struct Metamsg {
    auto_link: bool,
    linkers: Arc<Vec<Box<dyn Linkable>>>,
    context: MetamsgContext,
}

impl Metamsg {
    pub fn new() -> MetamsgBuilder {
        MetamsgBuilder::new()
    }
}

pub struct MetamsgBuilder {
    sender: Option<Sender>,
    receiver: Option<Receiver>,
    auto_link: bool,
    linkers: Arc<Vec<Box<dyn Linkable>>>,
}

impl MetamsgBuilder {
    pub fn new() -> MetamsgBuilder {
        MetamsgBuilder {
            sender: None,
            receiver: None,
            auto_link: false,
            linkers: Arc::new(vec![]),
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
        self.linkers.push(linker);
        self
    }

    pub fn build(self) -> Metamsg {
        Metamsg {
            auto_link: false,
            linkers: Arc::new(vec![]),
            context: MetamsgContext::new(),
        }
    }
}

/// Support consume specify index message.
pub struct Receiver {}

pub struct Sender {}


pub trait Primitives: Send + Sync {
    async fn send(&self, byte: BytesMut) {}

    async fn recv(&self, byte: BytesMut) {}
}

impl Metamsg {
    pub async fn send(&self, byte: BytesMut) {}

    pub async fn recv(&self, byte: BytesMut) {}

    // todo connect 是否可以包含listen（accept）
    pub async fn connect(&self, endpoint: Endpoint) {}

    pub async fn listen(&self, endpoint: Endpoint) {}
}

/// Example: "tcp://127.0.0.1:9999"
pub struct Endpoint {
    prefix: String,
    address: String,
}
