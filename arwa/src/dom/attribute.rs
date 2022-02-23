use delegate::delegate;

use crate::dom::{
    impl_node_traits, impl_owned_node, impl_try_from_node, DynamicElement, Name, NonColonName,
};

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
            pub fn namespace_uri(&self) -> Option<String>;

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }

    pub fn name(&self) -> Name {
        Name::trusted(self.inner.name().into())
    }

    pub fn local_name(&self) -> Option<NonColonName> {
        NonColonName::parse(self.inner.local_name().as_ref()).ok()
    }

    pub fn prefix(&self) -> Option<NonColonName> {
        self.inner.prefix().map(|n| NonColonName::trusted(n))
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

impl_node_traits!(Attribute);
impl_owned_node!(Attribute);
impl_try_from_node!(Attribute, Attr);
