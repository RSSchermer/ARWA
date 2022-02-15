use crate::collection::{Collection, Sequence};
use crate::html::HtmlTrElement;

mod table_section_element_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_html_table_section_element(&self) -> &web_sys::HtmlTableSectionElement;
    }
}

pub trait TableSectionElement: table_section_element_seal::Seal {
    fn rows(&self) -> TableSectionRows {
        TableSectionRows {
            inner: self.inner.rows(),
        }
    }
}

pub struct TableSectionRows {
    inner: web_sys::HtmlCollection,
}


impl Collection for TableSectionRows {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for TableSectionRows {
    type Item = HtmlTrElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get_with_index(index).map(|e| HtmlTableRowElement::from(e.unchecked_into()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

#[derive(Clone)]
pub struct HtmlTheadElement {
    inner: web_sys::HtmlTableSectionElement,
}

impl table_section_element_seal::Seal for HtmlTheadElement {
    fn as_web_sys_html_table_section_element(&self) -> &web_sys::HtmlTableSectionElement {
        &self.inner
    }
}

impl TableSectionElement for HtmlTheadElement {}

impl AsRef<web_sys::HtmlTableSectionElement> for HtmlTheadElement {
    fn as_ref(&self) -> &web_sys::HtmlTableSectionElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTheadElement);
impl_try_from_element_with_tag_check!(HtmlTheadElement, web_sys::HtmlTableSectionElement, "THEAD");
impl_known_element!(HtmlTheadElement, web_sys::HtmlTableSectionElement, "THEAD");

#[derive(Clone)]
pub struct HtmlTbodyElement {
    inner: web_sys::HtmlTableSectionElement,
}

impl table_section_element_seal::Seal for HtmlTbodyElement {
    fn as_web_sys_html_table_section_element(&self) -> &web_sys::HtmlTableSectionElement {
        &self.inner
    }
}

impl TableSectionElement for HtmlTbodyElement {}

impl AsRef<web_sys::HtmlTableSectionElement> for HtmlTbodyElement {
    fn as_ref(&self) -> &web_sys::HtmlTableSectionElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTbodyElement);
impl_try_from_element_with_tag_check!(HtmlTbodyElement, web_sys::HtmlTableSectionElement, "TBODY");
impl_known_element!(HtmlTbodyElement, web_sys::HtmlTableSectionElement, "TBODY");

#[derive(Clone)]
pub struct HtmlTfootElement {
    inner: web_sys::HtmlTableSectionElement,
}

impl table_section_element_seal::Seal for HtmlTfootElement {
    fn as_web_sys_html_table_section_element(&self) -> &web_sys::HtmlTableSectionElement {
        &self.inner
    }
}

impl TableSectionElement for HtmlTfootElement {}

impl AsRef<web_sys::HtmlTableSectionElement> for HtmlTfootElement {
    fn as_ref(&self) -> &web_sys::HtmlTableSectionElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTfootElement);
impl_try_from_element_with_tag_check!(HtmlTfootElement, web_sys::HtmlTableSectionElement, "TFOOT");
impl_known_element!(HtmlTfootElement, web_sys::HtmlTableSectionElement, "TFOOT");
