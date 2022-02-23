use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlPictureElement {
    inner: web_sys::HtmlPictureElement,
}

impl From<web_sys::HtmlPictureElement> for HtmlPictureElement {
    fn from(inner: web_sys::HtmlPictureElement) -> Self {
        HtmlPictureElement { inner }
    }
}

impl AsRef<web_sys::HtmlPictureElement> for HtmlPictureElement {
    fn as_ref(&self) -> &web_sys::HtmlPictureElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlPictureElement);
impl_try_from_element!(HtmlPictureElement);
impl_known_element!(HtmlPictureElement, "PICTURE");
