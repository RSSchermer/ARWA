use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlDivElement {
    inner: web_sys::HtmlDivElement,
}

impl_html_common_traits!(HtmlDivElement);
