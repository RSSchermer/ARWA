use std::marker;

use wasm_bindgen::UnwrapThrowExt;

use crate::event::{
    impl_event_target_traits, impl_try_from_event_target, impl_typed_event_traits,
    typed_event_iterator,
};
use crate::message::{message_sender_seal, MessageSender};
use crate::url::Url;
use crate::worker::{worker_seal, Worker};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ServiceWorkerState {
    Parsed,
    Installing,
    Installed,
    Activating,
    Activated,
    Redundant,
}

impl ServiceWorkerState {
    fn from_web_sys(service_worker_state: web_sys::ServiceWorkerState) -> Self {
        match service_worker_state {
            web_sys::ServiceWorkerState::Parsed => ServiceWorkerState::Parsed,
            web_sys::ServiceWorkerState::Installing => ServiceWorkerState::Installing,
            web_sys::ServiceWorkerState::Installed => ServiceWorkerState::Installed,
            web_sys::ServiceWorkerState::Activating => ServiceWorkerState::Activating,
            web_sys::ServiceWorkerState::Activated => ServiceWorkerState::Activated,
            web_sys::ServiceWorkerState::Redundant => ServiceWorkerState::Redundant,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct ServiceWorker {
    inner: web_sys::ServiceWorker,
}

impl ServiceWorker {
    pub fn script_url(&self) -> Url {
        Url::parse(self.inner.script_url().as_ref()).unwrap_throw()
    }

    pub fn state(&self) -> ServiceWorkerState {
        ServiceWorkerState::from_web_sys(self.inner.state())
    }

    pub fn on_state_change(&self) -> OnStateChange<Self> {
        OnStateChange::new(self.inner.as_ref())
    }
}

impl worker_seal::Seal for ServiceWorker {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl Worker for ServiceWorker {}

impl message_sender_seal::Seal for ServiceWorker {}

impl MessageSender for ServiceWorker {}

impl From<web_sys::ServiceWorker> for ServiceWorker {
    fn from(inner: web_sys::ServiceWorker) -> Self {
        ServiceWorker { inner }
    }
}

impl AsRef<web_sys::ServiceWorker> for ServiceWorker {
    fn as_ref(&self) -> &web_sys::ServiceWorker {
        &self.inner
    }
}

impl_event_target_traits!(ServiceWorker);
impl_try_from_event_target!(ServiceWorker);

#[derive(Clone)]
pub struct StateChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(StateChangeEvent, Event, "statechange");

typed_event_iterator!(
    OnStateChange,
    OnStateChangeWithOptions,
    StateChangeEvent,
    "statechange"
);
