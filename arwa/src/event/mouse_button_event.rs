use wasm_bindgen::JsCast;

use crate::event::on_event::FromEvent;
use crate::event::{Event, MouseEvent, UiEvent};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum MouseButton {
    Primary = 0,
    Auxiliary = 1,
    Secondary = 2,
    Fourth = 3,
    Fifth = 4,
}

pub struct MouseButtonEvent {
    inner: web_sys::MouseEvent,
}

impl MouseButtonEvent {
    pub fn button(&self) -> MouseButton {
        match self.inner.button() {
            0 => MouseButton::Primary,
            1 => MouseButton::Auxiliary,
            2 => MouseButton::Secondary,
            3 => MouseButton::Fourth,
            4 => MouseButton::Fifth,
            _ => unreachable!(),
        }
    }
}

impl_common_ui_event_traits!(MouseButtonEvent, MouseEvent);

impl MouseEvent for MouseButtonEvent {}
