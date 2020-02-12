use std::convert::TryFrom;
use std::ops::{Bound, RangeBounds};

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::error::{InvalidStateError, RangeError, SetTextRangeError};
use crate::event::GenericEventTarget;
use crate::html::{
    AutoComplete, FormMethod, GenericHtmlElement, HtmlDataListElement, HtmlElement,
    HtmlFormElement, Labels,
};
use crate::{
    Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node,
    SelectionDirection,
};

pub use web_sys::ValidityState;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputType {
    Hidden,
    Text,
    Search,
    Tel,
    Url,
    Email,
    Password,
    Date,
    Month,
    Week,
    Time,
    DatetimeLocal,
    Number,
    Range,
    Color,
    Checkbox,
    Radio,
    File,
    Submit,
    Image,
    Reset,
    Button,
}

impl Default for InputType {
    fn default() -> Self {
        InputType::Text
    }
}

#[derive(Clone)]
pub struct HtmlInputElement {
    inner: web_sys::HtmlInputElement,
}

impl HtmlInputElement {
    // TODO: input_mode. I can't seem to find it in the WHATWG spec and the MDN documentation on
    // input_mode is very sparse. Feels like there should be some enum somewhere for all available
    // modes.

    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn value_as_number(&self) -> f64;

            pub fn set_value_as_number(&self, value_as_number: f64);

            pub fn default_value(&self) -> String;

            pub fn set_default_value(&self, default_value: &str);

            pub fn accept(&self) -> String;

            pub fn set_accept(&self, accept: &str);

            pub fn alt(&self) -> String;

            pub fn set_alt(&self, alt: &str);

            pub fn autofocus(&self) -> bool;

            pub fn set_autofocus(&self, autofocus: bool);

            pub fn default_checked(&self) -> bool;

            pub fn set_default_checked(&self, default_checked: bool);

            pub fn checked(&self) -> bool;

            pub fn set_checked(&self, checked: bool);

            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn form_action(&self) -> String;

            pub fn set_form_action(&self, form_action: &str);

            pub fn form_no_validate(&self) -> bool;

            pub fn set_form_no_validate(&self, form_no_validate: bool);

            pub fn form_target(&self) -> String;

            pub fn set_form_target(&self, form_target: &str);

            pub fn width(&self) -> u32;

            pub fn set_width(&self, width: u32);

            pub fn height(&self) -> u32;

            pub fn set_height(&self, height: u32);

            pub fn indeterminate(&self) -> bool;

            pub fn set_indeterminate(&self, indeterminate: bool);

            pub fn min(&self) -> String;

            pub fn set_min(&self, min: &str);

            pub fn max(&self) -> String;

            pub fn set_max(&self, max: &str);

            pub fn multiple(&self) -> bool;

            pub fn set_multiple(&self, multiple: bool);

            pub fn pattern(&self) -> String;

            pub fn set_pattern(&self, pattern: &str);

            pub fn placeholder(&self) -> String;

            pub fn set_placeholder(&self, placeholder: &str);

            pub fn read_only(&self) -> bool;

            pub fn set_read_only(&self, read_only: bool);

            pub fn required(&self) -> bool;

            pub fn set_required(&self, required: bool);

            pub fn size(&self) -> u32;

            pub fn set_size(&self, size: u32);

            pub fn src(&self) -> String;

            pub fn set_src(&self, src: &str);

            pub fn step(&self) -> String;

            pub fn set_step(&self, step: &str);

            pub fn will_validate(&self) -> bool;

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn set_custom_validity(&self, error: &str);

            pub fn validity(&self) -> ValidityState;

            pub fn select(&self);
        }
    }

    pub fn input_type(&self) -> InputType {
        match &*self.inner.type_() {
            "hidden" => InputType::Hidden,
            "search" => InputType::Search,
            "tel" => InputType::Tel,
            "url" => InputType::Url,
            "email" => InputType::Email,
            "password" => InputType::Password,
            "date" => InputType::Date,
            "month" => InputType::Month,
            "week" => InputType::Week,
            "time" => InputType::Time,
            "datetime-local" => InputType::DatetimeLocal,
            "number" => InputType::Number,
            "range" => InputType::Range,
            "color" => InputType::Color,
            "checkbox" => InputType::Checkbox,
            "radio" => InputType::Radio,
            "file" => InputType::File,
            "submit" => InputType::Submit,
            "image" => InputType::Image,
            "reset" => InputType::Reset,
            "button" => InputType::Button,
            _ => InputType::Text,
        }
    }

    pub fn set_input_type(&self, input_type: InputType) {
        let input_type = match input_type {
            InputType::Hidden => "hidden",
            InputType::Text => "text",
            InputType::Search => "search",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Date => "date",
            InputType::Month => "month",
            InputType::Week => "week",
            InputType::Time => "time",
            InputType::DatetimeLocal => "datetime-local",
            InputType::Number => "number",
            InputType::Range => "range",
            InputType::Color => "color",
            InputType::Checkbox => "checkbox",
            InputType::Radio => "radio",
            InputType::File => "file",
            InputType::Submit => "submit",
            InputType::Image => "image",
            InputType::Reset => "reset",
            InputType::Button => "button",
        };

        self.inner.set_type(input_type);
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

    pub fn max_length(&self) -> u32 {
        self.inner.max_length() as u32
    }

    pub fn set_max_length(&self, max_length: u32) {
        self.inner.set_max_length(max_length as i32);
    }

    pub fn min_length(&self) -> u32 {
        self.inner.min_length() as u32
    }

    pub fn set_min_length(&self, min_length: u32) {
        self.inner.set_min_length(min_length as i32);
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

    // TODO: files

    pub fn list(&self) -> Option<HtmlDataListElement> {
        self.inner.list().map(|e| {
            let list: web_sys::HtmlDataListElement = e.unchecked_into();

            list.into()
        })
    }

    pub fn validation_message(&self) -> String {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.inner.validation_message().unwrap()
    }

    pub fn selection_start(&self) -> Option<u32> {
        self.inner.selection_start().ok().flatten()
    }

    pub fn selection_end(&self) -> Option<u32> {
        self.inner.selection_end().ok().flatten()
    }

    pub fn selection_direction(&self) -> Option<SelectionDirection> {
        self.inner
            .selection_direction()
            .ok()
            .flatten()
            .map(|direction| match &*direction {
                "forward" => SelectionDirection::Forward,
                "backward" => SelectionDirection::Backward,
                _ => SelectionDirection::None,
            })
    }

    // TODO: set_selection_start, set_selection_end, set_selection_direction or rely on
    // set_selection_range?

    pub fn labels(&self) -> Option<Labels> {
        self.inner.labels().map(|inner| Labels::new(inner))
    }

    pub fn set_text_range<R>(&self, range: R, text: &str) -> Result<(), SetTextRangeError>
    where
        R: RangeBounds<u32>,
    {
        let start = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => self.selection_end().unwrap_or(0),
        };

        let end = match range.end_bound() {
            Bound::Included(end) => *end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => self.selection_end().unwrap_or(0),
        };

        self.inner
            .set_range_text_with_start_and_end(text, start, end)
            .map_err(|err| {
                let err: web_sys::DomException = err.unchecked_into();

                match &*err.name() {
                    "IndexSizeError" => RangeError::new(err).into(),
                    "RangeError" => RangeError::new(err).into(),
                    "InvalidStateError" => InvalidStateError::new(err).into(),
                    _ => unreachable!(),
                }
            })
    }

    pub fn set_selection_range<R>(
        &self,
        range: R,
        direction: SelectionDirection,
    ) -> Result<(), InvalidStateError>
    where
        R: RangeBounds<u32>,
    {
        let start = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => self.selection_end().unwrap_or(0),
        };

        let end = match range.end_bound() {
            Bound::Included(end) => *end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => self.selection_end().unwrap_or(0),
        };

        let direction = match direction {
            SelectionDirection::Forward => "forward",
            SelectionDirection::Backward => "backward",
            SelectionDirection::None => "none",
        };

        self.inner
            .set_selection_range_with_direction(start, end, direction)
            .map_err(|err| InvalidStateError::new(err.unchecked_into()))
    }
}

impl_html_common_traits!(HtmlInputElement);
