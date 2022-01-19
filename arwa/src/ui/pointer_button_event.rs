#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PointerButton {
    Main,
    Auxiliary,
    Secondary,
    Fourth,
    Fifth,
    Eraser,
}

impl Into<i16> for PointerButton {
    fn into(self) -> i16 {
        match self {
            PointerButton::Main => 0,
            PointerButton::Auxiliary => 1,
            PointerButton::Secondary => 2,
            PointerButton::Fourth => 3,
            PointerButton::Fifth => 4,
            PointerButton::Eraser => 5,
        }
    }
}

pub(crate) mod pointer_button_event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_mouse_event(&self) -> &web_sys::MouseEvent;
    }
}

pub trait PointerButtonEvent: pointer_button_event_seal::Seal {
    fn button(&self) -> PointerButton {
        match self.inner.button() {
            0 => PointerButton::Primary,
            1 => PointerButton::Auxiliary,
            2 => PointerButton::Secondary,
            3 => PointerButton::Fourth,
            4 => PointerButton::Fifth,
            _ => unreachable!(),
        }
    }
}
