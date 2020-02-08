use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::html::{AutoComplete, GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FormMethod {
    Get,
    Post,
    Dialog,
}

impl Default for FormMethod {
    fn default() -> Self {
        FormMethod::Get
    }
}

#[derive(Clone)]
pub struct HtmlFormElement {
    inner: web_sys::HtmlFormElement,
}

impl HtmlFormElement {
    // TODO: enum for encoding? Spec seems to only allow 3 specific values
    // (`application/x-www-form-urlencoded`, `multipart/form-data`, `text/plain`).

    delegate! {
        target self.inner {
            pub fn accept_charset(&self) -> String;

            pub fn set_accept_charset(&self, accept_charset: &str);

            pub fn action(&self) -> String;

            pub fn set_action(&self, action: &str);

            pub fn encoding(&self) -> String;

            pub fn set_encoding(&self, encoding: &str);

            pub fn no_validate(&self) -> bool;

            pub fn set_no_validate(&self, no_validate: bool);

            pub fn target(&self) -> String;

            pub fn set_target(&self, target: &str);

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn reset(&self);
        }
    }

    pub fn autocomplete(&self) -> AutoComplete {
        match &*self.inner.autocomplete() {
            "off" => AutoComplete::Off,
            _ => AutoComplete::On,
        }
    }

    pub fn set_autocomplete(&self, autocomplete: AutoComplete) {
        let autocomplete = match autocomplete {
            AutoComplete::On => "on",
            AutoComplete::Off => "off",
        };

        self.inner.set_autocomplete(autocomplete);
    }

    pub fn method(&self) -> FormMethod {
        match &*self.inner.method() {
            "post" => FormMethod::Post,
            "dialog" => FormMethod::Dialog,
            _ => FormMethod::Get,
        }
    }

    pub fn set_method(&self, method: FormMethod) {
        let method = match method {
            FormMethod::Get => "get",
            FormMethod::Post => "post",
            FormMethod::Dialog => "dialog",
        };

        self.inner.set_method(method);
    }

    pub fn elements(&self) -> FormControlElements {
        FormControlElements {
            inner: self.inner.elements().unchecked_into(),
        }
    }

    pub fn submit(&self) {
        // Despite the web_sys return type, I can find no indication in the spec that `submit` can
        // actually error, so just unwrap for now.
        self.inner.submit().unwrap();
    }
}

impl_html_common_traits!(HtmlFormElement);

pub struct FormControlElements {
    inner: web_sys::HtmlFormControlsCollection,
}

impl FormControlElements {
    // TODO: decide what to do about iteration, indexing
    pub fn get(&self, id_or_name: &str) -> Option<FormControl> {
        self.inner
            .named_item(id_or_name)
            .map(|inner| FormControl { inner })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }
}

impl Write for FormControlElements {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

pub struct FormControl {
    inner: js_sys::Object,
}

impl TryFrom<FormControl> for RadioNodes {
    type Error = InvalidCast<FormControl>;

    fn try_from(value: FormControl) -> Result<Self, Self::Error> {
        value
            .inner
            .dyn_into::<web_sys::RadioNodeList>()
            .map(|inner| RadioNodes { inner })
            .map_err(|inner| InvalidCast(FormControl { inner }))
    }
}

impl TryFrom<FormControl> for GenericHtmlElement {
    type Error = InvalidCast<FormControl>;

    fn try_from(value: FormControl) -> Result<Self, Self::Error> {
        value
            .inner
            .dyn_into::<web_sys::HtmlElement>()
            .map(|e| e.into())
            .map_err(|inner| InvalidCast(FormControl { inner }))
    }
}

impl Write for FormControl {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

// TODO: impl TryFrom<FormControl> for: HtmlInputElement, HtmlButtonElement, HtmlFieldsetElement,
// HtmlObjectElement, HtmlOutputElement, HtmlSelectElement, HtmlTextareaElement

pub struct RadioNodes {
    inner: web_sys::RadioNodeList,
}

impl RadioNodes {
    delegate! {
        target self.inner {
            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }

    pub fn get(&self, index: usize) -> Option<GenericNode> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get(index))
            .map(|e| e.into())
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<GenericNode> {
        self.get(0)
    }

    pub fn last(&self) -> Option<GenericNode> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> RadioNodesIter {
        RadioNodesIter {
            radio_nodes: self,
            current: 0,
        }
    }
}

impl Write for RadioNodes {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for RadioNodes {
    type Item = GenericNode;
    type IntoIter = RadioNodesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        RadioNodesIntoIter {
            radio_nodes: self,
            current: 0,
        }
    }
}

pub struct RadioNodesIter<'a> {
    radio_nodes: &'a RadioNodes,
    current: usize,
}

impl<'a> Iterator for RadioNodesIter<'a> {
    type Item = GenericNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.radio_nodes.get(current)
    }
}

pub struct RadioNodesIntoIter {
    radio_nodes: RadioNodes,
    current: usize,
}

impl Iterator for RadioNodesIntoIter {
    type Item = GenericNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.radio_nodes.get(current)
    }
}
