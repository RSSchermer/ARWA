use std::convert::TryFrom;
use std::ops::{Bound, RangeBounds};

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::error::{InvalidStateError, RangeError, SetTextRangeError};
use crate::html::{AutoComplete, GenericHtmlElement, HtmlElement, HtmlFormElement, Labels};
use crate::{
    Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node,
    SelectionDirection, TextWrap,
};

pub use web_sys::ValidityState;

#[derive(Clone)]
pub struct HtmlTextAreaElement {
    inner: web_sys::HtmlTextAreaElement,
}

impl HtmlTextAreaElement {
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

            pub fn placeholder(&self) -> String;

            pub fn set_placeholder(&self, placeholder: &str);

            pub fn read_only(&self) -> bool;

            pub fn set_read_only(&self, read_only: bool);

            pub fn required(&self) -> bool;

            pub fn set_required(&self, required: bool);

            pub fn text_length(&self) -> u32;

            pub fn cols(&self) -> u32;

            pub fn set_cols(&self, cols: u32);

            pub fn rows(&self) -> u32;

            pub fn set_rows(&self, rows: u32);

            pub fn will_validate(&self) -> bool;

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn set_custom_validity(&self, error: &str);

            pub fn validity(&self) -> ValidityState;

            pub fn select(&self);
        }
    }

    pub fn default_value(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.default_value().unwrap()
    }

    pub fn set_default_value(&self, default_value: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.set_default_value(default_value).unwrap();
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

    pub fn wrap(&self) -> TextWrap {
        match &*self.inner.wrap() {
            "hard" => TextWrap::Hard,
            _ => TextWrap::Soft,
        }
    }

    pub fn set_wrap(&self, wrap: TextWrap) {
        let wrap = match wrap {
            TextWrap::Hard => "hard",
            TextWrap::Soft => "soft",
        };

        self.inner.set_wrap(wrap);
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

    pub fn validation_message(&self) -> String {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.inner.validation_message().unwrap()
    }

    pub fn labels(&self) -> Labels {
        Labels::new(self.inner.labels())
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

impl_html_common_traits!(HtmlTextAreaElement);
