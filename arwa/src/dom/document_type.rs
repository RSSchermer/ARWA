use delegate::delegate;

use crate::console::{Write, Writer};
use crate::dom::{child_node_seal, ChildNode, DocumentFragment, HierarchyRequestError};
use crate::Node;

pub struct DocumentType {
    inner: web_sys::DocumentType,
}

impl DocumentType {
    delegate! {
        target self.inner {
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

impl_node_traits!(DocumentType, web_sys::DocumentType);

impl child_node_seal::Seal for DocumentType {
    fn as_web_sys_node(&self) -> &web_sys::Node {
        self.as_ref()
    }
}

impl ChildNode for DocumentType {
    fn disconnect(&self) {
        self.inner.remove();
    }

    fn replace_with<T>(&self, replacement: &T)
    where
        T: ChildNode,
    {
        self.inner
            .replace_with_with_node_1(replacement.as_web_sys_node())
            .unwrap_throw();
    }

    fn try_replace_with<T>(&self, replacement: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.inner
            .replace_with_with_node_1(replacement.as_web_sys_node())
            .map_err(|err| HierarchyRequestError::new(err.into()))
    }

    fn before_insert_node<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        self.inner
            .before_with_node_1(node.as_web_sys_node())
            .unwrap_throw();
    }

    fn try_before_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.inner
            .before_with_node_1(node.as_web_sys_node())
            .map_err(|err| Hierarchy_request_error::new(err.unchecked_into()))
    }

    fn before_insert_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment,
    {
        self.inner
            .before_with_node_1(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw();
    }

    fn after_insert_node<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        self.inner
            .after_with_node_1(node.as_web_sys_node())
            .unwrap_throw();
    }

    fn try_after_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.inner
            .after_with_node_1(node.as_web_sys_node())
            .map_err(|err| Hierarchy_request_error::new(err.unchecked_into()))
    }

    fn after_insert_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment,
    {
        self.inner
            .after_with_node_1(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw();
    }
}
