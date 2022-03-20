use std::marker;

use delegate::delegate;
use web_sys::Node;

use crate::dom::{impl_try_from_element, DynamicNode};
use crate::event::{impl_typed_event_traits, typed_event_iterator};
use crate::html::{impl_extendable_element, impl_html_element_traits, impl_known_element};
use crate::unchecked_cast_array::unchecked_cast_array;

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
        SlotAssignedNodes::new(self.inner.assigned_nodes())
    }

    pub fn assigned_nodes_flattened(&self) -> SlotAssignedNodes {
        let mut options = web_sys::AssignedNodesOptions::new();

        options.flatten(true);

        SlotAssignedNodes::new(self.inner.assigned_nodes_with_options(&options))
    }
}

impl From<web_sys::HtmlSlotElement> for HtmlSlotElement {
    fn from(inner: web_sys::HtmlSlotElement) -> Self {
        HtmlSlotElement { inner }
    }
}

impl AsRef<web_sys::HtmlSlotElement> for HtmlSlotElement {
    fn as_ref(&self) -> &web_sys::HtmlSlotElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlSlotElement);
impl_try_from_element!(HtmlSlotElement);
impl_known_element!(HtmlSlotElement, "SLOT");
impl_extendable_element!(HtmlSlotElement, "slot");

unchecked_cast_array!(DynamicNode, Node, SlotAssignedNodes);

pub(crate) mod slot_change_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait SlotChangeEventTarget: slot_change_event_target_seal::Seal + Sized {
    fn on_slot_change(&self) -> OnSlotChange<Self> {
        OnSlotChange::new(self.as_web_sys_event_target())
    }
}

pub struct SlotChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_typed_event_traits!(SlotChangeEvent, Event, "slotchange");

typed_event_iterator!(
    OnSlotChange,
    OnSlotChangeWithOptions,
    SlotChangeEvent,
    "slotchange"
);
