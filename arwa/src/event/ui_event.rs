use wasm_bindgen::JsCast;

use crate::event::on_event::FromEvent;
use crate::event::Event;

pub trait UiEvent: AsRef<web_sys::UiEvent> {
    // TODO: view

    // TODO: `detail` on the specific event types for which it is actually meaningful
}

pub struct GenericUiEvent {
    inner: web_sys::UiEvent,
}

impl_common_event_traits!(GenericUiEvent, UiEvent);

impl UiEvent for GenericUiEvent {}
