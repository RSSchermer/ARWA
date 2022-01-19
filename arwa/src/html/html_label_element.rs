use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::GenericEventTarget;
use crate::html::{GenericHtmlElement, HtmlElement, HtmlFormElement};
use crate::{DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlLabelElement {
    inner: web_sys::HtmlLabelElement,
}

impl HtmlLabelElement {
    delegate! {
        target self.inner {
            pub fn html_for(&self) -> String;

            pub fn set_html_for(&self, html_for: &str);
        }
    }

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }

    pub fn control(&self) -> Option<GenericHtmlElement> {
        self.inner.control().map(|e| e.into())
    }
}

impl_html_common_traits!(HtmlLabelElement);

pub struct OutputHtmlFor {
    inner: web_sys::DomTokenList,
}

impl OutputHtmlFor {
    // TODO: it's unclear from the WHATWG spec and MDN if this is meant to be modifiable.

    pub fn get(&self, index: usize) -> Option<String> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<String> {
        self.get(0)
    }

    pub fn last(&self) -> Option<String> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> OutputHtmlForIter {
        OutputHtmlForIter {
            output_html_for: self,
            current: 0,
        }
    }
}

impl IntoIterator for OutputHtmlFor {
    type Item = String;
    type IntoIter = OutputHtmlForIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        OutputHtmlForIntoIter {
            output_html_for: self,
            current: 0,
        }
    }
}

pub struct OutputHtmlForIter<'a> {
    output_html_for: &'a OutputHtmlFor,
    current: usize,
}

impl<'a> Iterator for OutputHtmlForIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.output_html_for.get(current)
    }
}

pub struct OutputHtmlForIntoIter {
    output_html_for: OutputHtmlFor,
    current: usize,
}

impl Iterator for OutputHtmlForIntoIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.output_html_for.get(current)
    }
}
