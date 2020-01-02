use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{FormMethod, GenericHtmlElement, HtmlElement, HtmlFormElement, Labels};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

pub use web_sys::ValidityState;

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
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn autofocus(&self) -> bool;

            pub fn set_autofocus(&self, autofocus: bool);

            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn form_action(&self) -> String;

            pub fn set_form_action(&self, form_action: &str);

            pub fn form_no_validate(&self) -> bool;

            pub fn set_form_no_validate(&self, form_no_validate: bool);

            pub fn form_target(&self) -> String;

            pub fn set_form_target(&self, form_target: &str);

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn set_custom_validity(&self, error: &str);

            pub fn validity(&self) -> ValidityState;
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

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }

    pub fn form_encoding(&self) -> String {
        self.inner.form_enctype()
    }

    pub fn set_form_encoding(&self, encoding: &str) {
        self.inner.set_form_enctype(encoding);
    }

    pub fn form_method(&self) -> FormMethod {
        match &*self.inner.form_method() {
            "post" => FormMethod::Post,
            "dialog" => FormMethod::Dialog,
            _ => FormMethod::Get,
        }
    }

    pub fn set_form_method(&self, method: FormMethod) {
        let method = match method {
            FormMethod::Get => "get",
            FormMethod::Post => "post",
            FormMethod::Dialog => "dialog",
        };

        self.inner.set_form_method(method);
    }

    pub fn validation_message(&self) -> String {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.inner.validation_message().unwrap()
    }

    pub fn labels(&self) -> Labels {
        Labels::new(self.inner.labels())
    }
}

impl_html_common_traits!(HtmlButtonElement);
