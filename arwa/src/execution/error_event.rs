use std::marker;

use delegate::delegate;

use crate::event::impl_typed_event_traits;

#[derive(Clone)]
pub struct ErrorEvent<T> {
    inner: web_sys::ErrorEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> ErrorEvent<T> {
    delegate! {
        to self.inner {
            pub fn message(&self) -> String;

            pub fn filename(&self) -> String;
        }
    }

    pub fn line_number(&self) -> u32 {
        self.inner.lineno()
    }

    pub fn column_number(&self) -> u32 {
        self.inner.colno()
    }
}

impl_typed_event_traits!(ErrorEvent, "error");
