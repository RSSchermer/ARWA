use std::marker;

use delegate::delegate;

use crate::ui::{impl_mouse_event_traits, impl_ui_event_traits};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeltaMode {
    Pixel,
    Line,
    Page,
}

#[derive(Clone)]
pub struct WheelEvent<T> {
    inner: web_sys::WheelEvent,
    _marker: marker::PhantomData<T>,
}

impl<T> WheelEvent<T> {
    delegate! {
        to self.inner {
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

impl<T> AsRef<web_sys::WheelEvent> for WheelEvent<T> {
    fn as_ref(&self) -> &web_sys::WheelEvent {
        &self.inner
    }
}

impl_ui_event_traits!(WheelEvent, WheelEvent, "wheel");
impl_mouse_event_traits!(WheelEvent);
