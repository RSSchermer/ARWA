use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlTableCaptionElement {
    inner: web_sys::HtmlTableCaptionElement,
}

impl_html_common_traits!(HtmlTableCaptionElement);
