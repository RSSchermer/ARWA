use wasm_bindgen::JsCast;

use crate::dom::{impl_owned_node, HierarchyRequestError, QuerySelectorAll};

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
            fn from_web_sys_node_unchecked(node: web_sys::Node) -> Self {
                $tpe {
                    inner: node.unchecked_into(),
                }
            }

            fn as_web_sys_node(&self) -> &web_sys::Node {
                use crate::dom::document_fragment_seal::Seal;

                self.as_web_sys_document_fragment().as_ref()
            }
        }

        impl $crate::dom::ParentNode for $tpe {
            fn query_selector_first(
                &self,
                selector: &$crate::dom::Selector,
            ) -> Option<$crate::dom::DynamicElement> {
                use crate::dom::document_fragment_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                self.as_web_sys_document_fragment()
                    .query_selector(selector.as_ref())
                    .unwrap_throw()
                    .map(|e| e.into())
            }

            fn query_selector_all(
                &self,
                selector: &$crate::dom::Selector,
            ) -> $crate::dom::QuerySelectorAll {
                use crate::dom::document_fragment_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                QuerySelectorAll::new(
                    self.as_web_sys_document_fragment()
                        .query_selector_all(selector.as_ref())
                        .unwrap_throw(),
                )
            }

            fn child_elements(&self) -> $crate::dom::ChildElements {
                use crate::dom::document_fragment_seal::Seal;

                $crate::dom::ChildElements::new(self.as_web_sys_document_fragment().children())
            }

            fn prepend_child<T>(&self, node: &T)
            where
                T: $crate::dom::ChildNode,
            {
                use crate::dom::document_fragment_seal::Seal;

                if let Err(err) = self
                    .as_web_sys_document_fragment()
                    .prepend_with_node_1(node.as_web_sys_node())
                {
                    wasm_bindgen::throw_val(err)
                }
            }

            fn try_prepend_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
            where
                T: $crate::dom::ChildNode,
            {
                use crate::dom::document_fragment_seal::Seal;

                self.as_web_sys_document_fragment()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .map_err(|err| $crate::dom::HierarchyRequestError::new(err.unchecked_into()))
            }

            fn prepend_fragment<T>(&self, document_fragment: &T)
            where
                T: $crate::dom::DocumentFragment,
            {
                use crate::dom::document_fragment_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                self.as_web_sys_document_fragment()
                    .prepend_with_node_1(document_fragment.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }
        }

        impl AsRef<web_sys::DocumentFragment> for $tpe {
            fn as_ref(&self) -> &web_sys::DocumentFragment {
                use crate::dom::document_fragment_seal::Seal;

                self.as_web_sys_document_fragment()
            }
        }

        $crate::dom::impl_node_traits!($tpe);
        $crate::dom::impl_try_from_node!($tpe, $web_sys_tpe);
    };
    ($tpe:ident) => {
        $crate::dom::impl_document_fragment_traits!($tpe, $tpe);
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

impl_document_fragment_traits!(GenericDocumentFragment, DocumentFragment);
impl_owned_node!(GenericDocumentFragment);
