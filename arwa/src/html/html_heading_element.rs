use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlHeadingElement {
    inner: web_sys::HtmlHeadingElement,
}

impl_html_common_traits!(HtmlHeadingElement);
