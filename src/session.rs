use crate::engine::Engine;
use connection::manager::connection_manager::Connection;
use connection::manager::ConnectionId;

// ProtocolBase provides the protocol-specific handling for sockets.
// This is the new style API for sockets, and is how protocols provide
// their specific handling.

// todo Session 包含各种类型的 Connection，并根据connection_id发送消息
// todo 每种协议的Session不一样，有自己的Session，每个Metamsg对应一个Session
// todo Session 包含 多个Engine，多个Engine之间可以互传消息
// todo dispatch Message(Bytes, MaybeUnit) to the Engine or Connection
pub trait Session {
    fn add_conn(conn: Connection);

    fn remove_conn(conn_id: ConnectionId);
}
