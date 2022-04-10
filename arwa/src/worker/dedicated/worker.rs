use wasm_bindgen::prelude::*;
use wasm_bindgen::{throw_val, JsCast, JsValue};

use crate::event::{impl_event_target_traits, impl_try_from_event_target};
use crate::message::{
    message_event_target_seal, message_sender_seal, MessageEventTarget, MessageSender,
};
use crate::url::Url;
use crate::worker::dedicated::{current, DedicatedWorkerGlobalScope};
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

    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce(DedicatedWorkerGlobalScope) + Send + 'static,
    {
        let closure = Box::new(Box::new(f) as Box<dyn FnOnce(DedicatedWorkerGlobalScope) + Send>);
        let ptr = Box::into_raw(closure);

        match spawn_worker(&wasm_bindgen::module(), &wasm_bindgen::memory(), ptr) {
            Ok(worker) => worker.into(),
            Err(err) => {
                // Since the worker failed to initialize, the closure does not get used and cleaned
                // up. Clean it up here
                unsafe {
                    Box::from_raw(ptr);
                }

                throw_val(err);
            }
        }
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

#[wasm_bindgen(module = "/src/worker/dedicated/spawn_worker.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn spawn_worker(
        module: &JsValue,
        memory: &JsValue,
        pointer: *mut Box<dyn FnOnce(DedicatedWorkerGlobalScope) + Send>,
    ) -> Result<web_sys::Worker, JsValue>;
}

#[doc(hidden)]
#[wasm_bindgen(js_name = __arwa_init_spawned_worker)]
pub fn init_spawned_worker(pointer: *mut Box<dyn FnOnce(DedicatedWorkerGlobalScope) + Send>) {
    let boxed = unsafe { Box::from_raw(pointer) };
    let closure = Box::into_inner(boxed);

    closure(current());
}
