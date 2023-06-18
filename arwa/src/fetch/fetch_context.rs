use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

use crate::fetch::{Request, Response};
use crate::type_error_wrapper;

pub(crate) mod fetch_context_seal {
    pub trait Seal {}
}

pub trait FetchContext: fetch_context_seal::Seal {
    fn fetch(&self, request: &Request) -> Fetch;
}

enum ContextInternal {
    Window(web_sys::Window),
    Worker(web_sys::WorkerGlobalScope),
}

impl ContextInternal {
    fn fetch(
        &self,
        request: &web_sys::Request,
        abort_controller: &web_sys::AbortController,
    ) -> JsFuture {
        // Per the spec, this should make a copy of the request except for the body. The beef of
        // the cost is probably in copying the headers. For the time being, I'm considering this
        // worth it, as in return we get to ignore AbortController as part of the API and have
        // drop-cancelled fetch requests.
        let mut init = web_sys::RequestInit::new();

        init.signal(Some(&abort_controller.signal()));

        let promise = match self {
            ContextInternal::Window(window) => window.fetch_with_request_and_init(request, &init),
            ContextInternal::Worker(worker) => worker.fetch_with_request_and_init(request, &init),
        };

        promise.into()
    }
}

type_error_wrapper!(NetworkError);

#[must_use = "futures do nothing unless polled or spawned"]
pub struct Fetch {
    context: ContextInternal,
    request: Option<web_sys::Request>,
    inner: Option<JsFuture>,
    abort_controller: Option<web_sys::AbortController>,
}

impl Fetch {
    pub(crate) fn window_context(context: web_sys::Window, request: web_sys::Request) -> Self {
        Fetch {
            context: ContextInternal::Window(context),
            request: Some(request),
            inner: None,
            abort_controller: None,
        }
    }

    pub(crate) fn worker_context(
        context: web_sys::WorkerGlobalScope,
        request: web_sys::Request,
    ) -> Self {
        Fetch {
            context: ContextInternal::Worker(context),
            request: Some(request),
            inner: None,
            abort_controller: None,
        }
    }
}

impl Future for Fetch {
    type Output = Result<Response, NetworkError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Initialize if uninitialized
        if let Some(request) = self.request.take() {
            let abort_controller = web_sys::AbortController::new().unwrap_throw();
            let inner = self.context.fetch(&request, &abort_controller);

            self.inner = Some(inner);
            self.abort_controller = Some(abort_controller);
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        let poll_result = inner.poll(cx);

        if poll_result.is_ready() {
            self.abort_controller = None;
        }

        poll_result
            .map_ok(|v| Response::from(v.unchecked_into::<web_sys::Response>()))
            .map_err(|e| NetworkError::new(e.unchecked_into()))
    }
}

impl Drop for Fetch {
    fn drop(&mut self) {
        if let Some(abort_controller) = self.abort_controller.take() {
            abort_controller.abort();
        }
    }
}
