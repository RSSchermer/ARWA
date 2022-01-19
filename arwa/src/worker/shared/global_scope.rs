#[derive(Clone)]
pub struct SharedWorkerGlobalScope {
    inner: web_sys::DedicatedWorkerGlobalScope,
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

impl_worker_global_scope_traits!(SharedWorkerGlobalScope, web_sys::SharedWorkerGlobalScope);

typed_event_stream!(OnConnect, OnConnectWithOptions, ConnectEvent, "connect");
