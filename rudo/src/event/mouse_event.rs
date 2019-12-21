use bitflags::bitflags;
use wasm_bindgen::JsCast;

use crate::event::{Event, FromEvent, GenericEventTarget, UiEvent};

bitflags! {
    pub struct MouseButtons: u16 {
        const PRIMARY = 0b00000001;
        const SECONDARY = 0b00000010;
        const AUXILIARY = 0b00000100;
        const FOURTH = 0b00001000;
        const FIFTH = 0b000010000;
    }
}

pub trait MouseEvent: AsRef<web_sys::MouseEvent> {
    // TODO: enum for modifier states?
    fn get_modifier_state(&self, key: &str) -> bool {
        self.as_ref().get_modifier_state(key)
    }

    fn screen_x(&self) -> i32 {
        self.as_ref().screen_x()
    }

    fn screen_y(&self) -> i32 {
        self.as_ref().screen_y()
    }

    fn client_x(&self) -> i32 {
        self.as_ref().client_x()
    }

    fn client_y(&self) -> i32 {
        self.as_ref().client_y()
    }

    fn ctrl_key(&self) -> bool {
        self.as_ref().ctrl_key()
    }

    fn shift_key(&self) -> bool {
        self.as_ref().shift_key()
    }

    fn alt_key(&self) -> bool {
        self.as_ref().alt_key()
    }

    fn meta_key(&self) -> bool {
        self.as_ref().meta_key()
    }

    fn region(&self) -> Option<String> {
        self.as_ref().region()
    }

    fn movement_x(&self) -> i32 {
        self.as_ref().movement_x()
    }

    fn movement_y(&self) -> i32 {
        self.as_ref().movement_y()
    }

    fn buttons(&self) -> MouseButtons {
        MouseButtons::from_bits_truncate(self.as_ref().buttons())
    }

    // TODO: `button`. Is this perhaps not the correct  abstraction? Should there be a MouseEvent
    // trait and a MouseButtonEvent trait and separate types for each indivual mouse event (e.g.
    // `MouseUp`, `MouseOver`, etc.)?

    fn related_target(&self) -> Option<GenericEventTarget> {
        self.as_ref()
            .related_target()
            .map(|related_target| GenericEventTarget::from(related_target))
    }
}

pub struct GenericMouseEvent {
    inner: web_sys::MouseEvent,
}

impl_common_ui_event_traits!(GenericMouseEvent, MouseEvent);

impl MouseEvent for GenericMouseEvent {}
