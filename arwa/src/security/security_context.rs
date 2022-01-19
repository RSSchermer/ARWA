pub(crate) mod security_context_seal {
    pub trait Seal {}
}

pub trait SecurityContext: security_context_seal::Seal {
    fn origin(&self) -> String;

    fn is_secure_context(&self) -> bool;
}
