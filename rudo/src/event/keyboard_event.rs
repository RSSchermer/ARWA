use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::on_event::FromEvent;
use crate::event::{Event, UiEvent};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyLocation {
    Standard = 0,
    Left = 1,
    Right = 2,
    Numpad = 3,
}

pub struct KeyboardEvent {
    inner: web_sys::KeyboardEvent,
}

impl KeyboardEvent {
    delegate! {
        target self.inner {
            pub fn get_modifier_state(&self, key: &str) -> bool;

            pub fn alt_key(&self) -> bool;

            pub fn ctrl_key(&self) -> bool;

            pub fn shift_key(&self) -> bool;

            pub fn meta_key(&self) -> bool;

            pub fn repeat(&self) -> bool;

            pub fn is_composing(&self) -> bool;

            pub fn key(&self) -> String;

            pub fn code(&self) -> String;
        }
    }

    pub fn location(&self) -> KeyLocation {
        match self.inner.location() {
            0 => KeyLocation::Standard,
            1 => KeyLocation::Left,
            2 => KeyLocation::Right,
            3 => KeyLocation::Numpad,
            // Note: there are old browser version that support additional values, but none of these
            // browser version support WASM, so we should never reach those.
            _ => unreachable!(),
        }
    }
}

impl_common_ui_event_traits!(KeyboardEvent);
