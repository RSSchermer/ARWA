pub(crate) mod connection_status_seal {
    pub trait Seal {}
}

/// Implemented for types that have an associated connection status.
pub trait ConnectionStatus {
    /// Returns `true` if the connection is currently "online", `false` otherwise.
    fn online(&self) -> bool;
}
