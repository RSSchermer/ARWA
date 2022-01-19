bitflags! {
    pub struct PointerButtons: u16 {
        const PRIMARY =   0b00000001;
        const SECONDARY = 0b00000010;
        const AUXILIARY = 0b00000100;
        const FOURTH =    0b00001000;
        const FIFTH =     0b00010000;
        const ERASER =    0b00100000;
    }
}

pub(crate) mod pointer_button_state_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_mouse_event(&self) -> &web_sys::MouseEvent;
    }
}

pub trait PointerButtonState: pointer_button_state_seal::Seal {
    fn buttons(&self) -> PointerButtons {
        PointerButtons::from_bits_truncate(self.as_web_sys_mouse_event().buttons())
    }
}
