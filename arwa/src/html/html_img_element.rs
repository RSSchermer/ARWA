use crate::security::{ReferrerPolicy, CORS};
use crate::url::{AbsoluteOrRelativeUrl, Url, Url};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use url::Url;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ImageDecoding {
    Sync,
    Async,
    Auto,
}

impl Default for ImageDecoding {
    fn default() -> Self {
        ImageDecoding::Auto
    }
}

#[derive(Clone)]
pub struct HtmlImgElement {
    inner: web_sys::HtmlImageElement,
}

impl HtmlImgElement {
    delegate! {
        target self.inner {
            pub fn alt(&self) -> String;

            pub fn set_alt(&self, alt: &str);

            pub fn use_map(&self) -> String;

            pub fn set_use_map(&self, use_map: &str);

            pub fn is_map(&self) -> bool;

            pub fn set_is_map(&self, is_map: bool);

            pub fn width(&self) -> u32;

            pub fn set_width(&self, width: u32);

            pub fn height(&self) -> u32;

            pub fn set_height(&self, height: u32);

            pub fn natural_width(&self) -> u32;

            pub fn natural_height(&self) -> u32;

            pub fn complete(&self) -> bool;
        }
    }

    pub fn src(&self) -> Option<Url> {
        Url::parse(self.inner.src()).ok()
    }

    pub fn set_src<T>(&self, src: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_src(src.as_str());
    }

    pub fn current_src(&self) -> Url {
        Url::new(self.inner.current_src())
    }

    pub fn cross_origin(&self) -> CORS {
        if let Some(cross_origin) = self.inner.cross_origin() {
            match &*cross_origin {
                "use-credentials" => CORS::UseCredentials,
                _ => CORS::Anonymous,
            }
        } else {
            CORS::Anonymous
        }
    }

    pub fn set_cross_origin(&self, cross_origin: CORS) {
        let cross_origin = match cross_origin {
            CORS::Anonymous => "anonymous",
            CORS::UseCredentials => "use-credentials",
        };

        self.inner.set_cross_origin(Some(cross_origin));
    }

    pub fn referrer_policy(&self) -> ReferrerPolicy {
        ReferrerPolicy::from_str(&self.inner.referrer_policy())
    }

    pub fn set_referrer_policy(&self, referrer_policy: ReferrerPolicy) {
        self.inner.set_referrer_policy(referrer_policy.as_ref())
    }

    pub fn decoding(&self) -> ImageDecoding {
        match &*self.inner.decoding() {
            "sync" => ImageDecoding::Sync,
            "async" => ImageDecoding::Async,
            _ => ImageDecoding::Auto,
        }
    }

    pub fn set_decoding(&self, decoding: ImageDecoding) {
        let decoding = match decoding {
            ImageDecoding::Sync => "sync",
            ImageDecoding::Async => "async",
            ImageDecoding::Auto => "auto",
        };

        self.inner.set_decoding(decoding);
    }

    pub fn decode(&self) -> ImageDecode {
        ImageDecode {
            image_element: Some(self.inner.clone()),
            inner: None,
        }
    }

    // TODO: `srcset` and `sizes`.
}

impl From<web_sys::HtmlImageElement> for HtmlImgElement {
    fn from(inner: web_sys::HtmlImageElement) -> Self {
        HtmlImageElement { inner }
    }
}

impl AsRef<web_sys::HtmlImageElement> for HtmlImgElement {
    fn as_ref(&self) -> &web_sys::HtmlImageElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlImgElement);
impl_try_from_element!(HtmlImgElement, web_sys::HtmlImageElement);
impl_known_element!(HtmlImgElement, "IMG");

#[derive(Clone)]
pub struct ImageEncodingError {
    inner: web_sys::DomException,
}

impl ImageEncodingError {
    fn new(inner: web_sys::DomException) -> Self {
        ImageEncodingError { inner }
    }
}

pub struct ImageDecode {
    image_element: Option<web_sys::HtmlImageElement>,
    inner: Option<JsFuture>,
}

impl Future for ImageDecode {
    type Output = Result<(), ImageEncodingError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(image_element) = self.image_element.take() {
            self.inner = Some(image_element.decode().into());
        }

        self.inner
            .as_mut()
            .unwrap()
            .poll(cx)
            .map_ok(|_| ())
            .map_err(|err| ImageEncodingError::new(err.unchecked_into()))
    }
}
