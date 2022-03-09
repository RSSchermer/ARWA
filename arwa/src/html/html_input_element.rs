use std::convert::TryFrom;
use std::ops::{Bound, Range, RangeBounds};

use delegate::delegate;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::collection::{Collection, Sequence};
use crate::dom::{impl_try_from_element, SelectionDirection};
use crate::file::File;
use crate::html::{
    constraint_validation_target_seal, form_listed_element_seal, form_submitter_element_seal,
    impl_html_element_traits, impl_known_element, labelable_element_seal, AutoComplete,
    ConstraintValidationTarget, DynamicFormListedElement, FormEncoding, FormListedElement,
    FormMethod, FormSubmitterElement, HtmlDatalistElement, HtmlFormElement, LabelableElement,
    Labels, ValidityState,
};
use crate::url::Url;
use crate::InvalidCast;

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
            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn value_as_number(&self) -> f64;

            pub fn set_value_as_number(&self, value_as_number: f64);

            pub fn default_value(&self) -> String;

            pub fn set_default_value(&self, default_value: &str);

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

            pub fn width(&self) -> u32;

            pub fn set_width(&self, width: u32);

            pub fn height(&self) -> u32;

            pub fn set_height(&self, height: u32);

            pub fn indeterminate(&self) -> bool;

            pub fn set_indeterminate(&self, indeterminate: bool);

            pub fn multiple(&self) -> bool;

            pub fn set_multiple(&self, multiple: bool);

            pub fn placeholder(&self) -> String;

            pub fn set_placeholder(&self, placeholder: &str);

            pub fn read_only(&self) -> bool;

            pub fn set_read_only(&self, read_only: bool);

            pub fn required(&self) -> bool;

            pub fn set_required(&self, required: bool);

            pub fn size(&self) -> u32;

            pub fn set_size(&self, size: u32);

            pub fn step(&self) -> String;

            pub fn set_step(&self, step: &str);

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

    pub fn src(&self) -> Option<Url> {
        Url::parse(self.inner.src().as_ref()).ok()
    }

    pub fn set_src(&self, src: &Url) {
        self.inner.set_src(src.as_ref());
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

    pub fn files(&self) -> Option<InputFiles> {
        self.inner.files().map(|inner| InputFiles { inner })
    }

    pub fn set_files<S>(&self, files: S)
    where
        S: InputFilesSource,
    {
        self.inner.set_files(Some(files.as_web_sys_file_list()));
    }

    pub fn list(&self) -> Option<HtmlDatalistElement> {
        self.inner.list().map(|e| {
            let list: web_sys::HtmlDataListElement = e.unchecked_into();

            list.into()
        })
    }

    pub fn selection_range(&self) -> Option<Range<u32>> {
        let start = self.inner.selection_start().ok().flatten();
        let end = self.inner.selection_end().ok().flatten();

        match (start, end) {
            (Some(start), Some(end)) => Some(start..end),
            _ => None,
        }
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

    pub fn set_selection<R>(&self, range: R, direction: SelectionDirection)
    where
        R: RangeBounds<u32>,
    {
        let start = match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => self.inner.selection_start().ok().flatten().unwrap_or(0),
        };

        let end = match range.end_bound() {
            Bound::Included(end) => *end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => self.inner.selection_end().ok().flatten().unwrap_or(0),
        };

        let direction = match direction {
            SelectionDirection::Forward => "forward",
            SelectionDirection::Backward => "backward",
            SelectionDirection::None => "none",
        };

        self.inner
            .set_selection_range_with_direction(start, end, direction)
            .unwrap_throw()
    }

    pub fn set_selection_text(&self, text: &str) {
        self.inner.set_range_text(text).unwrap_throw()
    }

    // TODO: `accept`, `min`, `max`, `pattern` all take complex restricted string types.
}

impl form_listed_element_seal::Seal for HtmlInputElement {}

impl FormListedElement for HtmlInputElement {
    delegate! {
        target self.inner {
            fn name(&self) -> String;

            fn set_name(&self, name: &str);
        }
    }

    fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }
}

impl TryFrom<DynamicFormListedElement> for HtmlInputElement {
    type Error = InvalidCast<DynamicFormListedElement, HtmlInputElement>;

    fn try_from(value: DynamicFormListedElement) -> Result<Self, Self::Error> {
        let value: web_sys::HtmlElement = value.into();

        value
            .dyn_into::<web_sys::HtmlInputElement>()
            .map(|e| e.into())
            .map_err(|e| InvalidCast::new(DynamicFormListedElement::new(e)))
    }
}

impl form_submitter_element_seal::Seal for HtmlInputElement {}

impl FormSubmitterElement for HtmlInputElement {
    delegate! {
        target self.inner {
            fn form_no_validate(&self) -> bool;

            fn set_form_no_validate(&self, form_no_validate: bool);

            fn form_target(&self) -> String;

            fn set_form_target(&self, form_target: &str);
        }
    }

    fn form_action(&self) -> Option<Url> {
        Url::parse(self.inner.form_action().as_ref()).ok()
    }

    fn set_form_action(&self, form_action: &Url) {
        self.inner.set_form_action(form_action.as_ref());
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
            .set_form_enctype(encoding.as_ref().map(|e| e.as_ref()).unwrap_or(""));
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

impl labelable_element_seal::Seal for HtmlInputElement {}

impl LabelableElement for HtmlInputElement {
    fn labels(&self) -> Labels {
        Labels::new(self.inner.labels())
    }
}

impl constraint_validation_target_seal::Seal for HtmlInputElement {}

impl ConstraintValidationTarget for HtmlInputElement {
    delegate! {
        target self.inner {
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

impl From<web_sys::HtmlInputElement> for HtmlInputElement {
    fn from(inner: web_sys::HtmlInputElement) -> Self {
        HtmlInputElement { inner }
    }
}

impl AsRef<web_sys::HtmlInputElement> for HtmlInputElement {
    fn as_ref(&self) -> &web_sys::HtmlInputElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlInputElement);
impl_try_from_element!(HtmlInputElement);
impl_known_element!(HtmlInputElement, "INPUT");

pub struct InputFiles {
    inner: web_sys::FileList,
}

impl Collection for InputFiles {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for InputFiles {
    type Item = File;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|f| f.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

pub(crate) mod input_files_source_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_file_list(&self) -> &web_sys::FileList;
    }
}

pub trait InputFilesSource: input_files_source_seal::Seal {}

impl input_files_source_seal::Seal for web_sys::FileList {
    fn as_web_sys_file_list(&self) -> &web_sys::FileList {
        self
    }
}

impl InputFilesSource for web_sys::FileList {}

impl input_files_source_seal::Seal for InputFiles {
    fn as_web_sys_file_list(&self) -> &web_sys::FileList {
        &self.inner
    }
}

impl InputFilesSource for InputFiles {}
