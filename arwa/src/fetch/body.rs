use crate::file::Blob;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[derive(Clone, Copy, Debug)]
pub enum BodySource<'a> {
    String(&'a str),
    Blob(&'a Blob),
    Bytes(&'a [u8]),
}

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
        let text = match &self.internal {
            Internal::Request(r) => r.text(),
            Internal::Response(r) => r.text(),
        };

        BodyToString {
            // Note: no indication in the spec that this can actually fail at this step, instead
            // the promise will reject if the body has already been "disturbed".
            inner: text.unwrap_throw().into(),
        }
    }

    pub fn to_object(&self) -> BodyToObject {
        let json = match &self.internal {
            Internal::Request(r) => r.json(),
            Internal::Response(r) => r.json(),
        };

        BodyToObject {
            // Note: no indication in the spec that this can actually fail at this step, instead
            // the promise will reject if the body has already been "disturbed".
            inner: json.unwrap_throw().into(),
        }
    }

    pub fn to_buffer(&self) -> BodyToBuffer {
        let array_buffer = match &self.internal {
            Internal::Request(r) => r.array_buffer(),
            Internal::Response(r) => r.array_buffer(),
        };

        BodyToBuffer {
            // Note: no indication in the spec that this can actually fail at this step, instead
            // the promise will reject if the body has already been "disturbed".
            inner: array_buffer.unwrap_throw().into(),
        }
    }

    pub fn to_blob(&self) -> BodyToBlob {
        let blob = match &self.internal {
            Internal::Request(r) => r.array_buffer(),
            Internal::Response(r) => r.array_buffer(),
        };

        BodyToBlob {
            // Note: no indication in the spec that this can actually fail at this step, instead
            // the promise will reject if the body has already been "disturbed".
            inner: blob.unwrap_throw().into(),
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
    inner: JsFuture,
}

impl Future for BodyToString {
    type Output = Result<String, ConsumeBodyError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner
            .poll(cx)
            .map_ok(|v| v.as_string().unwrap_or(String::new()))
            .map_err(|err| ConsumeBodyError::new(err.unchecked_into()))
    }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct BodyToObject {
    inner: JsFuture,
}

impl Future for BodyToObject {
    type Output = Result<js_sys::Object, ConsumeBodyError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner
            .poll(cx)
            .map_ok(|v| v.unchecked_into())
            .map_err(|err| ConsumeBodyError::new(err.unchecked_into()))
    }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct BodyToBuffer {
    inner: JsFuture,
}

impl Future for BodyToBuffer {
    type Output = Result<js_sys::ArrayBuffer, ConsumeBodyError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner
            .poll(cx)
            .map_ok(|v| v.unchecked_into())
            .map_err(|err| ConsumeBodyError::new(err.unchecked_into()))
    }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct BodyToBlob {
    inner: JsFuture,
}

impl Future for BodyToBlob {
    type Output = Result<Blob, ConsumeBodyError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner
            .poll(cx)
            .map_ok(|v| Blob::from(v.unchecked_into()))
            .map_err(|err| ConsumeBodyError::new(err.unchecked_into()))
    }
}

#[derive(Clone)]
pub struct ConsumeBodyError {
    inner: js_sys::TypeError,
}

impl ConsumeBodyError {
    fn new(inner: js_sys::TypeError) -> Self {
        ConsumeBodyError { inner }
    }
}
