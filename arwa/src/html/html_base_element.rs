use delegate::delegate;

use crate::dom::impl_try_from_element;
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};
use crate::url::Url;

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
        Url::parse(self.inner.href().as_ref()).ok()
    }

    pub fn set_href(&self, href: &Url) {
        self.inner.set_href(href.as_ref());
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
impl_extendable_element!(HtmlBaseElement, "base");
