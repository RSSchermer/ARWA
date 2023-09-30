use std::future::Future;
use std::marker;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{throw_str, JsCast, UnwrapThrowExt};
use web_sys::DedicatedWorkerGlobalScope;

use crate::indexed_db::{transaction_capability, Database, Transaction, UpgradeTransaction};
use crate::window::Window;
use crate::worker::service::ServiceWorkerGlobalScope;
use crate::worker::shared::SharedWorkerGlobalScope;
use crate::{dom_exception_wrapper, impl_common_wrapper_traits};

mod indexed_db_context_seal {
    pub trait Seal {}
}

pub trait IndexedDbContext: indexed_db_context_seal::Seal {
    fn indexed_db(&self) -> Factory;
}

impl indexed_db_context_seal::Seal for Window {}
impl IndexedDbContext for Window {
    fn indexed_db(&self) -> Factory {
        let window: &web_sys::Window = self.as_ref();

        // I can't find any indication in the spec that is can actually error or be None
        let inner = window.indexed_db().unwrap_throw().unwrap_throw();

        Factory { inner }
    }
}

impl indexed_db_context_seal::Seal for DedicatedWorkerGlobalScope {}
impl IndexedDbContext for DedicatedWorkerGlobalScope {
    fn indexed_db(&self) -> Factory {
        let scope: &web_sys::WorkerGlobalScope = self.as_ref();

        // I can't find any indication in the spec that is can actually error or be None
        let inner = scope.indexed_db().unwrap_throw().unwrap_throw();

        Factory { inner }
    }
}

impl indexed_db_context_seal::Seal for SharedWorkerGlobalScope {}
impl IndexedDbContext for SharedWorkerGlobalScope {
    fn indexed_db(&self) -> Factory {
        let scope: &web_sys::SharedWorkerGlobalScope = self.as_ref();

        // I can't find any indication in the spec that is can actually error or be None
        let inner = scope.indexed_db().unwrap_throw().unwrap_throw();

        Factory { inner }
    }
}

impl indexed_db_context_seal::Seal for ServiceWorkerGlobalScope {}
impl IndexedDbContext for ServiceWorkerGlobalScope {
    fn indexed_db(&self) -> Factory {
        let scope: &web_sys::ServiceWorkerGlobalScope = self.as_ref();

        // I can't find any indication in the spec that is can actually error or be None
        let inner = scope.indexed_db().unwrap_throw().unwrap_throw();

        Factory { inner }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DatabaseVersion {
    Number(u32),
    Default,
}

impl Default for DatabaseVersion {
    fn default() -> Self {
        DatabaseVersion::Default
    }
}

dom_exception_wrapper!(OpenDatabaseError);
dom_exception_wrapper!(DeleteDatabaseError);

dom_exception_wrapper!(RequestOpenDatabaseError);
dom_exception_wrapper!(RequestDeleteDatabaseError);

pub fn default_upgrade_needed_callback(_: UpgradeTransaction, _: u32, _: u32) {}
pub fn default_blocked_callback() {}

pub struct OpenDataBaseRequest<'a, UpgradeNeeded, Blocked> {
    pub name: &'a str,
    pub version: DatabaseVersion,
    pub upgrade_needed: UpgradeNeeded,
    pub blocked: Blocked,
}

pub struct Factory {
    inner: web_sys::IdbFactory,
}

impl Factory {
    pub fn open_database<UpgradeNeeded, Blocked>(
        &self,
        request: OpenDataBaseRequest<UpgradeNeeded, Blocked>,
    ) -> Result<impl Future<Output = Result<Database, RequestOpenDatabaseError>>, OpenDatabaseError>
    where
        UpgradeNeeded: FnOnce(UpgradeTransaction, u32, u32) + 'static,
        Blocked: FnOnce() + 'static,
    {
        let OpenDataBaseRequest {
            name,
            version,
            upgrade_needed,
            blocked,
        } = request;

        let request = match version {
            DatabaseVersion::Number(n) => {
                if n == 0 {
                    throw_str(
                        "If an explicit version number is given, it must be greater than `0`",
                    );
                }

                self.inner.open_with_u32(name, n)
            }
            DatabaseVersion::Default => self.inner.open(name),
        };

        let request = request.map_err(|err| OpenDatabaseError::new(err.unchecked_into()))?;

        let request_clone = request.clone();

        let upgrade_needed_callback = move |e: web_sys::IdbVersionChangeEvent| {
            let old_version = e.old_version() as u32;
            // TODO: I'm not entirely sure if this can end up being triggered due to a database
            // deletion, in which case new_version would return None
            let new_version = e.new_version().unwrap_throw() as u32;

            let transaction = request_clone.transaction().unwrap_throw();
            let database = request_clone.result().unwrap_throw().unchecked_into();

            upgrade_needed(
                Transaction::upgrade(transaction, database),
                old_version,
                new_version,
            )
        };
        let upgrade_needed_callback = Closure::once(upgrade_needed_callback);

        let blocked_callback = move |_: web_sys::Event| blocked();
        let blocked_callback = Closure::once(blocked_callback);

        Ok(OpenDatabase::init(
            request,
            upgrade_needed_callback,
            blocked_callback,
        ))
    }

    pub fn delete_database(
        &self,
        name: &str,
    ) -> Result<impl Future<Output = Result<(), RequestDeleteDatabaseError>>, DeleteDatabaseError>
    {
        let request = self
            .inner
            .delete_database(name)
            .map_err(|err| DeleteDatabaseError::new(err.unchecked_into()))?;

        Ok(DeleteDatabase::init(request))
    }
}

impl_common_wrapper_traits!(Factory);

#[must_use = "futures do nothing unless polled or spawned"]
struct OpenDatabase {
    request: web_sys::IdbOpenDbRequest,
    upgrade_needed_callback: Closure<dyn FnMut(web_sys::IdbVersionChangeEvent)>,
    blocked_callback: Closure<dyn FnMut(web_sys::Event)>,
    wake_up_callback: Option<Closure<dyn FnMut(web_sys::Event)>>,
    waker: Option<Waker>,
}

impl OpenDatabase {
    fn init(
        request: web_sys::IdbOpenDbRequest,
        upgrade_needed_callback: Closure<dyn FnMut(web_sys::IdbVersionChangeEvent)>,
        blocked_callback: Closure<dyn FnMut(web_sys::Event)>,
    ) -> Self {
        request.set_onupgradeneeded(Some(upgrade_needed_callback.as_ref().unchecked_ref()));
        request.set_onblocked(Some(blocked_callback.as_ref().unchecked_ref()));

        OpenDatabase {
            request,
            upgrade_needed_callback,
            blocked_callback,
            wake_up_callback: None,
            waker: None,
        }
    }
}

impl Future for OpenDatabase {
    type Output = Result<Database, RequestOpenDatabaseError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check the ready-state first, before we potentially initialize the wake-up mechanism, as
        // the request may already have completed before the first time it gets polled
        if self.request.ready_state() == web_sys::IdbRequestReadyState::Done {
            if let Some(error) = self.request.error().unwrap_throw() {
                return Poll::Ready(Err(RequestOpenDatabaseError::new(error)));
            }

            let result = self.request.result().unwrap_throw();
            let database = Database {
                inner: result.unchecked_into(),
            };

            return Poll::Ready(Ok(database));
        }

        // Initialize wake-up mechanism
        if self.wake_up_callback.is_none() {
            self.waker = Some(cx.waker().clone());

            let self_ptr = (&mut *self) as *mut OpenDatabase;

            let wake_up_callback = move |_: web_sys::Event| {
                // SAFETY: safe because this is pinned
                let this = unsafe { &mut *self_ptr };

                if let Some(waker) = this.waker.take() {
                    waker.wake();
                }
            };
            let wake_up_callback = Closure::once(wake_up_callback);

            self.request
                .set_onsuccess(Some(wake_up_callback.as_ref().unchecked_ref()));
            self.request
                .set_onerror(Some(wake_up_callback.as_ref().unchecked_ref()));

            // Hold on to closure until the future drops so that callbacks remain valid
            self.wake_up_callback = Some(wake_up_callback);
        }

        Poll::Pending
    }
}

impl Drop for OpenDatabase {
    fn drop(&mut self) {
        self.request.set_onupgradeneeded(None);
        self.request.set_onblocked(None);

        if self.wake_up_callback.is_some() {
            // Was initialized
            self.request.set_onsuccess(None);
            self.request.set_onerror(None);
        }
    }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct DeleteDatabase {
    request: web_sys::IdbOpenDbRequest,
    wake_up_callback: Option<Closure<dyn FnMut(web_sys::Event)>>,
    waker: Option<Waker>,
}

impl DeleteDatabase {
    fn init(request: web_sys::IdbOpenDbRequest) -> Self {
        DeleteDatabase {
            request,
            wake_up_callback: None,
            waker: None,
        }
    }
}

impl Future for DeleteDatabase {
    type Output = Result<(), RequestDeleteDatabaseError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check the ready-state first, before we potentially initialize the wake-up mechanism, as
        // the request may already have completed before the first time it gets polled
        if self.request.ready_state() == web_sys::IdbRequestReadyState::Done {
            if let Some(error) = self.request.error().unwrap_throw() {
                return Poll::Ready(Err(RequestDeleteDatabaseError::new(error)));
            }

            return Poll::Ready(Ok(()));
        }

        // Initialize wake-up mechanism
        if self.wake_up_callback.is_none() {
            self.waker = Some(cx.waker().clone());

            let self_ptr = (&mut *self) as *mut DeleteDatabase;

            let wake_up_callback = move |_: web_sys::Event| {
                // SAFETY: safe because this is pinned
                let this = unsafe { &mut *self_ptr };

                if let Some(waker) = this.waker.take() {
                    waker.wake();
                }
            };
            let wake_up_callback = Closure::once(wake_up_callback);

            self.request
                .set_onsuccess(Some(wake_up_callback.as_ref().unchecked_ref()));
            self.request
                .set_onerror(Some(wake_up_callback.as_ref().unchecked_ref()));

            // Hold on to closure until the future drops so that callbacks remain valid
            self.wake_up_callback = Some(wake_up_callback);
        }

        Poll::Pending
    }
}

impl Drop for DeleteDatabase {
    fn drop(&mut self) {
        if self.wake_up_callback.is_some() {
            // Was initialized
            self.request.set_onsuccess(None);
            self.request.set_onerror(None);
        }
    }
}
