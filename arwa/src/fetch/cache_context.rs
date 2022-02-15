pub(crate) mod cache_context_seal {
    pub trait Seal {}
}

pub trait CacheContext: cache_context_seal::Seal {
    // TODO: implement Cache API.
}
