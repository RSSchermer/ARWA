use crate::window::Window;

pub(crate) mod ui_event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_ui_event(&self) -> &web_sys::UiEvent;
    }
}

pub trait UiEvent: ui_event_seal::Seal {
    fn view(&self) -> Option<Window> {
        self.as_web_sys_ui_event().view().map(|w| w.into())
    }
}

macro_rules! impl_ui_event_traits {
    ($tpe:ident, $web_sys_tpe:ident, $name:literal) => {
        impl<T> $crate::ui::ui_event_seal::Seal for $tpe<T> {
            fn as_web_sys_ui_event(&self) -> &web_sys::UiEvent {
                self.inner.as_ref()
            }
        }

        impl<T> $crate::ui::UiEvent for $tpe<T> {}

        impl<T> AsRef<web_sys::UiEvent> for $tpe<T> {
            fn as_ref(&self) -> &web_sys::UiEvent {
                use crate::ui::ui_event_seal::Seal;

                self.as_web_sys_ui_event()
            }
        }

        $crate::event::impl_typed_event_traits!($tpe, $web_sys_tpe, $name);
    };
}

pub(crate) use impl_ui_event_traits;
