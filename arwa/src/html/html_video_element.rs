use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement, HtmlMediaElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlVideoElement {
    inner: web_sys::HtmlVideoElement,
}

impl HtmlVideoElement {
    delegate! {
        target self.inner {
            pub fn width(&self) -> u32;

            pub fn set_width(&self, width: u32);

            pub fn height(&self) -> u32;

            pub fn set_height(&self, height: u32);

            pub fn video_width(&self) -> u32;

            pub fn video_height(&self) -> u32;

            pub fn poster(&self) -> String;

            pub fn set_poster(&self, poster: &str);
        }
    }
}

impl_html_common_traits!(HtmlVideoElement);

impl AsRef<web_sys::HtmlMediaElement> for HtmlVideoElement {
    fn as_ref(&self) -> &web_sys::HtmlMediaElement {
        self.inner.as_ref()
    }
}

impl HtmlMediaElement for HtmlVideoElement {}
