use std::future::Future;
use std::marker;
use std::pin::Pin;
use std::task::{Context, Poll};

use delegate::delegate;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

use crate::dom_exception_wrapper;
use crate::event::{
    impl_event_target_traits, impl_try_from_event_target, impl_typed_event_traits,
    typed_event_iterator,
};
use crate::worker::service::ServiceWorker;

dom_exception_wrapper!(ServiceWorkerRegistrationError);

pub struct ServiceWorkerRegistration {
    inner: web_sys::ServiceWorkerRegistration,
}

impl ServiceWorkerRegistration {
    delegate! {
        target self.inner {
            pub fn scope(&self) -> String;
        }
    }

    pub fn installing(&self) -> Option<ServiceWorker> {
        self.inner.installing().map(|s| s.into())
    }

    pub fn waiting(&self) -> Option<ServiceWorker> {
        self.inner.waiting().map(|s| s.into())
    }

    pub fn active(&self) -> Option<ServiceWorker> {
        self.inner.active().map(|s| s.into())
    }

    pub fn update(&self) -> ServiceWorkerRegistrationUpdate {
        ServiceWorkerRegistrationUpdate {
            registration: Some(self.inner.clone()),
            inner: None,
        }
    }

    pub fn unregister(&self) -> ServiceWorkerRegistrationUnregister {
        ServiceWorkerRegistrationUnregister {
            registration: Some(self.inner.clone()),
            inner: None,
        }
    }

    pub fn on_update_found(&self) -> OnUpdateFound<Self> {
        OnUpdateFound::new(self.inner.as_ref())
    }

    // Ignore ContextIndex, PushManager and NavigationPreload for now, they don't seem well
    // supported outside of Chromium currently.
}

impl From<web_sys::ServiceWorkerRegistration> for ServiceWorkerRegistration {
    fn from(inner: web_sys::ServiceWorkerRegistration) -> Self {
        ServiceWorkerRegistration { inner }
    }
}

impl AsRef<web_sys::ServiceWorkerRegistration> for ServiceWorkerRegistration {
    fn as_ref(&self) -> &web_sys::ServiceWorkerRegistration {
        &self.inner
    }
}

impl_event_target_traits!(ServiceWorkerRegistration);
impl_try_from_event_target!(ServiceWorkerRegistration);

#[must_use = "futures do nothing unless polled or spawned."]
pub struct ServiceWorkerRegistrationUpdate {
    registration: Option<web_sys::ServiceWorkerRegistration>,
    inner: Option<JsFuture>,
}

impl Future for ServiceWorkerRegistrationUpdate {
    type Output = Result<ServiceWorkerRegistration, ServiceWorkerRegistrationError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Initialize
        if let Some(registration) = self.registration.take() {
            self.inner = Some(registration.update().unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|ok| {
                let registration: web_sys::ServiceWorkerRegistration = ok.unchecked_into();

                registration.into()
            })
            .map_err(|err| ServiceWorkerRegistrationError::new(err.unchecked_into()))
    }
}

#[must_use = "futures do nothing unless polled or spawned."]
pub struct ServiceWorkerRegistrationUnregister {
    registration: Option<web_sys::ServiceWorkerRegistration>,
    inner: Option<JsFuture>,
}

impl Future for ServiceWorkerRegistrationUnregister {
    type Output = bool;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Initialize
        if let Some(registration) = self.registration.take() {
            self.inner = Some(registration.unregister().unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map(|result| result.unwrap_throw().as_bool().unwrap_throw())
    }
}

#[derive(Clone)]
pub struct UpdateFoundEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(UpdateFoundEvent, Event, "updatefound");

typed_event_iterator!(
    OnUpdateFound,
    OnUpdateFoundWithOptions,
    UpdateFoundEvent,
    "updatefound"
);
