use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::OnSlotChange;
use crate::html::{GenericHtmlElement, HtmlElement};
use crate::{Element, GenericElement, GenericNode, GlobalEventHandlers, InvalidCast, Node};

#[derive(Clone)]
pub struct HtmlSlotElement {
    inner: web_sys::HtmlSlotElement,
}

impl HtmlSlotElement {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn set_name(&self, name: &str);
        }
    }

    // TODO: web_sys is missing `assigned_elements` at this time.

    pub fn assigned_nodes(&self) -> SlotAssignedNodes {
        SlotAssignedNodes {
            inner: self.inner.assigned_nodes(),
        }
    }

    pub fn assigned_nodes_flattened(&self) -> SlotAssignedNodes {
        let mut options = web_sys::AssignedNodesOptions::new();

        options.flatten(true);

        SlotAssignedNodes {
            inner: self.inner.assigned_nodes_with_options(&options),
        }
    }

    pub fn on_slot_change(&self) -> OnSlotChange {
        OnSlotChange::new(self.inner.clone().into())
    }
}

impl_html_common_traits!(HtmlSlotElement);

pub struct SlotAssignedNodes {
    inner: js_sys::Array,
}

impl SlotAssignedNodes {
    pub fn get(&self, index: usize) -> Option<GenericNode> {
        u32::try_from(index).ok().and_then(|index| {
            let value = self.inner.get(index);

            if value.is_undefined() {
                None
            } else {
                let value: web_sys::Node = value.unchecked_into();

                Some(value.into())
            }
        })
    }

    pub fn len(&self) -> usize {
        self.inner.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }

    pub fn first(&self) -> Option<GenericNode> {
        self.get(0)
    }

    pub fn last(&self) -> Option<GenericNode> {
        let len = self.len();

        if len > 0 {
            self.get(len - 1)
        } else {
            None
        }
    }

    pub fn iter(&self) -> SlotAssignedNodesIter {
        SlotAssignedNodesIter {
            slot_assigned_nodes: self,
            current: 0,
        }
    }
}

impl IntoIterator for SlotAssignedNodes {
    type Item = GenericNode;
    type IntoIter = SlotAssignedNodesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        SlotAssignedNodesIntoIter {
            slot_assigned_nodes: self,
            current: 0,
        }
    }
}

pub struct SlotAssignedNodesIter<'a> {
    slot_assigned_nodes: &'a SlotAssignedNodes,
    current: usize,
}

impl<'a> Iterator for SlotAssignedNodesIter<'a> {
    type Item = GenericNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.slot_assigned_nodes.get(current)
    }
}

pub struct SlotAssignedNodesIntoIter {
    slot_assigned_nodes: SlotAssignedNodes,
    current: usize,
}

impl Iterator for SlotAssignedNodesIntoIter {
    type Item = GenericNode;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        self.current += 1;

        self.slot_assigned_nodes.get(current)
    }
}
