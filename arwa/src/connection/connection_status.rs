pub(crate) mod connection_status_seal {
    pub trait Seal {}
}

pub trait ConnectionStatus {
    fn online(&self) -> bool;
}
