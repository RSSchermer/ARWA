#[derive(Clone)]
pub struct HtmlDlElement {
    inner: web_sys::HtmlDListElement,
}

impl From<web_sys::HtmlDListElement> for HtmlDlElement {
    fn from(inner: web_sys::HtmlDListElement) -> Self {
        HtmlDlElement { inner }
    }
}

impl AsRef<web_sys::HtmlDListElement> for HtmlDlElement {
    fn as_ref(&self) -> &web_sys::HtmlDListElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlDlElement);
impl_try_from_element!(HtmlDlElement, web_sys::HtmlDListElement);
impl_known_element!(HtmlDlElement, "DL");
