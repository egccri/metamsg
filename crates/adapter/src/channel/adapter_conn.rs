use crate::message::Message;
use connection::manager::connection_manager::Connection;
use std::task::{Context, Poll};
use tower::Service;

pub struct AdapterConnection {
    inner: Connection,
}

impl<T> Service<Message<T>> for AdapterConnection {
    type Response = ();
    type Error = ();
    type Future = ();

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&mut self, req: Message<T>) -> Self::Future {
        todo!()
    }
}
