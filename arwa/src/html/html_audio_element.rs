use crate::html::{impl_html_media_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlAudioElement {
    inner: web_sys::HtmlAudioElement,
}

impl From<web_sys::HtmlAudioElement> for HtmlAudioElement {
    fn from(inner: web_sys::HtmlAudioElement) -> Self {
        HtmlAudioElement { inner }
    }
}

impl AsRef<web_sys::HtmlAudioElement> for HtmlAudioElement {
    fn as_ref(&self) -> &web_sys::HtmlAudioElement {
        &self.inner
    }
}

impl_html_media_element_traits!(HtmlAudioElement);
impl_known_element!(HtmlAudioElement, "AUDIO");
