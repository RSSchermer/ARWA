#[derive(Clone)]
pub struct HtmlMetaElement {
    inner: web_sys::HtmlMetaElement,
}

impl HtmlMetaElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);

            pub fn content(&self) -> String;

            pub fn set_content(&self, content: &str);
        }
    }
}

impl From<web_sys::HtmlMetaElement> for HtmlMetaElement {
    fn from(inner: web_sys::HtmlMetaElement) -> Self {
        HtmlMetaElement { inner }
    }
}

impl AsRef<web_sys::HtmlMetaElement> for HtmlMetaElement {
    fn as_ref(&self) -> &web_sys::HtmlMetaElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlMetaElement);
impl_try_from_element!(HtmlMetaElement);
impl_known_element!(HtmlMetaElement, "META");
