use std::cell::{RefCell, RefMut};
use std::convert::TryFrom;

use crate::collection::{Collection, Sequence};
use crate::dom::{impl_try_from_element, impl_try_from_element_with_tag_check, Token};
use crate::html::{impl_html_element_traits, impl_known_element};
use crate::InvalidCast;

mod table_cell_element_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_html_table_cell_element(&self) -> &web_sys::HtmlTableCellElement;
    }
}

pub trait TableCellElement: table_cell_element_seal::Seal {
    fn col_span(&self) -> u32 {
        self.as_web_sys_html_table_cell_element().col_span()
    }

    fn set_col_span(&self, col_span: u32) {
        self.as_web_sys_html_table_cell_element()
            .set_col_span(col_span);
    }

    fn row_span(&self) -> u32 {
        self.as_web_sys_html_table_cell_element().row_span()
    }

    fn set_row_span(&self, row_span: u32) {
        self.as_web_sys_html_table_cell_element()
            .set_row_span(row_span);
    }

    fn cell_index(&self) -> Option<u32> {
        let cell_index = self.as_web_sys_html_table_cell_element().cell_index();

        if cell_index < 0 {
            None
        } else {
            Some(cell_index as u32)
        }
    }

    // TODO: `scope` missing in web_sys
}

// Note: MDN documents this as a DOMTokenList, the spec defines it as a string of white space
// separated unique tokens (the exact same language used elsewhere for DOMTokenLists). web_sys
// exposes this as a String. Emulating a live DOMTokenList here with a caching Vec. Note that
// during iteration we have to check if we need to refresh at every step to be sound.

struct TableCellHeadersCache {
    raw: String,
    parsed: Vec<String>,
}

impl TableCellHeadersCache {
    fn refresh(&mut self, headers_string: String) {
        if self.raw != headers_string {
            let mut parsed_new = Vec::new();

            for token in headers_string.split_ascii_whitespace() {
                if !parsed_new.iter().any(|e| e == token) {
                    parsed_new.push(token.to_string());
                }
            }

            self.parsed = parsed_new;
            self.raw = headers_string;
        }
    }

    fn contains(&self, header: &str) -> bool {
        self.parsed.iter().any(|h| h == header)
    }

    fn serialize(&self) -> String {
        self.parsed.join(" ")
    }
}

pub struct TableCellHeaders {
    cell: web_sys::HtmlTableCellElement,
    cached: RefCell<TableCellHeadersCache>,
}

impl TableCellHeaders {
    fn refresh(&self) -> RefMut<TableCellHeadersCache> {
        let mut cached = self.cached.borrow_mut();

        cached.refresh(self.cell.headers());

        cached
    }

    pub fn contains(&self, header: &Token) -> bool {
        self.refresh().contains(header.as_ref())
    }

    pub fn insert(&self, header: &Token) -> bool {
        let mut cached = self.refresh();

        if !cached.contains(header.as_ref()) {
            cached.parsed.push(header.to_string());

            let new_headers = cached.serialize();

            self.cell.set_headers(&new_headers);

            cached.raw = new_headers;

            true
        } else {
            false
        }
    }

    pub fn remove(&self, header: &Token) -> bool {
        let mut cached = self.refresh();

        if cached.contains(header.as_ref()) {
            cached.parsed.retain(|h| h.as_str() != header);

            let new_headers = cached.serialize();

            self.cell.set_headers(&new_headers);

            cached.raw = new_headers;

            true
        } else {
            false
        }
    }

    pub fn toggle(&self, header: &Token) -> bool {
        let mut cached = self.refresh();

        let output = if cached.contains(header.as_ref()) {
            cached.parsed.retain(|h| h.as_str() != header);

            false
        } else {
            cached.parsed.push(header.to_string());

            true
        };

        let new_headers = cached.serialize();

        self.cell.set_headers(&new_headers);

        cached.raw = new_headers;

        output
    }

    pub fn replace(&self, old: &Token, new: &Token) -> bool {
        let mut cached = self.refresh();

        let mut did_replace = false;

        for header in cached.parsed.iter_mut() {
            if header.as_str() == old {
                *header = new.to_string();

                did_replace = true;

                break;
            }
        }

        if did_replace {
            let new_headers = cached.serialize();

            self.cell.set_headers(&new_headers);

            cached.raw = new_headers;

            true
        } else {
            false
        }
    }

    pub fn serialize(&self) -> String {
        self.refresh().serialize()
    }

    pub fn deserialize(&self, serialized: &str) {
        let serialized = serialized.to_string();

        let mut cached = self.cached.borrow_mut();

        cached.refresh(serialized);

        self.cell.set_headers(cached.serialize().as_ref());
    }
}

impl Collection for TableCellHeaders {
    fn len(&self) -> u32 {
        self.refresh().parsed.len() as u32
    }
}

impl Sequence for TableCellHeaders {
    type Item = Token;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.refresh()
            .parsed
            .get(index as usize)
            .map(|t| Token::trusted(t.clone()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::JsString::from(self.refresh().serialize()).split(" ")
    }
}

#[derive(Clone)]
pub struct DynamicTableCellElement {
    inner: web_sys::HtmlTableCellElement,
}

impl table_cell_element_seal::Seal for DynamicTableCellElement {
    fn as_web_sys_html_table_cell_element(&self) -> &web_sys::HtmlTableCellElement {
        &self.inner
    }
}

impl TableCellElement for DynamicTableCellElement {}

impl AsRef<web_sys::HtmlTableCellElement> for DynamicTableCellElement {
    fn as_ref(&self) -> &web_sys::HtmlTableCellElement {
        &self.inner
    }
}

impl From<web_sys::HtmlTableCellElement> for DynamicTableCellElement {
    fn from(inner: web_sys::HtmlTableCellElement) -> Self {
        DynamicTableCellElement { inner }
    }
}

impl_html_element_traits!(DynamicTableCellElement);
impl_try_from_element!(DynamicTableCellElement, HtmlTableCellElement);

#[derive(Clone)]
pub struct HtmlTdElement {
    inner: web_sys::HtmlTableCellElement,
}

impl table_cell_element_seal::Seal for HtmlTdElement {
    fn as_web_sys_html_table_cell_element(&self) -> &web_sys::HtmlTableCellElement {
        &self.inner
    }
}

impl TableCellElement for HtmlTdElement {}

impl AsRef<web_sys::HtmlTableCellElement> for HtmlTdElement {
    fn as_ref(&self) -> &web_sys::HtmlTableCellElement {
        &self.inner
    }
}

impl TryFrom<DynamicTableCellElement> for HtmlTdElement {
    type Error = InvalidCast<DynamicTableCellElement, HtmlTdElement>;

    fn try_from(value: DynamicTableCellElement) -> Result<Self, Self::Error> {
        let table_cell_element = value.inner;

        if table_cell_element.tag_name().as_str() == "TD" {
            Ok(HtmlTdElement {
                inner: table_cell_element,
            })
        } else {
            Err(InvalidCast::new(DynamicTableCellElement {
                inner: table_cell_element,
            }))
        }
    }
}

impl_html_element_traits!(HtmlTdElement);
impl_try_from_element_with_tag_check!(HtmlTdElement, HtmlTableCellElement, "TD");
impl_known_element!(HtmlTdElement, HtmlTableCellElement, "TD");

#[derive(Clone)]
pub struct HtmlThElement {
    inner: web_sys::HtmlTableCellElement,
}

impl table_cell_element_seal::Seal for HtmlThElement {
    fn as_web_sys_html_table_cell_element(&self) -> &web_sys::HtmlTableCellElement {
        &self.inner
    }
}

impl TableCellElement for HtmlThElement {}

impl AsRef<web_sys::HtmlTableCellElement> for HtmlThElement {
    fn as_ref(&self) -> &web_sys::HtmlTableCellElement {
        &self.inner
    }
}

impl TryFrom<DynamicTableCellElement> for HtmlThElement {
    type Error = InvalidCast<DynamicTableCellElement, HtmlThElement>;

    fn try_from(value: DynamicTableCellElement) -> Result<Self, Self::Error> {
        let table_cell_element = value.inner;

        if table_cell_element.tag_name().as_str() == "TH" {
            Ok(HtmlThElement {
                inner: table_cell_element,
            })
        } else {
            Err(InvalidCast::new(DynamicTableCellElement {
                inner: table_cell_element,
            }))
        }
    }
}

impl_html_element_traits!(HtmlThElement);
impl_try_from_element_with_tag_check!(HtmlThElement, HtmlTableCellElement, "TH");
impl_known_element!(HtmlThElement, HtmlTableCellElement, "TH");
