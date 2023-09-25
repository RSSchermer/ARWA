use std::str::FromStr;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::dom::{impl_try_from_element, DynamicDocument};
use crate::html::{
    constraint_validation_target_seal, form_listed_element_seal, impl_extendable_element,
    impl_html_element_traits, impl_known_element, ConstraintValidationTarget,
    DynamicFormListedElement, FormListedElement, HtmlFormElement, ValidityState,
};
use crate::media_type::MediaType;
use crate::url::Url;
use crate::window::Window;
use crate::InvalidCast;

#[derive(Clone)]
pub struct HtmlObjectElement {
    inner: web_sys::HtmlObjectElement,
}

impl HtmlObjectElement {
    delegate! {
        to self.inner {
            pub fn use_map(&self) -> String;

            pub fn set_use_map(&self, use_map: &str);
        }
    }

    pub fn data(&self) -> Option<Url> {
        Url::parse(self.inner.data().as_ref()).ok()
    }

    pub fn set_data(&self, data: &Url) {
        self.inner.set_data(data.as_ref());
    }

    pub fn media_type(&self) -> Option<MediaType> {
        MediaType::parse(self.inner.type_().as_ref()).ok()
    }

    pub fn set_media_type(&self, media_type: Option<&MediaType>) {
        self.inner
            .set_type(media_type.map(|m| m.as_ref()).unwrap_or(""));
    }

    pub fn width(&self) -> u32 {
        u32::from_str(&self.inner.width()).unwrap_or(0)
    }

    pub fn set_width(&self, width: u32) {
        self.inner.set_width(&width.to_string());
    }

    pub fn height(&self) -> u32 {
        u32::from_str(&self.inner.height()).unwrap_or(0)
    }

    pub fn set_height(&self, height: u32) {
        self.inner.set_height(&height.to_string());
    }

    pub fn content_document(&self) -> Option<DynamicDocument> {
        self.inner
            .content_document()
            .map(|document| document.into())
    }

    pub fn content_window(&self) -> Option<Window> {
        self.inner.content_window().map(|w| w.into())
    }
}

impl form_listed_element_seal::Seal for HtmlObjectElement {}

impl FormListedElement for HtmlObjectElement {
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

impl TryFrom<DynamicFormListedElement> for HtmlObjectElement {
    type Error = InvalidCast<DynamicFormListedElement, HtmlObjectElement>;

    fn try_from(value: DynamicFormListedElement) -> Result<Self, Self::Error> {
        let value: web_sys::HtmlElement = value.into();

        value
            .dyn_into::<web_sys::HtmlObjectElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast::new(DynamicFormListedElement::new(e)))
    }
}

impl constraint_validation_target_seal::Seal for HtmlObjectElement {}

impl ConstraintValidationTarget for HtmlObjectElement {
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

impl From<web_sys::HtmlObjectElement> for HtmlObjectElement {
    fn from(inner: web_sys::HtmlObjectElement) -> Self {
        HtmlObjectElement { inner }
    }
}

impl AsRef<web_sys::HtmlObjectElement> for HtmlObjectElement {
    fn as_ref(&self) -> &web_sys::HtmlObjectElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlObjectElement);
impl_try_from_element!(HtmlObjectElement);
impl_known_element!(HtmlObjectElement, "OBJECT");
impl_extendable_element!(HtmlObjectElement, "object");
