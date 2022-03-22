use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::collection::{Collection, Sequence};
use crate::dom::impl_try_from_element;
use crate::html::{
    form_listed_element_seal, impl_extendable_element, impl_html_element_traits,
    impl_known_element, DynamicFormListedElement, FormListedElement, HtmlFormElement,
};
use crate::InvalidCast;

#[derive(Clone)]
pub struct HtmlFieldsetElement {
    inner: web_sys::HtmlFieldSetElement,
}

impl HtmlFieldsetElement {
    // Note: ignoring attributes/method that are part of the constraint validation API, as
    // `fieldset` is "barred from constraint validation".

    delegate! {
        to self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);
        }
    }

    pub fn elements(&self) -> FieldsetElements {
        FieldsetElements {
            inner: self.inner.elements(),
        }
    }
}

impl form_listed_element_seal::Seal for HtmlFieldsetElement {}

impl FormListedElement for HtmlFieldsetElement {
    delegate! {
        to self.inner {
            fn name(&self) -> String;

            fn set_name(&self, name: &str);
        }
    }

    fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }
}

impl TryFrom<DynamicFormListedElement> for HtmlFieldsetElement {
    type Error = InvalidCast<DynamicFormListedElement, HtmlFieldsetElement>;

    fn try_from(value: DynamicFormListedElement) -> Result<Self, Self::Error> {
        let value: web_sys::HtmlElement = value.into();

        value
            .dyn_into::<web_sys::HtmlFieldSetElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast::new(DynamicFormListedElement::new(e)))
    }
}

impl From<web_sys::HtmlFieldSetElement> for HtmlFieldsetElement {
    fn from(inner: web_sys::HtmlFieldSetElement) -> Self {
        HtmlFieldsetElement { inner }
    }
}

impl AsRef<web_sys::HtmlFieldSetElement> for HtmlFieldsetElement {
    fn as_ref(&self) -> &web_sys::HtmlFieldSetElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlFieldsetElement);
impl_try_from_element!(HtmlFieldsetElement, HtmlFieldSetElement);
impl_known_element!(HtmlFieldsetElement, HtmlFieldSetElement, "FIELDSET");
impl_extendable_element!(HtmlFieldsetElement, "fieldset");

pub struct FieldsetElements {
    inner: web_sys::HtmlCollection,
}

impl Collection for FieldsetElements {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for FieldsetElements {
    type Item = DynamicFormListedElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get_with_index(index)
            .map(|e| DynamicFormListedElement::new(e.unchecked_into()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
