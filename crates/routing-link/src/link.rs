/// Support flight windows
pub struct NetworkLink {}

/// Store framed message
pub struct Table {}

/// Store strategy that for every `NetworkLink`
pub enum StoreStrategy {
    LastWill,
    Unused,
    SizeLimit,
    NumLimit,
}
