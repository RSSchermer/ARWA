use crate::media_type::MediaType;
use crate::url::{AbsoluteOrRelativeUrl, Url};

#[derive(Clone)]
pub struct HtmlSourceElement {
    inner: web_sys::HtmlSourceElement,
}

impl HtmlSourceElement {
    pub fn src(&self) -> Option<Url> {
        Url::parse(self.inner.src()).ok()
    }

    pub fn set_src<T>(&self, src: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_src(src.as_str());
    }

    pub fn media_type(&self) -> Option<MediaType> {
        MediaType::parse(self.inner.type_().as_ref()).ok()
    }

    pub fn set_media_type(&self, media_type: Option<&MediaType>) {
        self.inner
            .set_type(media_type.map(|m| m.as_ref()).unwrap_or(""));
    }

    // TODO: media
}

impl From<web_sys::HtmlSourceElement> for HtmlSourceElement {
    fn from(inner: web_sys::HtmlSourceElement) -> Self {
        HtmlSourceElement { inner }
    }
}

impl AsRef<web_sys::HtmlSourceElement> for HtmlSourceElement {
    fn as_ref(&self) -> &web_sys::HtmlSourceElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlSourceElement);
impl_try_from_element!(HtmlSourceElement);
impl_known_element!(HtmlSourceElement, "SOURCE");
