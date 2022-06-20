use delegate::delegate;

use crate::dom::{
    impl_node_traits, impl_owned_node, impl_try_from_node, DynamicElement, Name, NonColonName,
};
use wasm_bindgen::UnwrapThrowExt;

/// Represents an attribute on an [Element] node.
///
/// Associates a [name] with a [value] string. The attribute may be associated with a namespace, in
/// which case it has an associated [namespace_uri] and may have [prefix] that acts as a shorthand
/// for the namespace URI.
#[derive(Clone)]
pub struct Attribute {
    inner: web_sys::Attr,
}

impl Attribute {
    pub(crate) fn new(inner: web_sys::Attr) -> Self {
        Attribute { inner }
    }

    delegate! {
        to self.inner {
            /// The URI of the associated namespace, or `None` if the attribute is not associated
            /// with a namespace.
            pub fn namespace_uri(&self) -> Option<String>;

            /// Returns the value string of this attribute.
            pub fn value(&self) -> String;

            /// Sets the value of this attribute to the given string.
            pub fn set_value(&self, value: &str);
        }
    }

    /// The qualified name of the attribute.
    ///
    /// If this attribute has a namespace prefix, then this will be of the form `prefix:local_name`.
    /// Otherwise, the name will be identical to the [local_name].
    pub fn name(&self) -> Name {
        Name::trusted(self.inner.name().into())
    }

    /// The local portion of the attribute name.
    ///
    /// The qualified name of the attribute without a namespace prefix. For example, the local-name
    /// portion of a qualified name of the form `prefix:local_name` would be `local_name`. If the
    /// attribute does not have a namespace prefix, then the [local_name] is the same as the [name].
    pub fn local_name(&self) -> NonColonName {
        NonColonName::parse(self.inner.local_name().as_ref()).unwrap_throw()
    }

    /// The namespace prefix associated with the attribute, if any.
    ///
    /// Acts as a convenient shorthand for a namespace. In a qualified name of the form
    /// `prefix:local_name`, the prefix would be `prefix`.
    pub fn prefix(&self) -> Option<NonColonName> {
        self.inner.prefix().map(|n| NonColonName::trusted(n))
    }

    /// Returns a reference to the [Element] node that owns this attribute, or `None` if this
    /// attribute is not associated with an element.
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
