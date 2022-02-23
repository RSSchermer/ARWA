use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::event::{impl_event_target_traits, impl_try_from_event_target};
use crate::message::MessagePort;
use crate::url::AbsoluteOrRelativeUrl;
use crate::worker::{worker_seal, CreateWorkerError, Worker, WorkerOptions};

#[derive(Clone)]
pub struct SharedWorker {
    inner: web_sys::SharedWorker,
}

impl SharedWorker {
    pub fn create<T>(url: T, options: WorkerOptions) -> Self
    where
        T: AbsoluteOrRelativeUrl,
    {
        create_shared_worker_internal(url, options).unwrap_throw()
    }

    pub fn try_create<T>(url: T, options: WorkerOptions) -> Result<Self, CreateWorkerError>
    where
        T: AbsoluteOrRelativeUrl,
    {
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

impl_event_target_traits!(SharedWorker);
impl_try_from_event_target!(SharedWorker);

fn create_shared_worker_internal<T>(url: T, options: WorkerOptions) -> Result<SharedWorker, JsValue>
where
    T: AbsoluteOrRelativeUrl,
{
    let result = web_sys::SharedWorker::new_with_worker_options(
        url.as_str(),
        &options.into_web_sys_worker_options(),
    );

    result.map(|w| w.into())
}
