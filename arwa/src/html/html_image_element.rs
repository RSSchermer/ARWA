use std::convert::TryFrom;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use delegate::delegate;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{
    Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node, ReferrerPolicy,
    CORS,
};

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
pub struct HtmlImageElement {
    inner: web_sys::HtmlImageElement,
}

impl HtmlImageElement {
    delegate! {
        target self.inner {
            pub fn alt(&self) -> String;

            pub fn set_alt(&self, alt: &str);

            pub fn src(&self) -> String;

            pub fn set_src(&self, src: &str);

            pub fn srcset(&self) -> String;

            pub fn set_srcset(&self, srcset: &str);

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

            pub fn current_src(&self) -> String;

            pub fn sizes(&self) -> String;

            pub fn set_sizes(&self, sizes: &str);
        }
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
            inner: JsFuture::from(self.inner.decode()),
        }
    }
}

impl_html_common_traits!(HtmlImageElement);

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EncodingError(String);

pub struct ImageDecode {
    inner: JsFuture,
}

impl Future for ImageDecode {
    type Output = Result<(), EncodingError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner)
                .poll(cx)
                .map_ok(|_| ())
                .map_err(|err| {
                    let err: web_sys::DomException = err.unchecked_into();

                    EncodingError(err.message())
                })
        }
    }
}
