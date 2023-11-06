/// Rpc request, support QoS
/// Use `Receiver` consume specify index message if `Request` has some reply.
pub struct Request {
    reply: Option<Reply>,
}

pub struct Reply {}
