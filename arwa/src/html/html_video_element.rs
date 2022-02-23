use delegate::delegate;

use crate::html::{impl_html_media_element_traits, impl_known_element};

#[derive(Clone)]
pub struct HtmlVideoElement {
    inner: web_sys::HtmlVideoElement,
}

impl HtmlVideoElement {
    delegate! {
        target self.inner {
            pub fn width(&self) -> u32;

            pub fn set_width(&self, width: u32);

            pub fn height(&self) -> u32;

            pub fn set_height(&self, height: u32);

            pub fn video_width(&self) -> u32;

            pub fn video_height(&self) -> u32;

            pub fn poster(&self) -> String;

            pub fn set_poster(&self, poster: &str);
        }
    }
}

impl From<web_sys::HtmlVideoElement> for HtmlVideoElement {
    fn from(inner: web_sys::HtmlVideoElement) -> Self {
        HtmlVideoElement { inner }
    }
}

impl AsRef<web_sys::HtmlVideoElement> for HtmlVideoElement {
    fn as_ref(&self) -> &web_sys::HtmlVideoElement {
        &self.inner
    }
}

impl_html_media_element_traits!(HtmlVideoElement);
impl_known_element!(HtmlVideoElement, "VIDEO");
