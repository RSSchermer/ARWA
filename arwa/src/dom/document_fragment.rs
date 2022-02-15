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

macro_rules! impl_document_fragment_traits {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl $crate::dom::document_fragment_seal::Seal for $tpe {
            fn as_web_sys_document_fragment(&self) -> &web_sys::DocumentFragment {
                &self.inner
            }
        }

        impl $crate::dom::DocumentFragment for $tpe {}

        impl $crate::dom::parent_node_seal::Seal for $tpe {
            fn as_web_sys_node(&self) -> &web_sys::Node {
                self.as_web_sys_document_fragment().as_ref()
            }
        }

        impl $crate::dom::ParentNode for $tpe {
            fn query_selector_first(
                &self,
                selector: &CompiledSelector,
            ) -> Option<$crate::dom::DynamicElement> {
                self.as_web_sys_document_fragment()
                    .query_selector_first(selector.as_ref())
                    .unwrap_throw()
                    .map(|e| e.into())
            }

            fn query_selector_all(
                &self,
                selector: &CompiledSelector,
            ) -> $crate::dom::QuerySelectorAll {
                QuerySelectorAll::new(
                    self.as_web_sys_document_fragment()
                        .query_selector_all(selector.as_ref())
                        .unwrap_trhwo(),
                )
            }

            fn child_elements(&self) -> $crate::dom::ChildElements {
                $crate::dom::ChildElements::new(self.as_web_sys_document_fragment().children())
            }

            fn prepend_child<T>(&self, node: &T)
            where
                T: $crate::dom::ChildNode,
            {
                self.as_web_sys_document_fragment()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_prepend_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
            where
                T: $crate::dom::ChildNode,
            {
                self.as_web_sys_document_fragment()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .map_err(|err| $crate::dom::HierarchyRequestError::new(err.unchecked_into()))
            }

            fn prepend_fragment<T>(&self, document_fragment: &T)
            where
                T: $crate::dom::DocumentFragment,
            {
                self.as_web_sys_document_fragment()
                    .prepend_with_node_1(document_fragment.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }
        }

        impl AsRef<web_sys::DocumentFragment> for $tpe {
            fn as_ref(&self) -> &web_sys::DocumentFragment {
                self.as_web_sys_document_fragment()
            }
        }

        impl_node_traits!($tpe);
        impl_try_from_node!($tpe, $web_sys_tpe);
    };
}

pub(crate) use impl_document_fragment_traits;

pub struct GenericDocumentFragment {
    inner: web_sys::DocumentFragment,
}

impl From<web_sys::DocumentFragment> for GenericDocumentFragment {
    fn from(inner: web_sys::DocumentFragment) -> Self {
        GenericDocumentFragment { inner }
    }
}

impl_document_fragment_traits!(GenericDocumentFragment);
