use crate::cssom::{link_style_seal, CssStyleSheet, LinkStyle};

#[derive(Clone)]
pub struct HtmlStyleElement {
    inner: web_sys::HtmlStyleElement,
}

impl HtmlStyleElement {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);
        }
    }

    // TODO: media
}

impl link_style_seal::Seal for HtmlStyleElement {}

impl LinkStyle for HtmlStyleElement {
    fn sheet(&self) -> Option<CssStyleSheet> {
        self.inner
            .sheet()
            .map(|s| CssStyleSheet::from(s.unchecked_into()))
    }
}

impl From<web_sys::HtmlStyleElement> for HtmlStyleElement {
    fn from(inner: web_sys::HtmlStyleElement) -> Self {
        HtmlStyleElement { inner }
    }
}

impl AsRef<web_sys::HtmlStyleElement> for HtmlStyleElement {
    fn as_ref(&self) -> &web_sys::HtmlStyleElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlStyleElement);
impl_try_from_element!(HtmlStyleElement);
impl_known_element!(HtmlStyleElement, "STYLE");
