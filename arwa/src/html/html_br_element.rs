#[derive(Clone)]
pub struct HtmlBrElement {
    inner: web_sys::HtmlBrElement,
}

impl From<web_sys::HtmlBrElement> for HtmlBrElement {
    fn from(inner: web_sys::HtmlBrElement) -> Self {
        HtmlBrElement { inner }
    }
}

impl AsRef<web_sys::HtmlBrElement> for HtmlBrElement {
    fn as_ref(&self) -> &web_sys::HtmlBrElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlBrElement);
impl_try_from_element!(HtmlBrElement);
impl_known_element!(HtmlBrElement, "BR");
