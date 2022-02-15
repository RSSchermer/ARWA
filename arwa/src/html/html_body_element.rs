#[derive(Clone)]
pub struct HtmlBodyElement {
    inner: web_sys::HtmlBodyElement,
}

// Note: ignoring window event handlers on the body element, the actual Window objects seems the
// more appropriate target for these events.

impl From<web_sys::HtmlBodyElement> for HtmlBodyElement {
    fn from(inner: web_sys::HtmlBodyElement) -> Self {
        HtmlBodyElement { inner }
    }
}

impl AsRef<web_sys::HtmlBodyElement> for HtmlBodyElement {
    fn as_ref(&self) -> &web_sys::HtmlBodyElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlBodyElement);
impl_try_from_element!(HtmlBodyElement);
impl_known_element!(HtmlBodyElement, "BODY");
impl_shadow_host_for_element!(HtmlBodyElement);
