use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement, HtmlMediaElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlAudioElement {
    inner: web_sys::HtmlAudioElement,
}

impl AsRef<web_sys::HtmlMediaElement> for HtmlAudioElement {
    fn as_ref(&self) -> &web_sys::HtmlMediaElement {
        self.inner.as_ref()
    }
}

impl HtmlMediaElement for HtmlAudioElement {}

impl_html_common_traits!(HtmlAudioElement);
