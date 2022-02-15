#[derive(Clone)]
pub struct HtmlUlElement {
    inner: web_sys::HtmlUListElement,
}

impl From<web_sys::HtmlUListElement> for HtmlUlElement {
    fn from(inner: web_sys::HtmlUListElement) -> Self {
        HtmlUlElement { inner }
    }
}

impl AsRef<web_sys::HtmlUListElement> for HtmlUlElement {
    fn as_ref(&self) -> &web_sys::HtmlUListElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlUlElement);
impl_try_from_element!(HtmlUlElement, web_sys::HtmlUListElement);
impl_known_element!(HtmlUlElement, web_sys::HtmlUListElement, "UL");