use crate::fetch::RequestCredentials;
use crate::message::MessagePort;
use crate::url::ContextualUrl;
use crate::worker::{worker_seal, CreateWorkerError, Worker, WorkerOptions, WorkerType};
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

#[derive(Clone)]
pub struct SharedWorker {
    inner: web_sys::SharedWorker,
}

impl SharedWorker {
    pub fn create(url: ContextualUrl, options: WorkerOptions) -> Self {
        create_shared_worker_internal(url, options).unwrap_throw()
    }

    pub fn try_create(
        url: ContextualUrl,
        options: WorkerOptions,
    ) -> Result<Self, CreateWorkerError> {
        create_shared_worker_internal(url, options)
            .map_err(|err| CreateWorkerError::new(err.unchecked_into()))
    }

    pub fn port(&self) -> MessagePort {
        self.inner.port().into()
    }
}

impl worker_seal::Seal for SharedWorker {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl Worker for SharedWorker {}

impl From<web_sys::SharedWorker> for SharedWorker {
    fn from(inner: web_sys::SharedWorker) -> Self {
        SharedWorker { inner }
    }
}

impl AsRef<web_sys::SharedWorker> for SharedWorker {
    fn as_ref(&self) -> &web_sys::SharedWorker {
        &self.inner
    }
}

impl_common_wrapper_traits!(SharedWorker);

fn create_shared_worker_internal(
    url: ContextualUrl,
    options: WorkerOptions,
) -> Result<SharedWorker, JsValue> {
    let result = web_sys::SharedWorker::new_with_worker_options(
        url.as_ref(),
        &options.into_web_sys_worker_options(),
    );

    result.map(|w| w.into())
}
