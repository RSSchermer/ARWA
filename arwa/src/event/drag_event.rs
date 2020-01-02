use wasm_bindgen::JsCast;

use crate::event::on_event::FromEvent;
use crate::event::{Event, MouseEvent, UiEvent};

pub struct DragEvent {
    inner: web_sys::DragEvent,
}

impl DragEvent {
    // TODO: data_transfer
}

impl_common_mouse_event_traits!(DragEvent);
