use std::convert::TryFrom;

use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::event::{GenericEventTarget, OnSlotChange};
use crate::html::{DynamicHtmlElement, HtmlElement};
use crate::{DynamicElement, DynamicNode, Element, GlobalEventHandlers, InvalidCast, Node};
use std::marker;

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

unchecked_cast_array!(DynamicNode, web_sys::Node, SlotAssignedNodes);

pub(crate) mod slot_change_event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait SlotChangeEventTarget: slot_change_event_target_seal::Seal {
    fn on_slot_change(&self) -> OnSlotChange<Self> {
        OnSlotChange::new(self.as_web_sys_event_target().clone().into())
    }
}

pub struct SlotChangeEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl_event_traits!(SlotChangeEvent, web_sys::Event);

typed_event_stream!(
    OnSlotChange,
    OnSlotChangeWithOptions,
    SlotChangeEvent,
    "slotchange"
);
