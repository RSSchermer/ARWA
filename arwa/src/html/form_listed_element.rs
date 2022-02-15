use crate::html::HtmlFormElement;
use std::mem;
use std::ops::Deref;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

pub(crate) mod form_listed_element_seal {
    pub trait Seal {}
}

pub trait FormListedElement: form_listed_element_seal::Seal {
    fn form(&self) -> Option<HtmlFormElement>;

    fn name(&self) -> String;

    fn set_name(&self, name: &str);
}

pub struct DynamicFormListedElement {
    inner: web_sys::HtmlElement,
}

impl DynamicFormListedElement {
    pub(crate) fn new(inner: web_sys::HtmlElement) -> Self {
        DynamicFormListedElement { inner }
    }
}

impl form_listed_element_seal::Seal for DynamicFormListedElement {}

impl FormListedElement for DynamicFormListedElement {
    fn form(&self) -> Option<HtmlFormElement> {
        js_sys::Reflect::get(self.inner.as_ref(), &"form".into())
            .ok()
            .map(|value| {
                if value.is_null() {
                    None
                } else {
                    Some(HtmlFormElement::from(value.unchecked_into()))
                }
            })
    }

    fn name(&self) -> String {
        js_sys::Reflect::get(self.inner.as_ref(), &"form".into())
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or(String::new())
    }

    fn set_name(&self, name: &str) {
        js_sys::Reflect::set(self.inner.as_ref(), &"name".into(), &name.into()).unwrap_throw();
    }
}

impl_html_element_traits!(DynamicFormAssociatedElement, web_sys::HtmlElement);
