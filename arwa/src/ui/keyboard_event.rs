use delegate::delegate;
use wasm_bindgen::JsCast;

use crate::event::on_event::FromEvent;
use arwa::event::{Event, UiEvent};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeyLocation {
    Standard = 0,
    Left = 1,
    Right = 2,
    Numpad = 3,
}

mod keyboard_event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_keyboard_event(&self) -> &web_sys::KeyboardEvent;
    }
}

pub trait KeyboardEvent: keyboard_event_seal::Seal {
    fn repeat(&self) -> bool {
        self.as_web_sys_keyboard_event().repeat()
    }

    fn is_composing(&self) -> bool {
        self.as_web_sys_keyboard_event().is_composing()
    }

    fn key(&self) -> String {
        self.as_web_sys_keyboard_event().key()
    }

    fn code(&self) -> String {
        self.as_web_sys_keyboard_event().code()
    }

    fn location(&self) -> KeyLocation {
        match self.as_web_sys_keyboard_event().location() {
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

macro_rules! keyboard_event {
    ($event:ident) => {
        #[derive(Clone)]
        pub struct $event<T> {
            inner: web_sys::KeyboardEvent,
            _marker: std::marker::PhantomData<T>,
        }

        impl keyboard_event_seal::Seal for $event {
            fn as_web_sys_keyboard_event(&self) -> web_sys::KeyboardEvent {
                &self.inner
            }
        }

        impl KeyboardEvent for $event {}

        impl ModifierState for KeyboardEvent {
            delegate! {
                target self.inner {
                    fn get_modifier_state(&self, key: &str) -> bool;

                    fn alt_key(&self) -> bool;

                    fn ctrl_key(&self) -> bool;

                    fn shift_key(&self) -> bool;

                    fn meta_key(&self) -> bool;
                }
            }
        }

        impl AsRef<web_sys::KeyboardEvent> for $event {
            fn as_ref(&self) -> &web_sys::KeyboardEvent {
                self.as_web_sys_keyboard_event()
            }
        }

        impl_common_ui_event_traits!(KeyboardEvent);
    };
}

/// Event fired when a keyboard key is pressed.
keyboard_event!(KeyDownEvent);

/// Event fired when a keyboard key is released.
keyboard_event!(KeyUpEvent);
