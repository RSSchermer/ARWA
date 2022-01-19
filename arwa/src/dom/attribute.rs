use crate::dom::DynamicElement;
use delegate::delegate;

#[derive(Clone)]
pub struct Attribute {
    inner: web_sys::Attr,
}

impl Attribute {
    pub(crate) fn new(inner: web_sys::Attr) -> Self {
        Attribute { inner }
    }

    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn namespace_uri(&self) -> Option<String>;

            pub fn local_name(&self) -> String;

            pub fn prefix(&self) -> Option<String>;

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }

    pub fn owner_element(&self) -> Option<DynamicElement> {
        todo!("Missing in web-sys")
        //self.inner.owner_element().map(|e| e.into())
    }
}

impl AsRef<web_sys::Attr> for Attribute {
    fn as_ref(&self) -> &web_sys::Attr {
        &self.inner
    }
}

impl From<web_sys::Attr> for Attribute {
    fn from(inner: web_sys::Attr) -> Self {
        Attribute { inner }
    }
}

impl_node_traits!(Attribute, web_sys::Attr);

#[derive(Clone)]
pub struct InvalidAttributeName {
    inner: web_sys::DomException,
}

impl InvalidAttributeName {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        InvalidAttributeName { inner }
    }
}
