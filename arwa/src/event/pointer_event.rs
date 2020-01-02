use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::on_event::FromEvent;
use crate::event::{Event, MouseEvent, UiEvent};
use crate::PointerId;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
}

pub struct PointerEvent {
    inner: web_sys::PointerEvent,
}

impl PointerEvent {
    delegate! {
        target self.inner {
            pub fn width(&self) -> i32;

            pub fn height(&self) -> i32;

            pub fn pressure(&self) -> f32;

            pub fn tangential_pressure(&self) -> f32;

            pub fn tilt_x(&self) -> i32;

            pub fn tilt_y(&self) -> i32;

            pub fn twist(&self) -> i32;

            pub fn is_primary(&self) -> bool;
        }
    }

    pub fn pointer_id(&self) -> PointerId {
        PointerId::new(self.inner.pointer_id())
    }

    pub fn pointer_type(&self) -> PointerType {
        match &*self.inner.pointer_type() {
            "mouse" => PointerType::Mouse,
            "pen" => PointerType::Pen,
            "touch" => PointerType::Touch,
            _ => unreachable!(),
        }
    }
}

impl_common_mouse_event_traits!(PointerEvent);
