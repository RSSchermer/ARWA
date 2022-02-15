use crate::dom::GenericDocumentFragment;

#[derive(Clone)]
pub struct HtmlTemplateElement {
    inner: web_sys::HtmlTemplateElement,
}

impl HtmlTemplateElement {
    pub fn content(&self) -> GenericDocumentFragment {
        self.inner.content().into()
    }
}

impl From<web_sys::HtmlTemplateElement> for HtmlTemplateElement {
    fn from(inner: web_sys::HtmlTemplateElement) -> Self {
        HtmlTemplateElement { inner }
    }
}

impl AsRef<web_sys::HtmlTemplateElement> for HtmlTemplateElement {
    fn as_ref(&self) -> &web_sys::HtmlTemplateElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTemplateElement);
impl_try_from_element!(HtmlTemplateElement);
impl_known_element!(HtmlTemplateElement, "TEMPLATE");
