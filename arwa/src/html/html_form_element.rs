use crate::collection::{Collection, Sequence};
use crate::html::{AutoComplete, DynamicFormListedElement};
use crate::url::{AbsoluteOrRelativeUrl, Url};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FormEncoding {
    UrlEncoded,
    FormData,
    Plain,
}

impl AsRef<str> for FormEncoding {
    fn as_ref(&self) -> &str {
        match self {
            FormEncoding::UrlEncoded => "application/x-www-form-urlencoded",
            FormEncoding::FormData => "multipart/form-data",
            FormEncoding::Plain => "text/plain",
        }
    }
}

impl Default for FormEncoding {
    fn default() -> Self {
        FormEncoding::UrlEncoded
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FormMethod {
    Get,
    Post,
    Dialog,
}

impl Default for FormMethod {
    fn default() -> Self {
        FormMethod::Get
    }
}

#[derive(Clone)]
pub struct HtmlFormElement {
    inner: web_sys::HtmlFormElement,
}

impl HtmlFormElement {
    // Note: ignoring acceptCharset, per the spec the only valid value is "UTF-8".

    delegate! {
        target self.inner {
            pub fn no_validate(&self) -> bool;

            pub fn set_no_validate(&self, no_validate: bool);

            pub fn target(&self) -> String;

            pub fn set_target(&self, target: &str);

            pub fn check_validity(&self) -> bool;

            pub fn report_validity(&self) -> bool;

            pub fn reset(&self);
        }
    }

    pub fn action(&self) -> Option<Url> {
        Url::parse(self.inner.action()).ok()
    }

    pub fn set_action<T>(&self, action: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_action(action.as_str());
    }

    pub fn autocomplete(&self) -> AutoComplete {
        match &*self.inner.autocomplete() {
            "off" => AutoComplete::Off,
            _ => AutoComplete::On,
        }
    }

    pub fn set_autocomplete(&self, autocomplete: AutoComplete) {
        let autocomplete = match autocomplete {
            AutoComplete::On => "on",
            AutoComplete::Off => "off",
        };

        self.inner.set_autocomplete(autocomplete);
    }

    pub fn encoding(&self) -> FormEncoding {
        match self.inner.encoding().as_ref() {
            "multipart/form-data" => FormEncoding::FormData,
            "text/plain" => FormEncoding::Plain,
            // Note: both the missing value default and the invalid value default are url-encoded.
            _ => FormEncoding::UrlEncoded,
        }
    }

    pub fn set_encoding(&self, encoding: FormEncoding) {
        self.inner.set_encoding(encoding.as_ref());
    }

    pub fn method(&self) -> FormMethod {
        match &*self.inner.method() {
            "post" => FormMethod::Post,
            "dialog" => FormMethod::Dialog,
            _ => FormMethod::Get,
        }
    }

    pub fn set_method(&self, method: FormMethod) {
        let method = match method {
            FormMethod::Get => "get",
            FormMethod::Post => "post",
            FormMethod::Dialog => "dialog",
        };

        self.inner.set_method(method);
    }

    pub fn elements(&self) -> FormElements {
        FormElements {
            inner: self.inner.elements().unchecked_into(),
        }
    }

    pub fn submit(&self) {
        // Despite the web_sys return type, I can find no indication in the spec that `submit` can
        // actually error, so just unwrap for now.
        self.inner.submit().unwrap();
    }
}

impl From<web_sys::HtmlFormElement> for HtmlFormElement {
    fn from(inner: web_sys::HtmlFormElement) -> Self {
        HtmlFormElement { inner }
    }
}

impl AsRef<web_sys::HtmlFormElement> for HtmlFormElement {
    fn as_ref(&self) -> &web_sys::HtmlFormElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlFormElement);
impl_try_from_element!(HtmlFormElement);
impl_known_element!(HtmlFormElement, "FORM");

pub struct FormElements {
    inner: web_sys::HtmlFormControlsCollection,
}

impl FormElements {
    fn radio(&self, name: &str) -> Option<Radio> {
        self.inner
            .named_item(name)
            .and_then(|o| o.dyn_into::<web_sys::RadioNodeList>().ok())
            .map(|inner| Radio { inner })
    }
}

impl Collection for FormElements {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for FormElements {
    type Item = DynamicFormListedElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get_with_index(index)
            .map(|e| DynamicFormListedElement::new(e.unchecked_into()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

pub struct Radio {
    inner: web_sys::RadioNodeList,
}

impl Radio {
    delegate! {
        target self.inner {
            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }
}

impl Collection for Radio {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for Radio {
    // Note: this is not explicitly specced to only be form-associated elements, but experimentation
    // in chromium and firefox suggests that it is in practice, even with elements that spec a
    // `name` attribute and are not form-associated elements (e.g. `iframe`).
    type Item = DynamicFormListedElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get(index)
            .map(|e| DynamicFormListedElement::new(e.unchecked_into()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}
