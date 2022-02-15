use crate::html::{
    constraint_validation_target_seal, form_listed_element_seal, form_submitter_element_seal,
    labelable_element_seal, ConstraintValidationTarget, DynamicFormListedElement, FormEncoding,
    FormListedElement, FormMethod, FormSubmitterElement, HtmlFormElement, LabelableElement, Labels,
    ValidityState,
};
use crate::url::{AbsoluteOrRelativeUrl, Url};
use crate::InvalidCast;
use std::convert::TryFrom;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Submit
    }
}

#[derive(Clone)]
pub struct HtmlButtonElement {
    inner: web_sys::HtmlButtonElement,
}

impl HtmlButtonElement {
    delegate! {
        to self.inner {
            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn autofocus(&self) -> bool;

            pub fn set_autofocus(&self, autofocus: bool);

            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);
        }
    }

    pub fn button_type(&self) -> ButtonType {
        match &*self.inner.type_() {
            "reset" => ButtonType::Reset,
            "button" => ButtonType::Button,
            _ => ButtonType::Submit,
        }
    }

    pub fn set_button_type(&self, button_type: ButtonType) {
        let button_type = match button_type {
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
            ButtonType::Button => "button",
        };

        self.inner.set_type(button_type);
    }
}

impl form_listed_element_seal::Seal for HtmlButtonElement {}

impl FormListedElement for HtmlButtonElement {
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

impl TryFrom<DynamicFormListedElement> for HtmlButtonElement {
    type Error = InvalidCast<DynamicFormListedElement>;

    fn try_from(value: DynamicFormListedElement) -> Result<Self, Self::Error> {
        let value: web_sys::HtmlElement = value.into();

        value
            .dyn_into::<web_sys::HtmlButtonElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast(e.into()))
    }
}

impl form_submitter_element_seal::Seal for HtmlButtonElement {}

impl FormSubmitterElement for HtmlButtonElement {
    delegate! {
        to self.inner {
            fn form_no_validate(&self) -> bool;

            fn set_form_no_validate(&self, form_no_validate: bool);

            fn form_target(&self) -> String;

            fn set_form_target(&self, form_target: &str);
        }
    }

    fn form_action(&self) -> Option<Url> {
        Url::parse(self.inner.form_action()).ok()
    }

    fn set_form_action<T>(&self, form_action: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_form_action(form_action.as_str());
    }

    fn form_encoding(&self) -> Option<FormEncoding> {
        match self.inner.form_enctype().as_ref() {
            "multipart/form-data" => Some(FormEncoding::FormData),
            "text/plain" => Some(FormEncoding::Plain),
            "application/x-www-form-urlencoded" => Some(FormEncoding::UrlEncoded),
            _ => None,
        }
    }

    fn set_form_encoding(&self, encoding: Option<FormEncoding>) {
        self.inner
            .set_form_enctype(encoding.map(|e| e.as_ref()).unwrap_or(""));
    }

    fn form_method(&self) -> FormMethod {
        match &*self.inner.form_method() {
            "post" => FormMethod::Post,
            "dialog" => FormMethod::Dialog,
            _ => FormMethod::Get,
        }
    }

    fn set_form_method(&self, method: FormMethod) {
        let method = match method {
            FormMethod::Get => "get",
            FormMethod::Post => "post",
            FormMethod::Dialog => "dialog",
        };

        self.inner.set_form_method(method);
    }
}

impl labelable_element_seal::Seal for HtmlButtonElement {}

impl LabelableElement for HtmlButtonElement {
    fn labels(&self) -> Labels {
        Labels::new(self.inner.labels())
    }
}

impl constraint_validation_target_seal::Seal for HtmlButtonElement {}

impl ConstraintValidationTarget for HtmlButtonElement {
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

impl From<web_sys::HtmlButtonElement> for HtmlButtonElement {
    fn from(inner: web_sys::HtmlButtonElement) -> Self {
        HtmlButtonElement { inner }
    }
}

impl AsRef<web_sys::HtmlButtonElement> for HtmlButtonElement {
    fn as_ref(&self) -> &web_sys::HtmlButtonElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlButtonElement);
impl_try_from_element!(HtmlButtonElement);
impl_known_element!(HtmlButtonElement, "BUTTON");
