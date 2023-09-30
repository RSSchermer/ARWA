use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

#[must_use = "futures do nothing unless polled or spawned"]
pub(super) struct RequestFuture {
    request: web_sys::IdbRequest,
    wake_up_callback: Option<Closure<dyn FnMut(web_sys::Event)>>,
    waker: Option<Waker>,
}

impl RequestFuture {
    pub(super) fn from_request(request: web_sys::IdbRequest) -> Self {
        RequestFuture {
            request,
            wake_up_callback: None,
            waker: None,
        }
    }
}

impl Future for RequestFuture {
    type Output = Result<JsValue, web_sys::DomException>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Check the ready-state first, before we potentially initialize the wake-up mechanism, as
        // the request may already have completed before the first time it gets polled
        if self.request.ready_state() == web_sys::IdbRequestReadyState::Done {
            if let Some(error) = self.request.error().unwrap_throw() {
                return Poll::Ready(Err(error));
            }

            return Poll::Ready(Ok(self.request.result().unwrap_throw()));
        }

        // Initialize wake-up mechanism
        if self.wake_up_callback.is_none() {
            self.waker = Some(cx.waker().clone());

            let self_ptr = (&mut *self) as *mut RequestFuture;

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

impl Drop for RequestFuture {
    fn drop(&mut self) {
        if self.wake_up_callback.is_some() {
            // Was initialized
            self.request.set_onsuccess(None);
            self.request.set_onerror(None);
        }
    }
}
