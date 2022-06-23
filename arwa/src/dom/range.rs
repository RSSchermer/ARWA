use wasm_bindgen::{throw_val, JsCast, UnwrapThrowExt};

use crate::dom::{DocumentFragment, DynamicNode, HierarchyRequestError};
use crate::{impl_common_wrapper_traits, impl_js_cast};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RangeBoundaryCompare {
    Before,
    Same,
    After,
}

impl RangeBoundaryCompare {
    fn from_int(int: i16) -> Self {
        match int {
            -1 => RangeBoundaryCompare::Before,
            0 => RangeBoundaryCompare::Same,
            1 => RangeBoundaryCompare::After,
            _ => panic!("Invalid boundary value."),
        }
    }
}

mod range_seal {
    pub trait Seal {
        // TODO: get abstract range in web_sys
        // #[doc(hidden)]
        // fn as_web_sys_abstract_range(&self) -> &web_sys::AbstractRange;
    }
}

pub trait Range {
    fn start_container(&self) -> DynamicNode;

    fn start_offset(&self) -> u32;

    fn end_container(&self) -> DynamicNode;

    fn end_offset(&self) -> u32;

    fn is_collapsed(&self) -> bool;
}

#[derive(Clone)]
pub struct StaticRange {
    // TODO: StaticRange in web_sys
    inner: js_sys::Object,
}

impl range_seal::Seal for StaticRange {}

impl Range for StaticRange {
    fn start_container(&self) -> DynamicNode {
        todo!("StaticRange missing in web_sys")
    }

    fn start_offset(&self) -> u32 {
        todo!("StaticRange missing in web_sys")
    }

    fn end_container(&self) -> DynamicNode {
        todo!("StaticRange missing in web_sys")
    }

    fn end_offset(&self) -> u32 {
        todo!("StaticRange missing in web_sys")
    }

    fn is_collapsed(&self) -> bool {
        todo!("StaticRange missing in web_sys")
    }
}

pub(crate) mod range_bound_container_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_node(&self) -> &web_sys::Node;
    }
}

pub trait RangeBoundContainer: range_bound_container_seal::Seal {}

#[derive(Clone)]
pub struct LiveRange {
    inner: web_sys::Range,
}

impl LiveRange {
    // Ignore `new` constructor for now, not sure how that would function of the main thread. Prefer
    // document.create_range, which can't error.

    // Note: select_node_contents is defined as a mutating operation on a pre-existing range, but I
    // feel it makes far more sense as a constructor, especially in Rust where multiple constructors
    // are common. Note that because this takes a node, this can only ever be called on main thread.

    pub fn select_node_contents<T>(&self, container: T) -> Self
    where
        T: RangeBoundContainer,
    {
        let inner = web_sys::Range::new().unwrap_throw();

        inner
            .select_node_contents(container.as_web_sys_node())
            .unwrap_throw();

        LiveRange { inner }
    }

    // TODO: the spec doesn't actually state what happens when these belong to different
    // document(fragment)s, should test browser behavior at some point, because I would expect
    // either errors, or quietly aborting the update.

    pub fn set_start<T>(&self, container: T, offset: u32)
    where
        T: RangeBoundContainer,
    {
        self.inner
            .set_start(container.as_web_sys_node(), offset)
            .unwrap_throw()
    }

    pub fn set_end<T>(&self, container: T, offset: u32)
    where
        T: RangeBoundContainer,
    {
        self.inner
            .set_end(container.as_web_sys_node(), offset)
            .unwrap_throw()
    }

    pub fn collapse_to_start(&self) {
        self.inner.collapse_with_to_start(true)
    }

    pub fn collapse_to_end(&self) {
        self.inner.collapse_with_to_start(false)
    }

    pub fn delete_contents(&self) {
        // No indication in the spec that this can fail.
        self.inner.delete_contents().unwrap_throw()
    }

    pub fn duplicate_contents(&self) -> DocumentFragment {
        match self.inner.clone_contents() {
            Ok(fragment) => fragment.into(),
            Err(err) => throw_val(err),
        }
    }

    pub fn try_duplicate_contents(&self) -> Result<DocumentFragment, HierarchyRequestError> {
        self.inner
            .clone_contents()
            .map(|ok| ok.into())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    pub fn extract_contents(&self) -> DocumentFragment {
        match self.inner.extract_contents() {
            Ok(fragment) => fragment.into(),
            Err(err) => throw_val(err),
        }
    }

    pub fn try_extract_contents(&self) -> Result<DocumentFragment, HierarchyRequestError> {
        self.inner
            .extract_contents()
            .map(|ok| ok.into())
            .map_err(|err| HierarchyRequestError::new(err.unchecked_into()))
    }

    pub fn compare_start_to_start(&self, other: &LiveRange) -> RangeBoundaryCompare {
        let compare = self
            .inner
            .compare_boundary_points(0, other.as_ref())
            .unwrap_throw();

        RangeBoundaryCompare::from_int(compare)
    }

    pub fn compare_start_to_end(&self, other: &LiveRange) -> RangeBoundaryCompare {
        let compare = self
            .inner
            .compare_boundary_points(1, other.as_ref())
            .unwrap_throw();

        RangeBoundaryCompare::from_int(compare)
    }

    pub fn compare_end_to_start(&self, other: &LiveRange) -> RangeBoundaryCompare {
        let compare = self
            .inner
            .compare_boundary_points(3, other.as_ref())
            .unwrap_throw();

        RangeBoundaryCompare::from_int(compare)
    }

    pub fn compare_end_to_end(&self, other: &LiveRange) -> RangeBoundaryCompare {
        let compare = self
            .inner
            .compare_boundary_points(2, other.as_ref())
            .unwrap_throw();

        RangeBoundaryCompare::from_int(compare)
    }

    // TODO: set_start/end_before/after, select_node, do we add those? They seem convenience methods
    // with the drawback of triggering errors on parentless nodes.

    // Ignore surround_contents for now. It's a method with complicated error semantics and it's
    // behaviour be reproduced as a multistep process that first uses `try_extract_contents` and
    // then inserts the document fragment into the intended parent node.

    // Ignore insert_node for now. It has complex error mechanics re. the splittabillity of the
    // start_container node. While convenient, I think it's more clear to force the user to resolve
    // the start_container node and split it manually, then insert the new node.
}

impl Range for LiveRange {
    fn start_container(&self) -> DynamicNode {
        // No indication in the spec that this can fail, and some testing indicates that this will
        // always return a node, even if one e.g. disconnects the current start container node (it
        // will change the startContainer to its parent node it seems)
        self.inner.start_container().unwrap_throw().into()
    }

    fn start_offset(&self) -> u32 {
        self.inner.start_offset().unwrap_throw()
    }

    fn end_container(&self) -> DynamicNode {
        // No indication in the spec that this can fail, see start_container
        self.inner.end_container().unwrap_throw().into()
    }

    fn end_offset(&self) -> u32 {
        self.inner.end_offset().unwrap_throw()
    }

    fn is_collapsed(&self) -> bool {
        self.inner.collapsed()
    }
}

impl From<web_sys::Range> for LiveRange {
    fn from(inner: web_sys::Range) -> Self {
        LiveRange { inner }
    }
}

impl AsRef<web_sys::Range> for LiveRange {
    fn as_ref(&self) -> &web_sys::Range {
        &self.inner
    }
}

impl ToString for LiveRange {
    fn to_string(&self) -> String {
        js_sys::Object::to_string(self.as_ref()).into()
    }
}

impl_common_wrapper_traits!(LiveRange);
impl_js_cast!(LiveRange, Range);
