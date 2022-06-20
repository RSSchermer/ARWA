use crate::dom::impl_owned_node;

pub(crate) mod fragment_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_document_fragment(&self) -> &web_sys::DocumentFragment;
    }
}

/// Implemented by fragment node types.
pub trait Fragment: fragment_seal::Seal {}

macro_rules! impl_fragment_traits {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl $crate::dom::fragment_seal::Seal for $tpe {
            fn as_web_sys_document_fragment(&self) -> &web_sys::DocumentFragment {
                &self.inner
            }
        }

        impl $crate::dom::Fragment for $tpe {}

        $crate::dom::impl_node_traits!($tpe);
        $crate::dom::impl_try_from_node!($tpe, $web_sys_tpe);
        $crate::dom::impl_parent_node!($tpe);
        $crate::dom::impl_try_from_parent_node!($tpe, $web_sys_tpe);
    };
    ($tpe:ident) => {
        $crate::dom::impl_fragment_traits!($tpe, $tpe);
    };
}

pub(crate) use impl_fragment_traits;

/// Acts as the root of a DOM tree without a parent.
///
/// Document fragments are an efficient way to add multiple nodes to a document. Note that a
/// document fragment node cannot itself be directly connected to a document tree (it is a node
/// without a parent). Instead DOM modifying operations that take a document fragment will move
/// each of the document fragment's child nodes into the document tree. Such operations leave the
/// document fragment empty. If your intent is to reuse a document fragment multiple times, consider
/// first duplicating the document fragment with [ParentNode::duplicate_deep] and then passing the
/// duplicate to the DOM modifying operation; this leaves the original document fragment intact.
///
/// The following DOM modifying operations take a document fragment:
///
/// - [ParentNode::prepend_fragment] and [ParentNode::try_prepend_fragment]
/// - [ParentNode::append_fragment] and [ParentNode::try_append_fragment]
/// - [ChildNode::replace_with_fragment] and [ChildNode::try_replace_with_fragment]
/// - [ChildNode::before_insert_fragment] and [ChildNode::try_before_insert_fragment]
/// - [ChildNode::after_insert_fragment] and [ChildNode::try_after_insert_fragment]
///
pub struct DocumentFragment {
    inner: web_sys::DocumentFragment,
}

impl From<web_sys::DocumentFragment> for DocumentFragment {
    fn from(inner: web_sys::DocumentFragment) -> Self {
        DocumentFragment { inner }
    }
}

impl_fragment_traits!(DocumentFragment, DocumentFragment);
impl_owned_node!(DocumentFragment);
