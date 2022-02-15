use crate::message::{message_event_target_seal, MessageEventTarget};
use crate::url::AbsoluteOrRelativeUrl;
use crate::worker::service::{ServiceWorker, ServiceWorkerRegistration};
use crate::worker::WorkerType;
use std::future::Future;
use std::marker;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpdateViaCache {
    All,
    Imports,
    None,
}

impl Default for UpdateViaCache {
    fn default() -> Self {
        UpdateViaCache::Imports
    }
}

pub struct ServiceWorkerOptions<'a> {
    pub scope: Option<AbsoluteOrRelativeUrl<'a>>,
    pub worker_type: WorkerType,
    pub update_via_cache: UpdateViaCache,
}

impl Default for ServiceWorkerOptions<'_> {
    fn default() -> Self {
        ServiceWorkerOptions {
            scope: None,
            worker_type: WorkerType::default(),
            update_via_cache: UpdateViaCache::default(),
        }
    }
}

#[derive(Clone)]
pub struct ServiceWorkerContainer {
    inner: web_sys::ServiceWorkerContainer,
}

impl ServiceWorkerContainer {
    pub fn ready(&self) -> ServiceWorkerReady {
        ServiceWorkerReady {
            container: Some(self.inner.clone()),
            inner: None,
        }
    }

    pub fn controller(&self) -> Option<ServiceWorker> {
        self.inner.controller().map(|s| s.into())
    }

    pub fn register(
        &self,
        script_url: AbsoluteOrRelativeUrl,
        options: ServiceWorkerOptions,
    ) -> ServiceWorkerRegister {
        let ServiceWorkerOptions {
            scope,
            worker_type,
            update_via_cache,
        } = options;

        let mut opts = web_sys::RegistrationOptions::new();

        if let Some(scope) = scope {
            opts.scope(scope.as_ref());
        }

        match update_via_cache {
            UpdateViaCache::None => {
                opts.update_via_cache(web_sys::ServiceWorkerUpdateViaCache::None)
            }
            UpdateViaCache::Imports => {
                opts.update_via_cache(web_sys::ServiceWorkerUpdateViaCache::Imports)
            }
            UpdateViaCache::All => opts.update_via_cache(web_sys::ServiceWorkerUpdateViaCache::All),
        }

        match worker_type {
            WorkerType::Classic => (),
            WorkerType::Module => todo!("missing in web-sys"),
        }

        ServiceWorkerRegister {
            init: Some(RegisterInit {
                container: self.inner.clone(),
                script_url: script_url.to_string(),
                opts,
            }),
            inner: None,
        }
    }

    // Note: while get_registration make the scope url argument optional, we don't here. A scope of
    // `None` should be equivalent to `ready`.

    pub fn registration_for(&self, scope: AbsoluteOrRelativeUrl) -> ServiceWorkerRegistrationFor {
        ServiceWorkerRegistrationFor {
            init: Some(RegistrationForInit {
                container: self.inner.clone(),
                scope: scope.to_string(),
            }),
            inner: None,
        }
    }

    pub fn registrations(&self) -> ServiceWorkerRegistrations {
        ServiceWorkerRegistrations {
            container: Some(self.inner.clone()),
            inner: None,
        }
    }

    // TODO: `start_messages` is missing in web_sys.

    pub fn on_controller_change(&self) -> OnControllerChange<Self> {
        OnControllerChange::new(self.inner.clone().into())
    }
}

impl message_event_target_seal::Seal for ServiceWorkerContainer {
    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.as_ref()
    }
}

impl MessageEventTarget for ServiceWorkerContainer {}

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

impl_event_target_traits!(ServiceWorker);
impl_try_from_event_targets!(ServiceWorker, web_sys::ServiceWorker);

#[must_use = "a future does nothing unless polled or spawned"]
pub struct ServiceWorkerReady {
    container: Option<web_sys::ServiceWorkerContainer>,
    inner: Option<JsFuture>,
}

impl Future for ServiceWorkerReady {
    type Output = ServiceWorkerRegistration;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(container) = self.container.take() {
            // Initialize
            self.inner = Some(container.ready().unwrap().into())
        }

        self.inner.as_mut().unwrap().poll(cx).map(|result| {
            let registration: web_sys::ServiceWorkerRegistration = result.unwrap().unchecked_into();

            registration.into()
        })
    }
}

struct RegisterInit {
    container: web_sys::ServiceWorkerContainer,
    script_url: String,
    opts: web_sys::RegistrationOptions,
}

#[must_use = "a future does nothing unless polled or spawned"]
pub struct ServiceWorkerRegister {
    init: Option<RegisterInit>,
    inner: Option<JsFuture>,
}

impl Future for ServiceWorkerRegister {
    type Output = Result<ServiceWorkerRegistration, ServiceWorkerRegistrationError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let RegisterInit {
                container,
                script_url,
                opts,
            } = init;

            self.inner = Some(container.register_with_options(&script_url, &opts));
        }

        self.inner
            .as_mut()
            .unwrap()
            .poll(cx)
            .map_ok(|ok| {
                let registration: web_sys::ServiceWorkerRegistration = ok.unchecked_into();

                registration.into()
            })
            .map_err(|err| ServiceWorkerRegistrationError::new(err.unchecked_into()))
    }
}

struct RegistrationForInit {
    container: web_sys::ServiceWorkerContainer,
    scope: String,
}
#[must_use = "a future does nothing unless polled or spawned"]
pub struct ServiceWorkerRegistrationFor {
    init: Option<RegistrationForInit>,
    inner: Option<JsFuture>,
}

impl Future for ServiceWorkerRegistrationFor {
    type Output = Option<ServiceWorkerRegistration>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let RegistrationForInit { container, scope } = init;

            self.inner = Some(container.get_registration_with_document_url(scope).into())
        }

        self.inner.as_mut().unwrap().poll(cx).map(|result| {
            result.ok().and_then(|ok| {
                if ok.is_undefined() {
                    None
                } else {
                    let registration: web_sys::ServiceWorkerRegistration = ok.unchecked_into();

                    Some(registration.into())
                }
            })
        })
    }
}

#[must_use = "a future does nothing unless polled or spawned"]
pub struct ServiceWorkerRegistrations {
    container: Option<web_sys::ServiceWorkerContainer>,
    inner: Option<JsFuture>,
}

impl Future for ServiceWorkerRegistrations {
    type Output = ServiceWorkers;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(container) = self.container.take() {
            self.inner = Some(container.get_registrations().into())
        }

        self.inner.as_mut().unwrap().poll(cx).map(|result| {
            let inner = result.unwrap_or_else(|_| js_sys::Array::new());

            ServiceWorkers { inner }
        })
    }
}

unchecked_cast_array!(ServiceWorker, web_sys::ServiceWorker, ServiceWorkers);

#[derive(Clone)]
pub struct ControllerChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(ControllerChangeEvent, web_sys::Event, "controllerchange");

typed_event_stream!(
    OnControllerChange,
    OnControllerChangeWithOptions,
    ControllerChangeEvent,
    "controllerchange"
);
