use wasm_bindgen::{throw_val, JsCast, JsValue};

use crate::event::{impl_event_target_traits, impl_try_from_event_target};
use crate::message::{
    message_event_target_seal, message_sender_seal, MessageEventTarget, MessageSender,
};
use crate::url::Url;
use crate::worker::{worker_seal, CreateWorkerError, Worker, WorkerOptions};

#[derive(Clone)]
pub struct DedicatedWorker {
    inner: web_sys::Worker,
}

impl DedicatedWorker {
    pub fn create(url: &Url, options: WorkerOptions) -> Self {
        match create_dedicated_worker_internal(url, options) {
            Ok(worker) => worker,
            Err(err) => throw_val(err),
        }
    }

    pub fn try_create(url: &Url, options: WorkerOptions) -> Result<Self, CreateWorkerError> {
        create_dedicated_worker_internal(url, options)
            .map_err(|err| CreateWorkerError::new(err.unchecked_into()))
    }

    pub fn terminate(&self) {
        self.inner.terminate()
    }
}

impl worker_seal::Seal for DedicatedWorker {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl Worker for DedicatedWorker {}

impl message_sender_seal::Seal for DedicatedWorker {}

impl MessageSender for DedicatedWorker {}

impl message_event_target_seal::Seal for DedicatedWorker {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl MessageEventTarget for DedicatedWorker {}

impl From<web_sys::Worker> for DedicatedWorker {
    fn from(inner: web_sys::Worker) -> Self {
        DedicatedWorker { inner }
    }
}

impl AsRef<web_sys::Worker> for DedicatedWorker {
    fn as_ref(&self) -> &web_sys::Worker {
        &self.inner
    }
}

impl_event_target_traits!(DedicatedWorker);
impl_try_from_event_target!(DedicatedWorker, Worker);

fn create_dedicated_worker_internal(
    url: &Url,
    options: WorkerOptions,
) -> Result<DedicatedWorker, JsValue> {
    let result =
        web_sys::Worker::new_with_options(url.as_ref(), &options.into_web_sys_worker_options());

    result.map(|w| w.into())
}
