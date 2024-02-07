use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod file_system_handle_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys(&self) -> &web_sys::FileSystemHandle;
    }
}

pub trait FileSystemHandle: file_system_handle_seal::Seal {
    fn name(&self) -> String {
        self.as_web_sys().name()
    }

    fn is_same_entry(&self, other: &Self) -> IsSameEntry {
        let promise = self.as_web_sys().is_same_entry(other.as_web_sys());

        IsSameEntry {
            inner: promise.into(),
        }
    }
}

#[pin_project]
pub struct IsSameEntry {
    #[pin]
    inner: JsFuture,
}

impl Future for IsSameEntry {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .inner
            .poll(cx)
            .map(|r| r.unwrap_throw() == true)
    }
}
