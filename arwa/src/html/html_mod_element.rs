use crate::dom::impl_try_from_element_with_tag_check;
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};

mod html_mod_element_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_html_mod_element(&self) -> &web_sys::HtmlModElement;
    }
}

pub trait HtmlModElement: html_mod_element_seal::Seal {
    fn cite(&self) -> String {
        self.as_web_sys_html_mod_element().cite()
    }

    fn set_cite(&self, cite: &str) {
        self.as_web_sys_html_mod_element().set_cite(cite);
    }

    // TODO: date_time
}

#[derive(Clone)]
pub struct HtmlDelElement {
    inner: web_sys::HtmlModElement,
}

impl html_mod_element_seal::Seal for HtmlDelElement {
    fn as_web_sys_html_mod_element(&self) -> &web_sys::HtmlModElement {
        &self.inner
    }
}

impl HtmlModElement for HtmlDelElement {}

impl AsRef<web_sys::HtmlModElement> for HtmlDelElement {
    fn as_ref(&self) -> &web_sys::HtmlModElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlDelElement);
impl_try_from_element_with_tag_check!(HtmlDelElement, HtmlModElement, "DEL");
impl_known_element!(HtmlDelElement, HtmlModElement, "DEL");
impl_extendable_element!(HtmlDelElement, "del");

#[derive(Clone)]
pub struct HtmlInsElement {
    inner: web_sys::HtmlModElement,
}

impl html_mod_element_seal::Seal for HtmlInsElement {
    fn as_web_sys_html_mod_element(&self) -> &web_sys::HtmlModElement {
        &self.inner
    }
}

impl HtmlModElement for HtmlInsElement {}

impl AsRef<web_sys::HtmlModElement> for HtmlInsElement {
    fn as_ref(&self) -> &web_sys::HtmlModElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlInsElement);
impl_try_from_element_with_tag_check!(HtmlInsElement, HtmlModElement, "INS");
impl_known_element!(HtmlInsElement, HtmlModElement, "INS");
impl_extendable_element!(HtmlInsElement, "ins");
