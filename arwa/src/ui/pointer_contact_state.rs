pub(crate) mod pointer_contact_state_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_pointer_event(&self) -> &web_sys::PointerEvent;
    }
}

pub trait PointerContactState: pointer_contact_state_seal::Seal {
    fn width(&self) -> i32 {
        self.as_web_sys_pointer_event().width()
    }

    fn height(&self) -> i32 {
        self.as_web_sys_pointer_event().height()
    }

    fn pressure(&self) -> f32 {
        self.as_web_sys_pointer_event().pressure()
    }

    fn tangential_pressure(&self) -> f32 {
        self.as_web_sys_pointer_event().tangential_pressure()
    }

    fn tilt_x(&self) -> i32 {
        self.as_web_sys_pointer_event().tilt_x()
    }

    fn tilt_y(&self) -> i32 {
        self.as_web_sys_pointer_event().tilt_y()
    }

    fn twist(&self) -> i32 {
        self.as_web_sys_pointer_event().twist()
    }
}
