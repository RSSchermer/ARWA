use wasm_bindgen::JsCast;

use crate::fetch::{cache_context_seal, CacheContext};
use crate::message::{
    message_event_target_seal, message_sender_seal, MessageEventTarget, MessageSender,
};
use crate::worker::impl_worker_global_scope_traits;

pub fn current() -> DedicatedWorkerGlobalScope {
    js_sys::global()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        .expect("Current context is not a dedicated worker context.")
        .into()
}

#[derive(Clone)]
pub struct DedicatedWorkerGlobalScope {
    inner: web_sys::DedicatedWorkerGlobalScope,
}

impl DedicatedWorkerGlobalScope {
    pub fn name(&self) -> Option<String> {
        let name = self.inner.name();

        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }

    pub fn close(&self) {
        self.inner.close();
    }
}

impl message_event_target_seal::Seal for DedicatedWorkerGlobalScope {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl MessageEventTarget for DedicatedWorkerGlobalScope {}

impl message_sender_seal::Seal for DedicatedWorkerGlobalScope {}

impl MessageSender for DedicatedWorkerGlobalScope {}

impl cache_context_seal::Seal for DedicatedWorkerGlobalScope {}

impl CacheContext for DedicatedWorkerGlobalScope {}

impl From<web_sys::DedicatedWorkerGlobalScope> for DedicatedWorkerGlobalScope {
    fn from(inner: web_sys::DedicatedWorkerGlobalScope) -> Self {
        DedicatedWorkerGlobalScope { inner }
    }
}

impl AsRef<web_sys::DedicatedWorkerGlobalScope> for DedicatedWorkerGlobalScope {
    fn as_ref(&self) -> &web_sys::DedicatedWorkerGlobalScope {
        &self.inner
    }
}

impl_worker_global_scope_traits!(DedicatedWorkerGlobalScope, DedicatedWorkerGlobalScope);
