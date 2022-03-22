use std::marker;

use delegate::delegate;
use js_sys::Promise;
use wasm_bindgen::JsValue;

use crate::event::impl_typed_event_traits;

mod promise_rejection_event_seal {
    pub trait Seal {}
}

pub trait PromiseRejectionEvent: promise_rejection_event_seal::Seal {
    fn promise(&self) -> Promise;

    fn reason(&self) -> JsValue;
}

pub struct RejectionHandledEvent<T> {
    inner: web_sys::PromiseRejectionEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> promise_rejection_event_seal::Seal for RejectionHandledEvent<T> {}

impl<T> PromiseRejectionEvent for RejectionHandledEvent<T> {
    delegate! {
        to self.inner {
            fn promise(&self) -> Promise;

            fn reason(&self) -> JsValue;
        }
    }
}

impl_typed_event_traits!(
    RejectionHandledEvent,
    PromiseRejectionEvent,
    "rejectionhandled"
);

pub struct UnhandledRejectionEvent<T> {
    inner: web_sys::PromiseRejectionEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> promise_rejection_event_seal::Seal for UnhandledRejectionEvent<T> {}

impl<T> PromiseRejectionEvent for UnhandledRejectionEvent<T> {
    delegate! {
        to self.inner {
            fn promise(&self) -> Promise;

            fn reason(&self) -> JsValue;
        }
    }
}

impl_typed_event_traits!(
    UnhandledRejectionEvent,
    PromiseRejectionEvent,
    "unhandledrejection"
);
