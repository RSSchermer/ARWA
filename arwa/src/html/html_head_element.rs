#[derive(Clone)]
pub struct HtmlHeadElement {
    inner: web_sys::HtmlHeadElement,
}

impl From<web_sys::HtmlHeadElement> for HtmlHeadElement {
    fn from(inner: web_sys::HtmlHeadElement) -> Self {
        HtmlHeadElement { inner }
    }
}

impl AsRef<web_sys::HtmlHeadElement> for HtmlHeadElement {
    fn as_ref(&self) -> &web_sys::HtmlHeadElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlHeadElement);
impl_try_from_element!(HtmlHeadElement);
impl_known_element!(HtmlHeadElement, "HEAD");
