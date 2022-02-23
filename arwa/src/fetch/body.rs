use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

use crate::file::Blob;
use crate::type_error_wrapper;

#[derive(Clone, Copy)]
pub enum BodySource<'a> {
    String(&'a str),
    Blob(&'a Blob),
    Bytes(&'a [u8]),
}

type_error_wrapper!(ConsumeBodyError);

#[derive(Clone)]
enum Internal {
    Request(web_sys::Request),
    Response(web_sys::Response),
}

#[derive(Clone)]
pub struct Body {
    internal: Internal,
}

impl Body {
    pub(crate) fn request(request: web_sys::Request) -> Self {
        Body {
            internal: Internal::Request(request),
        }
    }

    pub(crate) fn response(response: web_sys::Response) -> Self {
        Body {
            internal: Internal::Response(response),
        }
    }

    pub fn is_used(&self) -> bool {
        match &self.internal {
            Internal::Request(r) => r.body_used(),
            Internal::Response(r) => r.body_used(),
        }
    }

    pub fn to_string(&self) -> BodyToString {
        BodyToString {
            init: Some(self.internal.clone()),
            inner: None,
        }
    }

    pub fn to_object(&self) -> BodyToObject {
        BodyToObject {
            init: Some(self.internal.clone()),
            inner: None,
        }
    }

    pub fn to_buffer(&self) -> BodyToBuffer {
        BodyToBuffer {
            init: Some(self.internal.clone()),
            inner: None,
        }
    }

    pub fn to_blob(&self) -> BodyToBlob {
        BodyToBlob {
            init: Some(self.internal.clone()),
            inner: None,
        }
    }

    // TODO: FormData. Also curious if calling `json` would return a `FormData` object if the
    // response has a form-data mime-type.

    // pub fn to_byte_stream(&self) -> Option<BodyToByteStream> {
    //     todo!()
    // }

    // pub fn to_vec(&self) -> ResponseBodyToVec {
    //     todo!()
    // }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct BodyToString {
    init: Option<Internal>,
    inner: Option<JsFuture>,
}

impl Future for BodyToString {
    type Output = Result<String, ConsumeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Initialize
        if let Some(init) = self.init.take() {
            let res = match init {
                Internal::Request(r) => r.text(),
                Internal::Response(r) => r.text(),
            };

            // Note: no indication in the spec that this can actually fail at this step, instead
            // the promise will reject if the body has already been "disturbed".
            self.inner = Some(res.unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|v| v.as_string().unwrap_or(String::new()))
            .map_err(|err| ConsumeBodyError::new(err.unchecked_into()))
    }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct BodyToObject {
    init: Option<Internal>,
    inner: Option<JsFuture>,
}

impl Future for BodyToObject {
    type Output = Result<js_sys::Object, ConsumeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Initialize
        if let Some(init) = self.init.take() {
            let res = match init {
                Internal::Request(r) => r.json(),
                Internal::Response(r) => r.json(),
            };

            // Note: no indication in the spec that this can actually fail at this step, instead
            // the promise will reject if the body has already been "disturbed".
            self.inner = Some(res.unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|v| v.unchecked_into())
            .map_err(|err| ConsumeBodyError::new(err.unchecked_into()))
    }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct BodyToBuffer {
    init: Option<Internal>,
    inner: Option<JsFuture>,
}

impl Future for BodyToBuffer {
    type Output = Result<js_sys::ArrayBuffer, ConsumeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Initialize
        if let Some(init) = self.init.take() {
            let res = match init {
                Internal::Request(r) => r.array_buffer(),
                Internal::Response(r) => r.array_buffer(),
            };

            // Note: no indication in the spec that this can actually fail at this step, instead
            // the promise will reject if the body has already been "disturbed".
            self.inner = Some(res.unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|v| v.unchecked_into())
            .map_err(|err| ConsumeBodyError::new(err.unchecked_into()))
    }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct BodyToBlob {
    init: Option<Internal>,
    inner: Option<JsFuture>,
}

impl Future for BodyToBlob {
    type Output = Result<Blob, ConsumeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Initialize
        if let Some(init) = self.init.take() {
            let res = match init {
                Internal::Request(r) => r.blob(),
                Internal::Response(r) => r.blob(),
            };

            // Note: no indication in the spec that this can actually fail at this step, instead
            // the promise will reject if the body has already been "disturbed".
            self.inner = Some(res.unwrap_throw().into());
        }

        let inner = Pin::new(self.inner.as_mut().unwrap_throw());

        inner
            .poll(cx)
            .map_ok(|v| Blob::from(v.unchecked_into::<web_sys::Blob>()))
            .map_err(|err| ConsumeBodyError::new(err.unchecked_into()))
    }
}
