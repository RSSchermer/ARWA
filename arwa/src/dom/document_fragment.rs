use std::convert::TryFrom;

use wasm_bindgen::JsCast;

use crate::dom::selector::CompiledSelector;
use crate::dom::ChildNode;
use crate::dom::DynamicNode;
use crate::dom::HierarchyRequestError;
use crate::dom::{
    parent_node_seal, ChildElements, DynamicElement, ParentNode, QuerySelectorAll,
    QuerySelectorSyntaxError,
};

pub(crate) mod document_fragment_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_document_fragment(&self) -> &web_sys::DocumentFragment;
    }
}

pub trait DocumentFragment: document_fragment_seal::Seal {}

pub struct GenericDocumentFragment {
    inner: web_sys::DocumentFragment,
}

impl document_fragment_seal::Seal for GenericDocumentFragment {
    fn as_web_sys_document_fragment(&self) -> &web_sys::DocumentFragment {
        &self.inner
    }
}

impl DocumentFragment for GenericDocumentFragment {}

impl parent_node_seal::Seal for GenericDocumentFragment {
    fn as_web_sys_node(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}

impl ParentNode for GenericDocumentFragment {
    fn query_selector_first(&self, selector: &CompiledSelector) -> Option<DynamicElement> {
        self.inner
            .query_selector_first(selector.as_ref())
            .unwrap_throw()
            .map(|e| e.into())
    }

    fn query_selector_all(&self, selector: &CompiledSelector) -> QuerySelectorAll {
        QuerySelectorAll::new(
            self.inner
                .query_selector_all(selector.as_ref())
                .unwrap_trhwo(),
        )
    }

    fn child_elements(&self) -> ChildElements {
        ChildElements::new(self.inner.children())
    }

    fn prepend_child<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        self.inner
            .prepend_with_node_1(node.as_web_sys_node())
            .unwrap_throw();
    }

    fn try_prepend_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.inner
            .prepend_with_node_1(node.as_web_sys_node())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    fn prepend_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment,
    {
        self.inner
            .prepend_with_node_1(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw();
    }
}

impl From<web_sys::DocumentFragment> for GenericDocumentFragment {
    fn from(inner: web_sys::DocumentFragment) -> Self {
        GenericDocumentFragment { inner }
    }
}

impl AsRef<web_sys::DocumentFragment> for GenericDocumentFragment {
    fn as_ref(&self) -> &web_sys::DocumentFragment {
        &self.inner
    }
}

impl_node_traits!(DocumentFragment, web_sys::DocumentFragment);
