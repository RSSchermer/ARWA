use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent, GenericEventTarget, UiEvent};

pub struct FocusEvent {
    inner: web_sys::FocusEvent,
}

impl FocusEvent {
    pub fn related_target(&self) -> Option<GenericEventTarget> {
        self.inner
            .related_target()
            .map(|related_target| GenericEventTarget::from(related_target))
    }
}

impl_common_ui_event_traits!(FocusEvent);
