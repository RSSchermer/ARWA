#[derive(Clone)]
pub struct HtmlParagraphElement {
    inner: web_sys::HtmlParagraphElement,
}

impl From<web_sys::HtmlParagraphElement> for HtmlParagraphElement {
    fn from(inner: web_sys::HtmlParagraphElement) -> Self {
        HtmlParagraphElement { inner }
    }
}

impl AsRef<web_sys::HtmlParagraphElement> for HtmlParagraphElement {
    fn as_ref(&self) -> &web_sys::HtmlParagraphElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlParagraphElement);
impl_try_from_element!(HtmlParagraphElement);
impl_known_element!(HtmlParagraphElement, "P");
