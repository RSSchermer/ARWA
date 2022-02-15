use crate::html::HtmlFormElement;

#[derive(Clone)]
pub struct HtmlOptionElement {
    inner: web_sys::HtmlOptionElement,
}

impl HtmlOptionElement {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);

            pub fn label(&self) -> String;

            pub fn set_label(&self, label: &str);

            pub fn default_selected(&self) -> bool;

            pub fn set_default_selected(&self, default_selected: bool);

            pub fn selected(&self) -> bool;

            pub fn set_selected(&self, selected: bool);

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);

            pub fn text(&self) -> String;

            pub fn set_text(&self, text: &str);
        }
    }

    pub fn form(&self) -> Option<HtmlFormElement> {
        self.inner.form().map(|form| form.into())
    }

    pub fn index(&self) -> usize {
        self.inner.index() as usize
    }
}

impl From<web_sys::HtmlOptionElement> for HtmlOptionElement {
    fn from(inner: web_sys::HtmlOptionElement) -> Self {
        HtmlOptionElement { inner }
    }
}

impl AsRef<web_sys::HtmlOptionElement> for HtmlOptionElement {
    fn as_ref(&self) -> &web_sys::HtmlOptionElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlOptionElement);
impl_try_from_element!(HtmlOptionElement);
impl_known_element!(HtmlOptionElement, "OPTION");
