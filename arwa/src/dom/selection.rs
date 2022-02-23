use delegate::delegate;
use wasm_bindgen::UnwrapThrowExt;

use crate::dom::{DynamicNode, LiveRange};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectionType {
    None,
    Caret,
    Range,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectionDirection {
    Forward,
    Backward,
    None,
}

impl Default for SelectionDirection {
    fn default() -> Self {
        SelectionDirection::None
    }
}

#[derive(Clone)]
pub struct Selection {
    inner: web_sys::Selection,
}

impl Selection {
    // Note that although the JS interface for Selection makes it seem like a selection can consist
    // or multiple ranges, this seems to be a historical artifact. Selection is currently specced to
    // be 0 or 1 ranges (although it seems Firefox still support multiple ranges in some
    // instances?). That means this API can be greatly simplified and leverage the LiveRange API for
    // manipulating the selection.

    delegate! {
        target self.inner {
            pub fn anchor_offset(&self) -> u32;

            pub fn focus_offset(&self) -> u32;
        }
    }

    pub fn anchor_node(&self) -> Option<DynamicNode> {
        self.inner.anchor_node().map(|n| n.into())
    }

    pub fn focus_node(&self) -> Option<DynamicNode> {
        self.inner.focus_node().map(|n| n.into())
    }

    pub fn selection_type(&self) -> SelectionType {
        match self.inner.type_().as_ref() {
            "None" => SelectionType::None,
            "Caret" => SelectionType::Caret,
            "Range" => SelectionType::Range,
            _ => unreachable!(),
        }
    }

    pub fn range(&self) -> Option<LiveRange> {
        self.inner.get_range_at(0).ok().map(|r| r.into())
    }

    pub fn set_range(&self, range: Option<LiveRange>) {
        self.inner.empty().unwrap_throw();

        if let Some(range) = range {
            self.inner.add_range(range.as_ref()).unwrap_throw();
        }
    }

    // TODO: currently there's no way to define a backwards range through this API. All selection
    // range modification is currently done with `set_range` or by modifying the current live range
    // (if present) through `range`. Rather than somewhat duplicating this behavior by adding a
    // version of `setBaseAndExtent`, I feel perhaps methods like `direct_backwards` and
    // `direct_forwards` (internally implemented using setBaseAndExtend using the current Range's
    // boundaries, which should never fail). However, is setting the direction of a programmatically
    // created selection even relevant? Anchor node/focus node information may only be useful for
    // browser/user created selections. Holding off on this until a use-case presents itself.
}
