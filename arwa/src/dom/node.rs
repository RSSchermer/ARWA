use bitflags::bitflags;
use wasm_bindgen::UnwrapThrowExt;

use crate::dom::NonColonName;
use crate::event::{impl_event_target_traits, impl_try_from_event_target};

bitflags! {
    pub struct CompareDocumentPosition: u16 {
        const DISCONNECTED = 0b00000001;
        const PRECEDING = 0b00000010;
        const FOLLOWING = 0b00000100;
        const CONTAINS = 0b00001000;
        const CONTAINED_BY = 0b000010000;
        const IMPLEMENTATION_SPECIFIC = 0b00100000;
    }
}

pub(crate) mod node_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn from_web_sys_node_unchecked(node: web_sys::Node) -> Self;

        #[doc(hidden)]
        fn as_web_sys_node(&self) -> &web_sys::Node;
    }
}

pub trait Node: node_seal::Seal + Sized {
    /// Creates a new node that is a shallow copy of the `source` node.
    ///
    /// The resulting node will not have any children.
    ///
    /// See also [ParentNode::duplicate_deep].
    fn duplicate(source: &Self) -> Self {
        let cloned_node = source.as_web_sys_node().clone_node().unwrap_throw();

        Self::from_web_sys_node_unchecked(cloned_node)
    }

    // Note: nodeValue only returns something other than null for CharacterData types. CharacterData
    // already provides a `data` property that has the exact same functionality.

    fn base_uri(&self) -> String {
        self.as_web_sys_node()
            .base_uri()
            .ok()
            .flatten()
            .unwrap_or_default()
    }

    fn text_content(&self) -> Option<String> {
        self.as_web_sys_node().text_content()
    }

    fn compare_document_position<T>(&self, other: &T) -> CompareDocumentPosition
    where
        T: Node,
    {
        let pos = self
            .as_web_sys_node()
            .compare_document_position(other.as_web_sys_node());

        CompareDocumentPosition::from_bits_truncate(pos)
    }

    fn is_default_namespace(&self, namespace: &str) -> bool {
        self.as_web_sys_node().is_default_namespace(Some(namespace))
    }

    fn lookup_namespace_uri(&self, prefix: &NonColonName) -> Option<String> {
        self.as_web_sys_node()
            .lookup_namespace_uri(Some(prefix.as_ref()))
    }

    fn lookup_prefix(&self, namespace: &str) -> Option<NonColonName> {
        self.as_web_sys_node()
            .lookup_prefix(Some(namespace))
            .map(|p| NonColonName::trusted(p))
    }

    // TODO:
    // - figure out what to do about is_equal, is_same with regards to PartialEq
}

/// A [Node] for which a specific type is not statically known.
///
/// You may try to resolve a value of this type to a more specific type using [TryFrom]. All Arwa
/// types that implement [Node] also implement `TryFrom<DynamicNode>`.
#[derive(Clone, PartialEq)]
pub struct DynamicNode {
    inner: web_sys::Node,
}

impl node_seal::Seal for DynamicNode {
    fn from_web_sys_node_unchecked(inner: web_sys::Node) -> Self {
        DynamicNode { inner }
    }

    fn as_web_sys_node(&self) -> &web_sys::Node {
        &self.inner
    }
}

impl Node for DynamicNode {}

impl AsRef<web_sys::Node> for DynamicNode {
    fn as_ref(&self) -> &web_sys::Node {
        &self.inner
    }
}

impl From<web_sys::Node> for DynamicNode {
    fn from(inner: web_sys::Node) -> Self {
        DynamicNode { inner }
    }
}

impl From<DynamicNode> for web_sys::Node {
    fn from(value: DynamicNode) -> Self {
        value.inner
    }
}

impl_event_target_traits!(DynamicNode);
impl_try_from_event_target!(DynamicNode, Node);

macro_rules! impl_node_traits {
    ($tpe:ident) => {
        impl $crate::dom::node_seal::Seal for $tpe {
            fn from_web_sys_node_unchecked(node: web_sys::Node) -> Self {
                use wasm_bindgen::JsCast;

                $tpe {
                    inner: node.unchecked_into(),
                }
            }

            fn as_web_sys_node(&self) -> &web_sys::Node {
                self.inner.as_ref()
            }
        }

        impl $crate::dom::Node for $tpe {}

        impl AsRef<web_sys::Node> for $tpe {
            fn as_ref(&self) -> &web_sys::Node {
                use crate::dom::node_seal::Seal;

                self.as_web_sys_node()
            }
        }

        impl From<$tpe> for $crate::dom::DynamicNode {
            fn from(node: $tpe) -> $crate::dom::DynamicNode {
                use wasm_bindgen::JsCast;

                $crate::dom::DynamicNode::from(node.inner.unchecked_into::<web_sys::Node>())
            }
        }

        $crate::event::impl_event_target_traits!($tpe);
    };
}

pub(crate) use impl_node_traits;

macro_rules! impl_try_from_node {
    ($tpe:ident, $web_sys_tpe:ident) => {
        $crate::event::impl_try_from_event_target!($tpe, $web_sys_tpe);

        impl TryFrom<$crate::dom::DynamicNode> for $tpe {
            type Error = $crate::InvalidCast<$crate::dom::DynamicNode, $tpe>;

            fn try_from(value: $crate::dom::DynamicNode) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::Node = value.into();

                value
                    .dyn_into::<$tpe>()
                    .map_err(|e| $crate::InvalidCast::new(e.into()))
            }
        }
    };
    ($tpe:ident) => {
        $crate::dom::impl_try_from_node!($tpe, $tpe);
    };
}

pub(crate) use impl_try_from_node;
