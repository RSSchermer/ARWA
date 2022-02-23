use std::mem;

use delegate::delegate;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::fetch::{Body, BodySource, Headers, Status};
use crate::impl_common_wrapper_traits;
use crate::type_error_wrapper;
use crate::url::{AbsoluteOrRelativeUrl, Url};

pub enum ResponseType {
    Default,
    Basic,
    CORS,
    Error,
    Opaque,
    OpaqueRedirect,
}

impl ResponseType {
    fn from_web_sys(response_type: web_sys::ResponseType) -> Self {
        match response_type {
            web_sys::ResponseType::Basic => ResponseType::Basic,
            web_sys::ResponseType::Cors => ResponseType::CORS,
            web_sys::ResponseType::Error => ResponseType::Error,
            web_sys::ResponseType::Opaque => ResponseType::Opaque,
            web_sys::ResponseType::Opaqueredirect => ResponseType::OpaqueRedirect,
            web_sys::ResponseType::Default => ResponseType::Default,
            _ => unreachable!(),
        }
    }
}

pub struct ResponseDescriptor<'a> {
    pub status: Status,
    pub status_text: &'a str,
    pub headers: Option<&'a Headers>,
    pub body: Option<BodySource<'a>>,
}

impl Default for ResponseDescriptor<'static> {
    fn default() -> Self {
        ResponseDescriptor {
            status: Status::OK,
            status_text: "",
            headers: None,
            body: None,
        }
    }
}

type_error_wrapper!(ResponseInitError);

pub struct Response {
    inner: web_sys::Response,
}

impl Response {
    pub fn init(&self, descriptor: ResponseDescriptor) -> Response {
        create_response_internal(descriptor).unwrap_throw()
    }

    pub fn try_init(&self, descriptor: ResponseDescriptor) -> Result<Response, ResponseInitError> {
        create_response_internal(descriptor)
            .map_err(|err| ResponseInitError::new(err.unchecked_into()))
    }

    pub fn redirect<T>(url: T, status: Status) -> Response
    where
        T: AbsoluteOrRelativeUrl,
    {
        web_sys::Response::redirect_with_status(url.as_str(), status.into())
            .unwrap_throw()
            .into()
    }

    delegate! {
        target self.inner {
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
        ResponseType::from_web_sys(self.inner.type_())
    }

    pub fn url(&self) -> Option<Url> {
        let url = self.inner.url();

        if url.is_empty() {
            None
        } else {
            // Assume URL is always valid for now (until counter-example).
            Some(Url::parse(&url).unwrap_throw())
        }
    }

    pub fn headers(&self) -> Headers {
        self.inner.headers().into()
    }

    pub fn body(&self) -> Body {
        Body::response(Clone::clone(&self.inner))
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
                // TODO: web_sys wants a mutable reference here (which I believe is a general
                // defensive strategy, with explicit whitelisting. Transmute for now, look into
                // getting it whitelisted in web_sys at some point in the future.
                #[allow(mutable_transmutes)]
                let bytes: &mut [u8] = unsafe { mem::transmute(bytes) };

                web_sys::Response::new_with_opt_u8_array_and_init(Some(bytes), &init)
            }
        }
    } else {
        web_sys::Response::new_with_opt_str_and_init(None, &init)
    };

    result.map(|r| r.into())
}
