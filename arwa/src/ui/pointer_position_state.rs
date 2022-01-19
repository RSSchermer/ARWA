pub(crate) mod pointer_position_state_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_mouse_event(&self) -> &web_sys::MouseEvent;
    }
}

pub trait PointerPositionState: pointer_position_state_seal::Seal {
    // Note: web_sys declares all these as i32, because they derive from the MouseEvent interface,
    // which is specced to return `long` values. However, the pointer events interface specs these
    // as `double`. Legacy mouse events that are now pointer events (`click`, `auxclick`,
    // `contextmenu`) are then specced to return rounded values, whereas all "true" pointer events
    // can return values with sub-pixel accuracy.
    // TODO: get web_sys to return f64 values, possible by using a custom MouseEvent IDL that uses
    // `double` types for these properties. For now, cast and accept rounding for all events, even
    // "true" pointer events.

    fn screen_x(&self) -> f64 {
        self.as_web_sys_mouse_event().screen_x() as f64
    }

    fn screen_y(&self) -> f64 {
        self.as_web_sys_mouse_event().screen_y() as f64
    }

    fn client_x(&self) -> f64 {
        self.as_web_sys_mouse_event().client_x() as f64
    }

    fn client_y(&self) -> f64 {
        self.as_web_sys_mouse_event().client_y() as f64
    }

    fn offset_x(&self) -> f64 {
        self.as_web_sys_mouse_event().offset_x() as f64
    }

    fn offset_y(&self) -> f64 {
        self.as_web_sys_mouse_event().offset_y() as f64
    }

    fn region(&self) -> Option<String> {
        self.as_web_sys_mouse_event().region()
    }
}
