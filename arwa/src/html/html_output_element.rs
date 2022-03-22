use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::dom::impl_try_from_element;
use crate::html::{
    constraint_validation_target_seal, form_listed_element_seal, impl_extendable_element,
    impl_html_element_traits, impl_known_element, labelable_element_seal,
    ConstraintValidationTarget, DynamicFormListedElement, FormListedElement, HtmlFormElement,
    LabelableElement, Labels, ValidityState,
};
use crate::InvalidCast;

#[derive(Clone)]
pub struct HtmlOutputElement {
    inner: web_sys::HtmlOutputElement,
}

impl HtmlOutputElement {
    delegate! {
        to self.inner {
            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn default_value(&self) -> String;

            pub fn set_default_value(&self, default_value: &str);
        }
    }

    // TODO: decide what to do about `type`, which is a readonly attribute that can only be
    // "output". This may make sense in a dynamic language, but may not in Rust.
}

impl form_listed_element_seal::Seal for HtmlOutputElement {}

impl FormListedElement for HtmlOutputElement {
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

impl TryFrom<DynamicFormListedElement> for HtmlOutputElement {
    type Error = InvalidCast<DynamicFormListedElement, HtmlOutputElement>;

    fn try_from(value: DynamicFormListedElement) -> Result<Self, Self::Error> {
        let value: web_sys::HtmlElement = value.into();

        value
            .dyn_into::<web_sys::HtmlOutputElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast::new(DynamicFormListedElement::new(e)))
    }
}

impl constraint_validation_target_seal::Seal for HtmlOutputElement {}

impl ConstraintValidationTarget for HtmlOutputElement {
    delegate! {
        to self.inner {
            fn will_validate(&self) -> bool;

            fn check_validity(&self) -> bool;

            fn report_validity(&self) -> bool;

            fn set_custom_validity(&self, error: &str);
        }
    }

    fn validity(&self) -> ValidityState {
        self.inner.validity().into()
    }

    fn validation_message(&self) -> String {
        self.inner.validation_message().unwrap_or(String::new())
    }
}

impl labelable_element_seal::Seal for HtmlOutputElement {}

impl LabelableElement for HtmlOutputElement {
    fn labels(&self) -> Labels {
        Labels::new(Some(self.inner.labels()))
    }
}

impl From<web_sys::HtmlOutputElement> for HtmlOutputElement {
    fn from(inner: web_sys::HtmlOutputElement) -> Self {
        HtmlOutputElement { inner }
    }
}

impl AsRef<web_sys::HtmlOutputElement> for HtmlOutputElement {
    fn as_ref(&self) -> &web_sys::HtmlOutputElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlOutputElement);
impl_try_from_element!(HtmlOutputElement);
impl_known_element!(HtmlOutputElement, "OUTPUT");
impl_extendable_element!(HtmlOutputElement, "output");
