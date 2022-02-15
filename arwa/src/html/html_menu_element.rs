#[derive(Clone)]
pub struct HtmlMenuElement {
    inner: web_sys::HtmlMenuElement,
}

impl From<web_sys::HtmlMenuElement> for HtmlMenuElement {
    fn from(inner: web_sys::HtmlMenuElement) -> Self {
        HtmlMenuElement { inner }
    }
}

impl AsRef<web_sys::HtmlMenuElement> for HtmlMenuElement {
    fn as_ref(&self) -> &web_sys::HtmlMenuElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlMenuElement);
impl_try_from_element!(HtmlMenuElement);
impl_known_element!(HtmlMenuElement, "MENU");