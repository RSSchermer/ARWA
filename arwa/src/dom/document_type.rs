use delegate::delegate;

use crate::dom::{
    impl_child_node, impl_node_traits, impl_owned_node, impl_try_from_child_node,
    impl_try_from_node,
};

pub struct DocumentType {
    inner: web_sys::DocumentType,
}

impl DocumentType {
    delegate! {
        to self.inner {
            pub fn name(&self) -> String;

            pub fn public_id(&self) -> String;

            pub fn system_id(&self) -> String;
        }
    }
}

impl From<web_sys::DocumentType> for DocumentType {
    fn from(inner: web_sys::DocumentType) -> Self {
        DocumentType { inner }
    }
}

impl AsRef<web_sys::DocumentType> for DocumentType {
    fn as_ref(&self) -> &web_sys::DocumentType {
        &self.inner
    }
}

impl_node_traits!(DocumentType);
impl_owned_node!(DocumentType);
impl_try_from_node!(DocumentType);
impl_child_node!(DocumentType);
impl_try_from_child_node!(DocumentType);
