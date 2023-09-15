pub mod common_interface;
pub mod common_listener;
pub mod connection_manager;

/// The union `int32` in `network_link`.
pub type ConnectionId = i32;

#[derive(thiserror::Error, Debug)]
pub enum ConnManagerError {
    #[error("Connection manager doesn't support type error.")]
    ConnManagerTypeNotSupport,
}
