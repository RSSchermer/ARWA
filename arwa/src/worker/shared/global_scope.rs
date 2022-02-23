use crate::event::typed_event_iterator;
use crate::worker::impl_worker_global_scope_traits;
use crate::worker::shared::ConnectEvent;

#[derive(Clone)]
pub struct SharedWorkerGlobalScope {
    inner: web_sys::SharedWorkerGlobalScope,
}

impl SharedWorkerGlobalScope {
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

    pub fn on_connect(&self) -> OnConnect<Self> {
        OnConnect::new(self.inner.as_ref())
    }
}

impl From<web_sys::SharedWorkerGlobalScope> for SharedWorkerGlobalScope {
    fn from(inner: web_sys::SharedWorkerGlobalScope) -> Self {
        SharedWorkerGlobalScope { inner }
    }
}

impl AsRef<web_sys::SharedWorkerGlobalScope> for SharedWorkerGlobalScope {
    fn as_ref(&self) -> &web_sys::SharedWorkerGlobalScope {
        &self.inner
    }
}

impl_worker_global_scope_traits!(SharedWorkerGlobalScope, SharedWorkerGlobalScope);

typed_event_iterator!(OnConnect, OnConnectWithOptions, ConnectEvent, "connect");
