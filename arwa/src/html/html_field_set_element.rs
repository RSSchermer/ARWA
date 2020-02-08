use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{GenericHtmlElement, HtmlElement, HtmlFormElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

use crate::console::{Write, Writer};
pub use web_sys::ValidityState;

#[derive(Clone)]
pub struct HtmlFieldSetElement {
    inner: web_sys::HtmlFieldSetElement,
}

impl HtmlFieldSetElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn will_validate(&self) -> bool;

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn set_custom_validity(&self, error: &str);

            pub fn validity(&self) -> ValidityState;
        }
    }

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }

    pub fn validation_message(&self) -> String {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.inner.validation_message().unwrap()
    }

    pub fn elements(&self) -> FieldSetElements {
        FieldSetElements {
            inner: self.inner.elements(),
        }
    }
}

impl_html_common_traits!(HtmlFieldSetElement);

pub struct FieldSetElements {
    inner: web_sys::HtmlCollection,
}

impl FieldSetElements {
    pub fn get(&self, index: usize) -> Option<GenericHtmlElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get_with_index(index))
            .map(|e| {
                let e: web_sys::HtmlElement = e.unchecked_into();

                e.into()
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<GenericHtmlElement> {
        self.inner.get_with_name(id).map(|e| {
            let e: web_sys::HtmlElement = e.unchecked_into();

            e.into()
        })
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

    pub fn first(&self) -> Option<GenericHtmlElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<GenericHtmlElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> FieldSetElementsIter {
        FieldSetElementsIter {
            field_set_elements: self,
            current: 0,
        }
    }
}

impl Write for FieldSetElements {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref());
    }
}

impl IntoIterator for FieldSetElements {
    type Item = GenericHtmlElement;
    type IntoIter = FieldSetElementsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        FieldSetElementsIntoIter {
            field_set_elements: self,
            current: 0,
        }
    }
}

pub struct FieldSetElementsIter<'a> {
    field_set_elements: &'a FieldSetElements,
    current: usize,
}

impl<'a> Iterator for FieldSetElementsIter<'a> {
    type Item = GenericHtmlElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.field_set_elements.get(current)
    }
}

pub struct FieldSetElementsIntoIter {
    field_set_elements: FieldSetElements,
    current: usize,
}

impl Iterator for FieldSetElementsIntoIter {
    type Item = GenericHtmlElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.field_set_elements.get(current)
    }
}
