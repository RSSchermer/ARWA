use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use delegate::delegate;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use crate::error::ServiceWorkerRegistrationError;
use crate::event::{OnControllerChange, OnError, OnMessage, OnStateChange};

pub use web_sys::ServiceWorkerState;

pub struct ServiceWorkerDescriptor<'a, 'b> {
    script_url: &'a str,
    scope: Option<&'b str>,
}

impl<'a, 'b> ServiceWorkerDescriptor<'a, 'b> {
    pub fn new(script_url: &'a str) -> Self {
        ServiceWorkerDescriptor {
            script_url,
            scope: None,
        }
    }

    pub fn scoped(script_url: &'a str, scope: &'b str) -> Self {
        ServiceWorkerDescriptor {
            script_url,
            scope: Some(scope),
        }
    }

    pub fn script_url(&self) -> &str {
        self.script_url
    }

    pub fn set_script_url(&mut self, script_url: &'a str) {
        self.script_url = script_url;
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope
    }

    pub fn set_scope(&mut self, scope: Option<&'b str>) {
        self.scope = scope;
    }
}

pub struct ServiceWorkerContainer {
    inner: web_sys::ServiceWorkerContainer,
}

impl ServiceWorkerContainer {
    pub fn ready(&self) -> ServiceWorkerReady {
        ServiceWorkerReady {
            inner: self.inner.ready().unwrap().into(),
        }
    }

    pub fn controller(&self) -> Option<ServiceWorker> {
        self.inner.controller().map(|s| s.into())
    }

    pub fn register(&self, descriptor: ServiceWorkerDescriptor) -> ServiceWorkerRegister {
        let ServiceWorkerDescriptor { script_url, scope } = descriptor;

        let promise = if let Some(scope) = scope {
            self.inner
                .register_with_options(script_url, web_sys::RegistrationOptions::new().scope(scope))
        } else {
            self.inner.register(script_url)
        };

        ServiceWorkerRegister {
            inner: promise.into(),
        }
    }

    // Note: while get_registration make the scope url argument optional, we don't here. A scope of
    // `None` should be equivalent to `ready`.

    pub fn registration_for(&self, scope: &str) -> ServiceWorkerRegistrationFor {
        ServiceWorkerRegistrationFor {
            inner: self.inner.get_registration_with_document_url(scope).into(),
        }
    }

    // TODO: decide on naming of an equivalent for `get_registrations`

    // TODO: `start_messages` is missing in web_sys.

    pub fn on_controller_change(&self) -> OnControllerChange {
        OnControllerChange::new(self.inner.clone().into())
    }

    pub fn on_message(&self) -> OnMessage {
        OnMessage::new(self.inner.clone().into())
    }

    pub fn on_error(&self) -> OnError {
        OnError::new(self.inner.clone().into())
    }
}

impl From<web_sys::ServiceWorkerContainer> for ServiceWorkerContainer {
    fn from(inner: web_sys::ServiceWorkerContainer) -> Self {
        ServiceWorkerContainer { inner }
    }
}

impl AsRef<web_sys::ServiceWorkerContainer> for ServiceWorkerContainer {
    fn as_ref(&self) -> &web_sys::ServiceWorkerContainer {
        &self.inner
    }
}

pub struct ServiceWorkerReady {
    inner: JsFuture,
}

impl Future for ServiceWorkerReady {
    type Output = ServiceWorkerRegistration;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner)
                .poll(cx)
                .map(|result| {
                    let registration: web_sys::ServiceWorkerRegistration =
                        result.unwrap().unchecked_into();

                    registration.into()
                })
        }
    }
}

pub struct ServiceWorkerRegister {
    inner: JsFuture,
}

impl Future for ServiceWorkerRegister {
    type Output = Result<ServiceWorkerRegistration, ServiceWorkerRegistrationError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner)
                .poll(cx)
                .map_ok(|ok| {
                    let registration: web_sys::ServiceWorkerRegistration = ok.unchecked_into();

                    registration.into()
                })
                .map_err(|err| ServiceWorkerRegistrationError::new(err.unchecked_into()))
        }
    }
}

pub struct ServiceWorkerRegistrationFor {
    inner: JsFuture,
}

impl Future for ServiceWorkerRegistrationFor {
    type Output = Option<ServiceWorkerRegistration>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner)
                .poll(cx)
                .map(|result| {
                    result.ok().and_then(|ok| {
                        if ok.is_undefined() {
                            None
                        } else {
                            let registration: web_sys::ServiceWorkerRegistration =
                                ok.unchecked_into();

                            Some(registration.into())
                        }
                    })
                })
        }
    }
}

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
            inner: self.inner.update().unwrap().into(),
        }
    }

    pub fn unregister(&self) -> ServiceWorkerRegistrationUnregister {
        ServiceWorkerRegistrationUnregister {
            inner: self.inner.update().unwrap().into(),
        }
    }

    pub fn on_update_found(&self) -> OnStateChange {
        OnStateChange::new(self.inner.clone().into())
    }
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

pub struct ServiceWorkerRegistrationUpdate {
    inner: JsFuture,
}

impl Future for ServiceWorkerRegistrationUpdate {
    type Output = Result<ServiceWorkerRegistration, ServiceWorkerRegistrationError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner)
                .poll(cx)
                .map_ok(|ok| {
                    let registration: web_sys::ServiceWorkerRegistration = ok.unchecked_into();

                    registration.into()
                })
                .map_err(|err| ServiceWorkerRegistrationError::new(err.unchecked_into()))
        }
    }
}

pub struct ServiceWorkerRegistrationUnregister {
    inner: JsFuture,
}

impl Future for ServiceWorkerRegistrationUnregister {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner)
                .poll(cx)
                .map(|result| result.unwrap().as_bool().unwrap())
        }
    }
}

pub struct ServiceWorker {
    inner: web_sys::ServiceWorker,
}

impl ServiceWorker {
    delegate! {
        target self.inner {
            pub fn script_url(&self) -> String;

            pub fn state(&self) -> ServiceWorkerState;
        }
    }

    pub fn post_message(&self, message: &JsValue) {
        // No indication in the spec that this can fail if invoked without transferables
        self.inner.post_message(message).unwrap();
    }

    // TODO: post_message with transferables

    pub fn on_state_change(&self) -> OnStateChange {
        OnStateChange::new(self.inner.clone().into())
    }

    pub fn on_error(&self) -> OnError {
        OnError::new(self.inner.clone().into())
    }
}

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
