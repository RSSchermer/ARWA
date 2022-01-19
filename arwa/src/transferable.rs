pub(crate) mod transferable_seal {
    pub trait Seal {}
}

/// Marker trait for types that can be send across agents.
pub trait Transferable: transferable_seal::Seal {}
