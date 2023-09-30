use std::marker;

use js_sys::Array;
use wasm_bindgen::{JsCast, JsValue};

use crate::dom_exception_wrapper;
use crate::event::{impl_typed_event_traits, typed_event_iterator};
use crate::indexed_db::{ReadOnlyTransaction, ReadWriteTransaction, Transaction};

dom_exception_wrapper!(CreateTransactionError);

pub struct Database {
    pub(super) inner: web_sys::IdbDatabase,
}

impl Database {
    pub fn name(&self) -> String {
        self.inner.name()
    }

    pub fn version(&self) -> u32 {
        self.inner.version() as u32
    }

    pub fn transaction<I>(
        &self,
        store_names: I,
    ) -> Result<ReadOnlyTransaction, CreateTransactionError>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let store_names = Array::from_iter(
            store_names
                .into_iter()
                .map(|n| JsValue::from_str(n.as_ref())),
        );

        self.inner
            .transaction_with_str_sequence_and_mode(
                store_names.as_ref(),
                web_sys::IdbTransactionMode::Readonly,
            )
            .map(|t| Transaction::read_only(t))
            .map_err(|err| CreateTransactionError::new(err.unchecked_into()))
    }

    pub fn transaction_rw<I>(
        &self,
        store_names: I,
    ) -> Result<ReadWriteTransaction, CreateTransactionError>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let store_names = Array::from_iter(
            store_names
                .into_iter()
                .map(|n| JsValue::from_str(n.as_ref())),
        );

        self.inner
            .transaction_with_str_sequence_and_mode(
                store_names.as_ref(),
                web_sys::IdbTransactionMode::Readwrite,
            )
            .map(|t| Transaction::read_write(t))
            .map_err(|err| CreateTransactionError::new(err.unchecked_into()))
    }

    pub fn close(self) {
        self.inner.close();
    }

    pub fn on_close(&self) -> OnClose<Self> {
        OnClose::new(self.inner.as_ref())
    }

    pub fn on_version_change(&self) -> OnVersionChange<Self> {
        OnVersionChange::new(self.inner.as_ref())
    }
}

#[derive(Clone)]
pub struct CloseEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(CloseEvent, Event, "close");

#[derive(Clone)]
pub struct VersionChangeEvent<T> {
    inner: web_sys::IdbVersionChangeEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> VersionChangeEvent<T> {
    pub fn old_version(&self) -> u32 {
        self.inner.old_version() as u32
    }

    pub fn new_version(&self) -> Option<u32> {
        self.inner.new_version().map(|v| v as u32)
    }
}

impl_typed_event_traits!(VersionChangeEvent, IdbVersionChangeEvent, "versionchange");

typed_event_iterator!(OnClose, OnCloseWithOptions, CloseEvent, "close");
typed_event_iterator!(
    OnVersionChange,
    OnVersionChangeWithOptions,
    VersionChangeEvent,
    "versionchange"
);
