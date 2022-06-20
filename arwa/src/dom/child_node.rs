use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::dom::{
    node_seal, owned_node_seal, Fragment, DynamicElement, DynamicNode,
    HierarchyRequestError, Node, OwnedNode,
};
use crate::event::impl_event_target_traits;

pub(crate) mod child_node_seal {
    use crate::dom::JsChildNode;

    pub trait Seal {
        #[doc(hidden)]
        fn as_js_child_node(&self) -> &JsChildNode;
    }
}

/// Implemented for nodes that can belong to a parent node in the document tree hierarchy.
pub trait ChildNode: child_node_seal::Seal {
    /// Returns the root node of the (shadow) document tree this child node belongs to.
    ///
    /// If this node is part of a shadow document fragment, this will return the [ShadowRoot]. See
    /// [root_node_composed] for an alternative that will return a root node beyond the shadow
    /// root (if any).
    ///
    /// May return this node itself if it is not connected to a tree.
    fn root_node(&self) -> DynamicNode {
        self.as_js_child_node().get_root_node().into()
    }

    /// Returns the root node of the document tree this child node belongs to.
    ///
    /// Contrary to [root_node], this may return a node beyond the [ShadowRoot] if this node is
    /// part of a shadow document fragment.
    ///
    /// May return this node itself if it is not connected to a tree.
    fn root_node_composed(&self) -> DynamicNode {
        let mut options = web_sys::GetRootNodeOptions::new();

        options.composed(true);

        self.as_js_child_node()
            .get_root_node_with_options(&options)
            .into()
    }

    /// Returns the node's parent node, or `None` if the node is not connected to a document tree.
    fn parent_node(&self) -> Option<DynamicNode> {
        self.as_js_child_node().parent_node().map(|n| n.into())
    }

    /// Returns the node's parent element, or `None` if the node does not belong to a parent
    /// element.
    ///
    /// Will return `None` if the node is connected to a parent node that is not an element. See
    /// [parent_node] for an alternative that will return any parent node.
    fn parent_element(&self) -> Option<DynamicElement> {
        self.as_js_child_node().parent_element().map(|e| e.into())
    }

    /// Returns the node that precedes this node amongst it parent's children, if any.
    ///
    /// Returns `None` if this node is the first child node of it's parent node. Always returns
    /// `None` if this node is not connected to a parent node.
    fn previous_sibling(&self) -> Option<DynamicChildNode> {
        self.as_js_child_node()
            .previous_sibling()
            .map(|n| DynamicChildNode::new(n))
    }

    /// Returns the node that succeeds this node amongst it parent's children, if any.
    ///
    /// Returns `None` if this node is the last child node of it's parent node. Always returns
    /// `None` if this node is not connected to a parent node.
    fn next_sibling(&self) -> Option<DynamicChildNode> {
        self.as_js_child_node()
            .next_sibling()
            .map(|n| DynamicChildNode::new(n))
    }

    /// Returns `true` if this node is connected to a parent node, `false` otherwise.
    fn is_connected(&self) -> bool {
        self.as_js_child_node().is_connected()
    }

    /// Disconnects this node from it's parent node.
    ///
    /// If the node is connected to a parent node, this will remove it from amongst the parent's
    /// children and will leave this node parent-less. Does nothing if this node is not connected
    /// to a parent node.
    fn disconnect(&self) {
        self.as_js_child_node().remove();
    }

    /// Disconnects this node from it's parent node and replaces it with the given `replacement`
    /// node.
    ///
    /// Will leave this node disconnected (see [disconnect]). If the `replacement` node is already
    /// connected to a parent node, this will first disconnect the `replacement` node from that
    /// parent node.
    ///
    /// # Panics
    ///
    /// Panics if this operation would result in an invalid DOM hierarchy (see
    /// [HierarchyRequestError] for details on invalid DOM hierarchies). See [try_replace_with]
    /// for an alternative that returns an error instead of panicking.
    fn replace_with<T>(&self, replacement: &T)
    where
        T: ChildNode,
    {
        self.as_js_child_node()
            .replace_with(replacement.as_js_child_node().as_ref())
            .unwrap_throw();
    }

    /// Disconnects this node from it's parent node and replaces it with the given `replacement`
    /// node, or returns a [HierarchyRequestError] if this operation would result in an invalid
    /// document hierarchy.
    ///
    /// Will leave this node disconnected (see [disconnect]). If the `replacement` node is already
    /// connected to a parent node, this will first disconnect the `replacement` node from that
    /// parent node.
    ///
    /// Returns a [HierarchyRequestError] if this operation would result in an invalid DOM
    /// hierarchy (see [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [replace_with] for an alternative that panics instead of returning an error.
    fn try_replace_with<T>(&self, replacement: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.as_js_child_node()
            .replace_with(replacement.as_js_child_node().as_ref())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Disconnects this node from it's parent node and replaces it with the nodes in the given
    /// `document_fragment`.
    ///
    /// Will leave this node disconnected (see [disconnect]).
    ///
    /// This will remove all nodes from the `document_fragment`, leaving the `document_fragment`
    /// "empty".
    ///
    /// # Panics
    ///
    /// Panics if this operation would result in an invalid DOM hierarchy (see
    /// [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [try_replace_with_fragment] for an alternative that returns an error instead of panicking.
    fn replace_with_fragment<T>(&self, document_fragment: &T)
        where
            T: Fragment,
    {
        self.as_js_child_node()
            .replace_with(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw();
    }

    /// Disconnects this node from it's parent node and replaces it with the nodes in the given
    /// `document_fragment`, or returns a [HierarchyRequestError] if this operation would result in
    /// an invalid document hierarchy.
    ///
    /// Will leave this node disconnected (see [disconnect]).
    ///
    /// This will remove all nodes from the `document_fragment`, leaving the `document_fragment`
    /// "empty".
    ///
    /// Returns a [HierarchyRequestError] if this operation would result in an invalid DOM
    /// hierarchy (see [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [replace_with_fragment] for an alternative that panics instead of returning an error.
    fn try_replace_with_fragment<T>(&self, document_fragment: &T) -> Result<(), HierarchyRequestError>
        where
            T: Fragment,
    {
        self.as_js_child_node()
            .replace_with(document_fragment.as_web_sys_document_fragment().as_ref())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Inserts the given `node` in the list of children of this node's parent node, just before
    /// this node.
    ///
    /// If the `node` is already connected to a parent node, this will first disconnect the `node`.
    /// Does nothing if this node is not connected to a parent node.
    ///
    /// # Panics
    ///
    /// Panics if this operation would result in an invalid DOM hierarchy (see
    /// [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [try_before_insert_node] for an alternative that returns an error instead of panicking.
    fn before_insert_node<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        self.as_js_child_node()
            .before(node.as_js_child_node().as_ref())
            .unwrap_throw();
    }

    /// Inserts the given `node` in the list of children of this node's parent node, just before
    /// this node, or returns a [HierarchyRequestError] if this operation would result in an invalid
    /// document hierarchy.
    ///
    /// If the `node` is already connected to a parent node, this will first disconnect the `node`.
    /// Does nothing if this node is not connected to a parent node.
    ///
    /// Returns a [HierarchyRequestError] if this operation would result in an invalid DOM
    /// hierarchy (see [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [before_insert_node] for an alternative that panics instead of returning an error.
    fn try_before_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.as_js_child_node()
            .before(node.as_js_child_node().as_ref())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Inserts the nodes in the given `document_fragment` in the list of children of this node's
    /// parent node, just before this node.
    ///
    /// This will remove all nodes from the `document_fragment`, leaving the `document_fragment`
    /// "empty".
    ///
    /// # Panics
    ///
    /// Panics if this operation would result in an invalid DOM hierarchy (see
    /// [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [try_before_insert_fragment] for an alternative that returns an error instead of panicking.
    fn before_insert_fragment<T>(&self, document_fragment: &T)
    where
        T: Fragment,
    {
        self.as_js_child_node()
            .before(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw();
    }

    /// Inserts the nodes in the given `document_fragment` in the list of children of this node's
    /// parent node, just before this node, or returns a [HierarchyRequestError] if this operation
    /// would result in an invalid document hierarchy.
    ///
    /// This will remove all nodes from the `document_fragment`, leaving the `document_fragment`
    /// "empty".
    ///
    /// Returns a [HierarchyRequestError] if this operation would result in an invalid DOM
    /// hierarchy (see [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [before_insert_fragment] for an alternative that panics instead of returning an error.
    fn try_before_insert_fragment<T>(&self, document_fragment: &T) -> Result<(), HierarchyRequestError>
        where
            T: Fragment,
    {
        self.as_js_child_node()
            .before(document_fragment.as_web_sys_document_fragment().as_ref())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Inserts the given `node` in the list of children of this node's parent node, just after this
    /// node.
    ///
    /// If the `node` is already connected to a parent node, this will first disconnect the `node`.
    /// Does nothing if this node is not connected to a parent node.
    ///
    /// # Panics
    ///
    /// Panics if this operation would result in an invalid DOM hierarchy (see
    /// [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [try_after_insert_node] for an alternative that returns an error instead of panicking.
    fn after_insert_node<T>(&self, node: &T)
    where
        T: ChildNode,
    {
        self.as_js_child_node()
            .after(node.as_js_child_node().as_ref())
            .unwrap_throw();
    }

    /// Inserts the given `node` in the list of children of this node's parent node, just after this
    /// node, or returns a [HierarchyRequestError] if this operation would result in an invalid
    /// document hierarchy.
    ///
    /// If the `node` is already connected to a parent node, this will first disconnect the `node`.
    /// Does nothing if this node is not connected to a parent node.
    ///
    /// Returns a [HierarchyRequestError] if this operation would result in an invalid DOM
    /// hierarchy (see [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [after_insert_node] for an alternative that panics instead of returning an error.
    fn try_after_insert_node<T>(&self, node: &T) -> Result<(), HierarchyRequestError>
    where
        T: ChildNode,
    {
        self.as_js_child_node()
            .after(node.as_js_child_node().as_ref())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    /// Inserts the nodes in the given `document_fragment` in the list of children of this node's
    /// parent node, just after this node.
    ///
    /// This will remove all nodes from the `document_fragment`, leaving the `document_fragment`
    /// "empty".
    ///
    /// # Panics
    ///
    /// Panics if this operation would result in an invalid DOM hierarchy (see
    /// [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [try_after_insert_fragment] for an alternative that returns an error instead of panicking.
    fn after_insert_fragment<T>(&self, document_fragment: &T)
        where
            T: Fragment,
    {
        self.as_js_child_node()
            .after(document_fragment.as_web_sys_document_fragment().as_ref())
            .unwrap_throw();
    }

    /// Inserts the nodes in the given `document_fragment` in the list of children of this node's
    /// parent node, just after this node, or returns a [HierarchyRequestError] if this operation
    /// would result in an invalid document hierarchy.
    ///
    /// This will remove all nodes from the `document_fragment`, leaving the `document_fragment`
    /// "empty".
    ///
    /// Returns a [HierarchyRequestError] if this operation would result in an invalid DOM
    /// hierarchy (see [HierarchyRequestError] for details on invalid DOM hierarchies). See
    /// [after_insert_fragment] for an alternative that panics instead of returning an error.
    fn try_after_insert_fragment<T>(&self, document_fragment: &T) -> Result<(), HierarchyRequestError>
        where
            T: Fragment,
    {
        self.as_js_child_node()
            .after(document_fragment.as_web_sys_document_fragment().as_ref())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }
}

/// A node that can be used as a [ChildNode], but for which a specific type is not statically known.
///
/// You may try to resolve a value of this type to a concrete type using [TryFrom] conversion. All
/// Arwa types that implement [ChildNode] also implement `TryFrom<DynamicChildNode>`.
#[derive(Clone, PartialEq)]
pub struct DynamicChildNode {
    inner: web_sys::Node,
}

impl DynamicChildNode {
    pub(crate) fn new(inner: web_sys::Node) -> Self {
        DynamicChildNode { inner }
    }
}

impl node_seal::Seal for DynamicChildNode {
    fn from_web_sys_node_unchecked(inner: web_sys::Node) -> Self {
        DynamicChildNode { inner }
    }

    fn as_web_sys_node(&self) -> &web_sys::Node {
        &self.inner
    }
}

impl Node for DynamicChildNode {}

impl owned_node_seal::Seal for DynamicChildNode {
    fn as_web_sys_node(&self) -> &web_sys::Node {
        &self.inner
    }
}

impl OwnedNode for DynamicChildNode {}

impl child_node_seal::Seal for DynamicChildNode {
    fn as_js_child_node(&self) -> &JsChildNode {
        self.inner.unchecked_ref()
    }
}

impl ChildNode for DynamicChildNode {}

impl AsRef<web_sys::Node> for DynamicChildNode {
    fn as_ref(&self) -> &web_sys::Node {
        &self.inner
    }
}

impl From<DynamicChildNode> for DynamicNode {
    fn from(value: DynamicChildNode) -> Self {
        DynamicNode::from(value.inner)
    }
}

impl From<DynamicChildNode> for web_sys::Node {
    fn from(value: DynamicChildNode) -> Self {
        value.inner
    }
}

impl_event_target_traits!(DynamicChildNode);

macro_rules! impl_child_node {
    ($tpe:ident) => {
        impl $crate::dom::child_node_seal::Seal for $tpe {
            fn as_js_child_node(&self) -> &$crate::dom::JsChildNode {
                use wasm_bindgen::JsCast;

                self.inner.unchecked_ref()
            }
        }

        impl $crate::dom::ChildNode for $tpe {}
    };
}

pub(crate) use impl_child_node;

macro_rules! impl_try_from_child_node {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl TryFrom<$crate::dom::DynamicChildNode> for $tpe {
            type Error = $crate::InvalidCast<$crate::dom::DynamicChildNode, $tpe>;

            fn try_from(value: $crate::dom::DynamicChildNode) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::Node = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| $crate::InvalidCast::new($crate::dom::DynamicChildNode::new(e)))
            }
        }
    };
    ($tpe:ident) => {
        $crate::dom::impl_try_from_child_node!($tpe, $tpe);
    };
}

pub(crate) use impl_try_from_child_node;

#[wasm_bindgen]
extern "C" {
    #[doc(hidden)]
    #[wasm_bindgen(extends = web_sys::Node)]
    pub type JsChildNode;

    #[wasm_bindgen(catch, method)]
    pub(crate) fn before(this: &JsChildNode, node: &web_sys::Node) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    pub(crate) fn after(this: &JsChildNode, node: &web_sys::Node) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name=replaceWith)]
    pub(crate) fn replace_with(this: &JsChildNode, node: &web_sys::Node) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    pub(crate) fn remove(this: &JsChildNode);
}
