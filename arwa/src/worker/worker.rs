use std::marker;

use delegate::delegate;

use crate::event::{impl_typed_event_traits, typed_event_iterator};

pub(crate) mod worker_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait Worker: worker_seal::Seal + Sized {
    fn on_error(&self) -> OnError<Self> {
        OnError::new(self.as_web_sys_event_target())
    }
}

typed_event_iterator!(OnError, OnErrorWithOptions, ErrorEvent, "error");

#[derive(Clone)]
pub struct ErrorEvent<T> {
    inner: web_sys::ErrorEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> ErrorEvent<T> {
    delegate! {
        target self.inner {
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
