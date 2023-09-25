use std::collections::HashMap;
use crate::rpc::protocol::request::Request;

mod protocol;

/// Default request id
pub type RequestId = u32;

/// `fn() -> RequestId` support a configured request_id generator.
pub struct Client {
    next_request_id: fn() -> RequestId,
    pending_requests: HashMap<RequestId, Request>
}
