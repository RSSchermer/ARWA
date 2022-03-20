use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::collection::{Collection, Sequence};
use crate::dom::{
    ChildNode, DocumentFragment, DynamicElement, DynamicNode, HierarchyRequestError, Selector,
};

pub(crate) mod parent_node_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn from_web_sys_node_unchecked(node: web_sys::Node) -> Self;

        #[doc(hidden)]
        fn as_web_sys_node(&self) -> &web_sys::Node;
    }
}

pub trait ParentNode: parent_node_seal::Seal + Sized {
    fn contains<T>(&self, node: &T) -> bool
    where
        T: ChildNode,
    {
        self.as_web_sys_node()
            .contains(Some(node.as_web_sys_node()))
    }

    fn query_selector_first(&self, selector: &Selector) -> Option<DynamicElement>;

    fn query_selector_all(&self, selector: &Selector) -> QuerySelectorAll;

    /// Creates a new node that is a deep copy of the `source` node.
    ///
    /// Recursively copies the source node's entire sub-tree.
    ///
    /// See also [duplicate].
    fn duplicate_deep(source: &Self) -> Self {
        let cloned_node = source
            .as_web_sys_node()
            .clone_node_with_deep(true)
            .unwrap_throw();

        Self::from_web_sys_node_unchecked(cloned_node)
    }

    fn normalize(&self) {
        self.as_web_sys_node().normalize()
    }

    /// Returns a live collection of the all nodes that are direct children of this parent node.
    ///
    /// See also [child_elements] to retrieve only the subset of nodes that implement [Element].
    fn child_nodes(&self) -> ChildNodes {
        ChildNodes::new(self.as_web_sys_node().child_nodes())
    }

    /// Returns a live collection of only the [Element nodes that are direct children of this parent node.
    ///
    /// See also [child_elements] to retrieve only the subset of nodes that implement [Element].
    fn child_elements(&self) -> ChildElements;

    fn prepend_child<T>(&self, node: &T)
    where
        T: ChildNode;

    fn try_prepend_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode;

    /// Prepends the document structure in the given `document_fragment` before the first child
    /// element in this node.
    ///
    /// This moves the nodes from the document fragment, leaving the document fragment empty.
    fn prepend_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment;

    fn append_child<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        self.as_web_sys_node()
            .append_child(node.as_web_sys_node())
            .unwrap_throw();
    }

    fn try_append_child<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.as_web_sys_node()
            .append_child(node.as_web_sys_node())
            .map(|_| ())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Append the document structure in the given `document_fragment` after the last child element
    /// in this node.
    ///
    /// This moves the nodes from the document fragment, leaving the document fragment empty.
    fn append_fragment<T>(&self, document_fragment: &T)
    where
        T: DocumentFragment,
    {
        // Note: this should never cause a hierarchy request error.
        self.as_web_sys_node()
            .append_child(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw();
    }
}

/// A live collection of all nodes that are direct children of the parent node.
///
/// Obtained by calling [child_nodes] on a [ParentNode] type.
pub struct ChildNodes {
    inner: web_sys::NodeList,
}

impl ChildNodes {
    pub(crate) fn new(inner: web_sys::NodeList) -> Self {
        ChildNodes { inner }
    }
}

impl Collection for ChildNodes {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for ChildNodes {
    type Item = DynamicNode;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.get(index).map(|n| n.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

/// A live collection of all nodes that are direct children of the parent node.
///
/// Obtained by calling [child_nodes] on a [ParentNode] type.
pub struct ChildElements {
    inner: web_sys::HtmlCollection,
}

impl ChildElements {
    pub(crate) fn new(inner: web_sys::HtmlCollection) -> Self {
        ChildElements { inner }
    }
}

impl Collection for ChildElements {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for ChildElements {
    type Item = DynamicElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner.item(index).map(|n| n.into())
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

pub struct QuerySelectorAll {
    inner: web_sys::NodeList,
}

impl QuerySelectorAll {
    pub(crate) fn new(inner: web_sys::NodeList) -> Self {
        QuerySelectorAll { inner }
    }
}

impl Collection for QuerySelectorAll {
    fn len(&self) -> u32 {
        self.inner.length()
    }
}

impl Sequence for QuerySelectorAll {
    type Item = DynamicElement;

    fn get(&self, index: u32) -> Option<Self::Item> {
        self.inner
            .get(index)
            .map(|n| DynamicElement::from(n.unchecked_into::<web_sys::Element>()))
    }

    fn to_host_array(&self) -> js_sys::Array {
        js_sys::Array::from(self.inner.as_ref())
    }
}

macro_rules! impl_parent_node_for_element {
    ($element:ident) => {
        impl $crate::dom::parent_node_seal::Seal for $element {
            fn from_web_sys_node_unchecked(node: web_sys::Node) -> Self {
                use wasm_bindgen::JsCast;

                $element {
                    inner: node.unchecked_into(),
                }
            }

            fn as_web_sys_node(&self) -> &web_sys::Node {
                use crate::dom::element_seal::Seal;

                self.as_web_sys_element().as_ref()
            }
        }

        impl $crate::dom::ParentNode for $element {
            fn query_selector_first(
                &self,
                selector: &$crate::dom::Selector,
            ) -> Option<$crate::dom::DynamicElement> {
                use crate::dom::element_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                self.as_web_sys_element()
                    .query_selector(selector.as_ref())
                    .unwrap_throw()
                    .map(|e| e.into())
            }

            fn query_selector_all(
                &self,
                selector: &$crate::dom::Selector,
            ) -> $crate::dom::QuerySelectorAll {
                use crate::dom::element_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                $crate::dom::QuerySelectorAll::new(
                    self.as_web_sys_element()
                        .query_selector_all(selector.as_ref())
                        .unwrap_throw(),
                )
            }

            fn child_elements(&self) -> $crate::dom::ChildElements {
                use crate::dom::element_seal::Seal;

                $crate::dom::ChildElements::new(self.as_web_sys_element().children())
            }

            fn prepend_child<T>(&self, node: &T)
            where
                T: $crate::dom::ChildNode,
            {
                use crate::dom::element_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                self.as_web_sys_element()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_prepend_child<T>(
                &self,
                node: &T,
            ) -> Result<(), $crate::dom::HierarchyRequestError>
            where
                T: $crate::dom::ChildNode,
            {
                use crate::dom::element_seal::Seal;
                use wasm_bindgen::JsCast;

                self.as_web_sys_element()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .map_err(|err| $crate::dom::HierarchyRequestError::new(err.unchecked_into()))
            }

            fn prepend_fragment<T>(&self, document_fragment: &T)
            where
                T: $crate::dom::DocumentFragment,
            {
                use crate::dom::element_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                self.as_web_sys_element()
                    .prepend_with_node_1(document_fragment.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }
        }
    };
}

pub(crate) use impl_parent_node_for_element;

macro_rules! impl_parent_node_for_document {
    ($document:ident) => {
        impl $crate::dom::parent_node_seal::Seal for $document {
            fn from_web_sys_node_unchecked(node: web_sys::Node) -> Self {
                $document {
                    inner: node.unchecked_into(),
                }
            }

            fn as_web_sys_node(&self) -> &web_sys::Node {
                use crate::dom::document_seal::Seal;

                self.as_web_sys_document().as_ref()
            }
        }

        impl $crate::dom::ParentNode for $document {
            fn query_selector_first(
                &self,
                selector: &$crate::dom::Selector,
            ) -> Option<$crate::dom::DynamicElement> {
                use crate::dom::document_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                self.as_web_sys_document()
                    .query_selector(selector.as_ref())
                    .unwrap_throw()
                    .map(|e| e.into())
            }

            fn query_selector_all(
                &self,
                selector: &$crate::dom::Selector,
            ) -> $crate::dom::QuerySelectorAll {
                use crate::dom::document_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                $crate::dom::QuerySelectorAll::new(
                    self.as_web_sys_document()
                        .query_selector_all(selector.as_ref())
                        .unwrap_throw(),
                )
            }

            fn child_elements(&self) -> $crate::dom::ChildElements {
                use crate::dom::document_seal::Seal;

                $crate::dom::ChildElements::new(self.as_web_sys_document().children())
            }

            fn prepend_child<T>(&self, node: &T)
            where
                T: $crate::dom::ChildNode,
            {
                use crate::dom::document_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                self.as_web_sys_document()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .unwrap_throw();
            }

            fn try_prepend_child<T>(
                &self,
                node: &T,
            ) -> Result<(), $crate::dom::HierarchyRequestError>
            where
                T: $crate::dom::ChildNode,
            {
                use crate::dom::document_seal::Seal;

                self.as_web_sys_document()
                    .prepend_with_node_1(node.as_web_sys_node())
                    .map_err(|err| $crate::dom::HierarchyRequestError::new(err.unchecked_into()))
            }

            fn prepend_fragment<T>(&self, document_fragment: &T)
            where
                T: $crate::dom::DocumentFragment,
            {
                use crate::dom::document_seal::Seal;
                use wasm_bindgen::UnwrapThrowExt;

                self.as_web_sys_document()
                    .prepend_with_node_1(document_fragment.as_web_sys_document_fragment().as_ref())
                    .unwrap_throw();
            }
        }
    };
}

pub(crate) use impl_parent_node_for_document;
