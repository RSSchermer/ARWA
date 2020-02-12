use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::event::GenericEventTarget;
use crate::{
    CssStyleDeclaration, Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast,
    Node, TextDirectionality,
};

pub trait HtmlElement: AsRef<web_sys::HtmlElement> {
    fn blur(&self) {
        self.as_ref().blur().unwrap();
    }

    fn click(&self) {
        self.as_ref().click();
    }

    fn focus(&self) {
        self.as_ref().focus().unwrap();
    }

    fn title(&self) -> String {
        self.as_ref().title()
    }

    fn set_title(&self, title: &str) {
        self.as_ref().set_title(title);
    }

    fn lang(&self) -> String {
        self.as_ref().title()
    }

    fn set_lang(&self, lang: &str) {
        self.as_ref().set_lang(lang);
    }

    fn dir(&self) -> TextDirectionality {
        match &*self.as_ref().dir().to_lowercase() {
            "ltr" => TextDirectionality::LeftToRight,
            "rtl" => TextDirectionality::RightToLeft,
            _ => TextDirectionality::Auto,
        }
    }

    fn set_dir(&self, dir: TextDirectionality) {
        let text_directionality = match dir {
            TextDirectionality::Auto => "auto",
            TextDirectionality::LeftToRight => "ltr",
            TextDirectionality::RightToLeft => "rtl",
        };

        self.as_ref().set_dir(text_directionality);
    }

    fn inner_text(&self) -> String {
        self.as_ref().inner_text()
    }

    fn set_inner_text(&self, inner_text: &str) {
        self.as_ref().set_inner_text(inner_text);
    }

    fn hidden(&self) -> bool {
        self.as_ref().hidden()
    }

    fn set_hidden(&self, hidden: bool) {
        self.as_ref().set_hidden(hidden);
    }

    fn tab_index(&self) -> i32 {
        self.as_ref().tab_index()
    }

    fn set_tab_index(&self, tab_index: i32) {
        self.as_ref().set_tab_index(tab_index);
    }

    fn draggable(&self) -> bool {
        self.as_ref().draggable()
    }

    fn set_draggable(&self, draggable: bool) {
        self.as_ref().set_draggable(draggable);
    }

    fn content_editable(&self) -> ContentEditable {
        match &*self.as_ref().content_editable().to_lowercase() {
            "inherit" => ContentEditable::Inherit,
            "false" => ContentEditable::False,
            _ => ContentEditable::True,
        }
    }

    fn is_content_editable(&self) -> bool {
        self.as_ref().is_content_editable()
    }

    fn set_content_editable(&self, content_editable: ContentEditable) {
        let content_editable = match content_editable {
            ContentEditable::True => "true",
            ContentEditable::False => "false",
            ContentEditable::Inherit => "inherit",
        };

        self.as_ref().set_content_editable(content_editable);
    }

    fn spellcheck(&self) -> bool {
        self.as_ref().spellcheck()
    }

    fn set_spellcheck(&self, spellcheck: bool) {
        self.as_ref().set_spellcheck(spellcheck);
    }

    fn style(&self) -> CssStyleDeclaration {
        self.as_ref().style().into()
    }

    fn offset_parent(&self) -> Option<GenericHtmlElement> {
        self.as_ref()
            .offset_parent()
            .map(|element| GenericHtmlElement {
                inner: element.unchecked_into(),
            })
    }

    fn offset_top(&self) -> i32 {
        self.as_ref().offset_top()
    }

    fn offset_left(&self) -> i32 {
        self.as_ref().offset_left()
    }

    fn offset_width(&self) -> i32 {
        self.as_ref().offset_width()
    }

    fn offset_height(&self) -> i32 {
        self.as_ref().offset_height()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContentEditable {
    True,
    False,
    Inherit,
}

#[derive(Clone, PartialEq)]
pub struct GenericHtmlElement {
    inner: web_sys::HtmlElement,
}

impl From<web_sys::HtmlElement> for GenericHtmlElement {
    fn from(inner: web_sys::HtmlElement) -> Self {
        GenericHtmlElement { inner }
    }
}

impl From<GenericHtmlElement> for web_sys::HtmlElement {
    fn from(value: GenericHtmlElement) -> Self {
        value.inner
    }
}

impl TryFrom<GenericEventTarget> for GenericHtmlElement {
    type Error = InvalidCast<GenericEventTarget>;

    fn try_from(value: GenericEventTarget) -> Result<Self, Self::Error> {
        let value: web_sys::EventTarget = value.into();

        value
            .dyn_into::<web_sys::HtmlElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl TryFrom<GenericNode> for GenericHtmlElement {
    type Error = InvalidCast<GenericNode>;

    fn try_from(value: GenericNode) -> Result<Self, Self::Error> {
        let value: web_sys::Node = value.into();

        value
            .dyn_into::<web_sys::HtmlElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl TryFrom<GenericElement> for GenericHtmlElement {
    type Error = InvalidCast<GenericElement>;

    fn try_from(value: GenericElement) -> Result<Self, Self::Error> {
        let value: web_sys::Element = value.into();

        value
            .dyn_into::<web_sys::HtmlElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl AsRef<web_sys::EventTarget> for GenericHtmlElement {
    fn as_ref(&self) -> &web_sys::EventTarget {
        self.inner.as_ref()
    }
}

impl AsRef<web_sys::Node> for GenericHtmlElement {
    fn as_ref(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}

impl AsRef<web_sys::Element> for GenericHtmlElement {
    fn as_ref(&self) -> &web_sys::Element {
        self.inner.as_ref()
    }
}

impl AsRef<web_sys::HtmlElement> for GenericHtmlElement {
    fn as_ref(&self) -> &web_sys::HtmlElement {
        &self.inner
    }
}

impl Write for GenericHtmlElement {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl GlobalEventHandlers for GenericHtmlElement {}
impl Node for GenericHtmlElement {}
impl Element for GenericHtmlElement {}
impl HtmlElement for GenericHtmlElement {}
