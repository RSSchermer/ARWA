#[derive(Clone)]
pub struct HtmlColgroupElement {
    inner: web_sys::HtmlTableColElement,
}

impl HtmlColgroupElement {
    delegate! {
        target self.inner {
            pub fn span(&self) -> u32;

            pub fn set_span(&self, span: u32);
        }
    }
}

impl From<web_sys::HtmlTableColElement> for HtmlColgroupElement {
    fn from(inner: web_sys::HtmlTableColElement) -> Self {
        HtmlColgroupElement { inner }
    }
}

impl AsRef<web_sys::HtmlTableColElement> for HtmlColgroupElement {
    fn as_ref(&self) -> &web_sys::HtmlTableColElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlColgroupElement);
impl_try_from_element_traits!(HtmlColgroupElement, web_sys::HtmlTableColElement);
impl_known_element!(HtmlColgroupElement, "COLGROUP");
