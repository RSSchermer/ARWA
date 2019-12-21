use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::html::{
    AutoComplete, GenericHtmlElement, HtmlElement, HtmlFormElement, HtmlOptGroupElement,
    HtmlOptionElement, Labels,
};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

pub use web_sys::ValidityState;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectType {
    SelectOne,
    SelectMultiple,
}

#[derive(Clone)]
pub struct HtmlSelectElement {
    inner: web_sys::HtmlSelectElement,
}

impl HtmlSelectElement {
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

            pub fn multiple(&self) -> bool;

            pub fn set_multiple(&self, multiple: bool);

            pub fn required(&self) -> bool;

            pub fn set_required(&self, required: bool);

            pub fn size(&self) -> u32;

            pub fn set_size(&self, size: u32);

            pub fn will_validate(&self) -> bool;

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn set_custom_validity(&self, error: &str);

            pub fn validity(&self) -> ValidityState;
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        let index = self.inner.selected_index();

        if index > 0 {
            Some(index as usize)
        } else {
            None
        }
    }

    pub fn set_selected_index(&self, index: Option<usize>) {
        let index = if let Some(index) = index {
            index as i32
        } else {
            -1
        };

        self.inner.set_selected_index(index);
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

    pub fn select_type(&self) -> SelectType {
        match &*self.inner.type_() {
            "select-one" => SelectType::SelectOne,
            "select-multiple" => SelectType::SelectMultiple,
            _ => unreachable!(),
        }
    }

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| HtmlFormElement::from(form))
    }

    pub fn validation_message(&self) -> String {
        // There's no indication in the spec that this can actually fail, unwrap for now.
        self.inner.validation_message().unwrap()
    }

    pub fn labels(&self) -> Labels {
        Labels::new(self.inner.labels())
    }

    pub fn options(&self) -> SelectOptions {
        SelectOptions {
            select: &self.inner,
        }
    }

    pub fn selected_options(&self) -> SelectSelectedOptions {
        SelectSelectedOptions {
            inner: self.inner.selected_options(),
        }
    }
}

impl_html_common_traits!(HtmlSelectElement);

pub enum SelectInsertable<'a> {
    Option(&'a HtmlOptionElement),
    OptGroup(&'a HtmlOptGroupElement),
}

impl<'a> From<&'a HtmlOptionElement> for SelectInsertable<'a> {
    fn from(option: &'a HtmlOptionElement) -> Self {
        SelectInsertable::Option(option)
    }
}

impl<'a> From<&'a HtmlOptGroupElement> for SelectInsertable<'a> {
    fn from(opt_group: &'a HtmlOptGroupElement) -> Self {
        SelectInsertable::OptGroup(opt_group)
    }
}

#[derive(Clone, Copy)]
pub struct SelectOptions<'a> {
    select: &'a web_sys::HtmlSelectElement,
}

impl<'a> SelectOptions<'a> {
    pub fn get(&self, index: usize) -> Option<HtmlOptionElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.select.get(index))
            .map(|e| {
                let e: web_sys::HtmlOptionElement = e.unchecked_into();

                HtmlOptionElement::from(e)
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlOptionElement> {
        self.select
            .named_item(id)
            .map(|e| HtmlOptionElement::from(e))
    }

    pub fn len(&self) -> usize {
        self.select.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<HtmlOptionElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlOptionElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    // TODO: panics seems appropriate here for the hierarchy exceptions? This seems to be the same
    // class of error as index bounds error which std tends to handle with a panic.

    pub fn prepend<E>(&self, item: E)
    where
        for<'b> E: Into<SelectInsertable<'b>>,
    {
        match item.into() {
            SelectInsertable::Option(option) => self
                .select
                .add_with_html_option_element_and_opt_i32(option.as_ref(), Some(0)),
            SelectInsertable::OptGroup(opt_group) => self
                .select
                .add_with_html_opt_group_element_and_opt_i32(opt_group.as_ref(), Some(0)),
        }
        .expect("Element cannot be an ancestor of the element into which it is being inserted.");
    }

    pub fn append<E>(&self, item: E)
    where
        for<'b> E: Into<SelectInsertable<'b>>,
    {
        match item.into() {
            SelectInsertable::Option(option) => {
                self.select.add_with_html_option_element(option.as_ref())
            }
            SelectInsertable::OptGroup(opt_group) => self
                .select
                .add_with_html_opt_group_element(opt_group.as_ref()),
        }
        .expect("Element cannot be an ancestor of the element into which it is being inserted.");
    }

    pub fn insert<E>(&self, index: usize, item: E)
    where
        for<'b> E: Into<SelectInsertable<'b>>,
    {
        if index > std::i32::MAX as usize || index >= self.len() {
            panic!("Index out of bounds");
        }

        match item.into() {
            SelectInsertable::Option(option) => self
                .select
                .add_with_html_option_element_and_opt_i32(option.as_ref(), Some(index as i32)),
            SelectInsertable::OptGroup(opt_group) => {
                self.select.add_with_html_opt_group_element_and_opt_i32(
                    opt_group.as_ref(),
                    Some(index as i32),
                )
            }
        }
        .expect("Element cannot be an ancestor of the element into which it is being inserted.");
    }

    // TODO: insert_before with reference to option/optgroup? Note that HtmlOptionElement does
    // define an `index` method, so this would only act as a convenience method, the behaviour
    // is easily reproduced with `insert(before.index(), item)` (although at the cost of an extra
    // browser API call).

    pub fn remove(&self, index: usize) {
        // TODO: I recall other instances where remove can error if the index is out of bounds,
        // should we make this consistent across the crate or conform the spec? Do for now.
        if index > std::i32::MAX as usize || index >= self.len() {
            panic!("Index out of bounds");
        }

        self.select.remove_with_index(index as i32)
    }
}

impl<'a> IntoIterator for SelectOptions<'a> {
    type Item = HtmlOptionElement;
    type IntoIter = SelectOptionsIntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SelectOptionsIntoIter {
            select_options: self,
            current: 0,
        }
    }
}

pub struct SelectOptionsIntoIter<'a> {
    select_options: SelectOptions<'a>,
    current: usize,
}

impl<'a> Iterator for SelectOptionsIntoIter<'a> {
    type Item = HtmlOptionElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.select_options.get(current)
    }
}

pub struct SelectSelectedOptions {
    inner: web_sys::HtmlCollection,
}

impl SelectSelectedOptions {
    pub fn get(&self, index: usize) -> Option<HtmlOptionElement> {
        u32::try_from(index)
            .ok()
            .and_then(|index| self.inner.get_with_index(index))
            .map(|e| {
                let e: web_sys::HtmlOptionElement = e.unchecked_into();

                HtmlOptionElement::from(e)
            })
    }

    pub fn find_by_id(&self, id: &str) -> Option<HtmlOptionElement> {
        self.inner.get_with_name(id).map(|e| {
            let e: web_sys::HtmlOptionElement = e.unchecked_into();

            HtmlOptionElement::from(e)
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

    pub fn first(&self) -> Option<HtmlOptionElement> {
        self.get(0)
    }

    pub fn last(&self) -> Option<HtmlOptionElement> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> SelectSelectedOptionsIter {
        SelectSelectedOptionsIter {
            select_selected_options: self,
            current: 0,
        }
    }
}

impl IntoIterator for SelectSelectedOptions {
    type Item = HtmlOptionElement;
    type IntoIter = SelectSelectedOptionsIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        SelectSelectedOptionsIntoIter {
            select_selected_options: self,
            current: 0,
        }
    }
}

pub struct SelectSelectedOptionsIter<'a> {
    select_selected_options: &'a SelectSelectedOptions,
    current: usize,
}

impl<'a> Iterator for SelectSelectedOptionsIter<'a> {
    type Item = HtmlOptionElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.select_selected_options.get(current)
    }
}

pub struct SelectSelectedOptionsIntoIter {
    select_selected_options: SelectSelectedOptions,
    current: usize,
}

impl Iterator for SelectSelectedOptionsIntoIter {
    type Item = HtmlOptionElement;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.select_selected_options.get(current)
    }
}
