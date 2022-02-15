use crate::fetch::{Body, BodySource, Headers, Status};
use crate::file::Blob;
use crate::url::{AbsoluteOrRelativeUrl, Url};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use url::Url;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;

pub enum ResponseType {
    Default,
    Basic,
    CORS,
    Error,
    Opaque,
    OpaqueRedirect,
}

pub struct ResponseDescriptor<'a> {
    pub status: Status,
    pub status_text: &'a str,
    pub headers: Option<&'a Headers>,
    pub body: Option<BodySource<'a>>,
}

impl Default for ResponseDescriptor {
    fn default() -> Self {
        ResponseDescriptor {
            status: Status::OK,
            status_text: "",
            headers: None,
            body: None,
        }
    }
}

#[derive(Clone)]
pub struct ResponseInitError {
    inner: js_sys::TypeError,
}

impl ResponseInitError {
    fn new(inner: js_sys::TypeError) -> Self {
        ResponseInitError { inner }
    }
}

pub struct Response {
    inner: web_sys::Response,
}

impl Response {
    fn init(&self, descriptor: ResponseDescriptor) -> Response {
        create_response_internal(descriptor).unwrap_throw()
    }

    fn try_init(&self, descriptor: ResponseDescriptor) -> Result<Response, ResponseInitError> {
        create_response_internal(descriptor)
            .map_err(|err| ResponseInitError::new(err.unchecked_into()))
    }

    fn redirect<T>(url: T, status: Status) -> Response
    where
        T: AbsoluteOrRelativeUrl,
    {
        web_sys::Response::redirect_with_status(url.as_str(), status)
            .unwrap_throw()
            .into()
    }

    delegate! {
        to self.inner {
            pub fn status(&self) -> u16;

            pub fn status_text(&self) -> String;
        }
    }

    pub fn is_ok(&self) -> bool {
        self.inner.ok()
    }

    pub fn is_redirected(&self) -> bool {
        self.inner.redirected()
    }

    pub fn response_type(&self) -> ResponseType {
        match self.inner.type_().as_ref() {
            "basic" => ResponseType::Basic,
            "cors" => ResponseType::CORS,
            "error" => ResponseType::Error,
            "opaque" => ResponseType::Opaque,
            "opaqueredirect" => ResponseType::OpaqueRedirect,
            _ => ResponseType::Default,
        }
    }

    pub fn url(&self) -> Option<Url> {
        let url = self.inner.url();

        if url.is_empty() {
            None
        } else {
            // Assume URL is always valid for now (until counter-example).
            Some(Url::parse(&url).unwrap())
        }
    }

    pub fn headers(&self) -> Headers {
        self.inner.headers().into()
    }

    pub fn body(&self) -> Body {
        Body::response(self.inner.clone())
    }
}

impl From<web_sys::Response> for Response {
    fn from(inner: web_sys::Response) -> Self {
        Response { inner }
    }
}

impl AsRef<web_sys::Response> for Response {
    fn as_ref(&self) -> &web_sys::Response {
        &self.inner
    }
}

impl_common_wrapper_traits!(Response);

fn create_response_internal(descriptor: ResponseDescriptor) -> Result<Response, JsValue> {
    let ResponseDescriptor {
        status,
        status_text,
        headers,
        body,
    } = descriptor;

    let mut init = web_sys::ResponseInit::new();

    if let Some(headers) = headers {
        init.headers(headers.as_ref());
    }

    init.status(status.into());
    init.status_text(status_text);

    let result = if let Some(body) = body {
        match body {
            BodySource::String(string) => {
                web_sys::Response::new_with_opt_str_and_init(Some(string), &init)
            }
            BodySource::Blob(blob) => {
                web_sys::Response::new_with_opt_blob_and_init(Some(blob.as_ref()), &init)
            }
            BodySource::Bytes(bytes) => {
                web_sys::Response::new_with_opt_u8_array_and_init(Some(bytes), &init)
            }
        }
    } else {
        web_sys::Response::new_with_opt_str_and_init(None, &init)
    };

    result.map(|r| r.into())
}
