use std::str::FromStr;

use crate::dom::impl_try_from_element;
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};
use crate::media_type::MediaType;
use crate::url::Url;

#[derive(Clone)]
pub struct HtmlEmbedElement {
    inner: web_sys::HtmlEmbedElement,
}

impl HtmlEmbedElement {
    pub fn src(&self) -> Option<Url> {
        Url::parse(self.inner.src().as_ref()).ok()
    }

    pub fn set_src(&self, src: &Url) {
        self.inner.set_src(src.as_ref());
    }

    pub fn width(&self) -> Option<u32> {
        u32::from_str(&self.inner.width()).ok()
    }

    pub fn set_width(&self, width: Option<u32>) {
        self.inner
            .set_width(&width.map(|w| w.to_string()).unwrap_or(String::new()));
    }

    pub fn height(&self) -> Option<u32> {
        u32::from_str(&self.inner.height()).ok()
    }

    pub fn set_height(&self, height: Option<u32>) {
        self.inner
            .set_height(&height.map(|w| w.to_string()).unwrap_or(String::new()));
    }

    pub fn media_type(&self) -> Option<MediaType> {
        MediaType::parse(self.inner.type_().as_ref()).ok()
    }

    pub fn set_media_type(&self, media_type: Option<&MediaType>) {
        self.inner
            .set_type(media_type.map(|m| m.as_ref()).unwrap_or(""));
    }
}

impl From<web_sys::HtmlEmbedElement> for HtmlEmbedElement {
    fn from(inner: web_sys::HtmlEmbedElement) -> Self {
        HtmlEmbedElement { inner }
    }
}

impl AsRef<web_sys::HtmlEmbedElement> for HtmlEmbedElement {
    fn as_ref(&self) -> &web_sys::HtmlEmbedElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlEmbedElement);
impl_try_from_element!(HtmlEmbedElement);
impl_known_element!(HtmlEmbedElement, "EMBED");
impl_extendable_element!(HtmlEmbedElement, "embed");
