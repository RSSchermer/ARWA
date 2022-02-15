use crate::url::{AbsoluteOrRelativeUrl, Url};

#[derive(Clone)]
pub struct HtmlBaseElement {
    inner: web_sys::HtmlBaseElement,
}

impl HtmlBaseElement {
    delegate! {
        target self.inner {
            pub fn target(&self) -> String;

            pub fn set_target(&self, target: &str);
        }
    }

    pub fn href(&self) -> Option<Url> {
        Url::parse(self.inner.href()).ok()
    }

    pub fn set_href<T>(&self, href: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_href(href.as_str());
    }
}

impl From<web_sys::HtmlBaseElement> for HtmlBaseElement {
    fn from(inner: web_sys::HtmlBaseElement) -> Self {
        HtmlBaseElement { inner }
    }
}

impl AsRef<web_sys::HtmlBaseElement> for HtmlBaseElement {
    fn as_ref(&self) -> &web_sys::HtmlBaseElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlBaseElement);
impl_try_from_element!(HtmlBaseElement);
impl_known_element!(HtmlBaseElement, "BASE");
