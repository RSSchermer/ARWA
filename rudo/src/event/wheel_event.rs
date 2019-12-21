use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::on_event::FromEvent;
use crate::event::{Event, MouseEvent, UiEvent};

pub enum DeltaMode {
    Pixel,
    Line,
    Page,
}

pub struct WheelEvent {
    inner: web_sys::WheelEvent,
}

impl WheelEvent {
    delegate! {
        target self.inner {
            pub fn delta_x(&self) -> f64;

            pub fn delta_y(&self) -> f64;

            pub fn delta_z(&self) -> f64;
        }
    }

    pub fn delta_mode(&self) -> DeltaMode {
        match self.inner.delta_mode() {
            0 => DeltaMode::Pixel,
            1 => DeltaMode::Line,
            2 => DeltaMode::Page,
            _ => unreachable!(),
        }
    }
}

impl_common_mouse_event_traits!(WheelEvent);
