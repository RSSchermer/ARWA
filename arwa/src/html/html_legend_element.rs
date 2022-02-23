use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element, HtmlFormElement};

#[derive(Clone)]
pub struct HtmlLegendElement {
    inner: web_sys::HtmlLegendElement,
}

impl HtmlLegendElement {
    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }
}

impl From<web_sys::HtmlLegendElement> for HtmlLegendElement {
    fn from(inner: web_sys::HtmlLegendElement) -> Self {
        HtmlLegendElement { inner }
    }
}

impl AsRef<web_sys::HtmlLegendElement> for HtmlLegendElement {
    fn as_ref(&self) -> &web_sys::HtmlLegendElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlLegendElement);
impl_try_from_element!(HtmlLegendElement);
impl_known_element!(HtmlLegendElement, "LEGEND");
