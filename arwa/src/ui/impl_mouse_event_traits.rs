macro_rules! impl_mouse_event_traits {
    ($event:ident) => {
        impl<T> AsRef<web_sys::MouseEvent> for $event<T> {
            fn as_ref(&self) -> &web_sys::MouseEvent {
                self.inner.as_ref()
            }
        }

        impl<T> $crate::ui::modifier_state_seal::Seal for $event<T> {}

        impl<T> $crate::ui::ModifierState for $event<T> {
            fn get_modifier_state(&self, key: &str) -> bool {
                let mouse_event: &web_sys::MouseEvent = self.as_ref();

                mouse_event.get_modifier_state(key)
            }

            fn alt_key(&self) -> bool {
                let mouse_event: &web_sys::MouseEvent = self.as_ref();

                mouse_event.alt_key()
            }

            fn ctrl_key(&self) -> bool {
                let mouse_event: &web_sys::MouseEvent = self.as_ref();

                mouse_event.ctrl_key()
            }

            fn shift_key(&self) -> bool {
                let mouse_event: &web_sys::MouseEvent = self.as_ref();

                mouse_event.shift_key()
            }

            fn meta_key(&self) -> bool {
                let mouse_event: &web_sys::MouseEvent = self.as_ref();

                mouse_event.meta_key()
            }
        }

        impl<T> $crate::ui::pointer_button_state_seal::Seal for $event<T> {
            fn as_web_sys_mouse_event(&self) -> &web_sys::MouseEvent {
                self.as_ref()
            }
        }

        impl<T> $crate::ui::PointerButtonState for $event<T> {}

        impl<T> $crate::ui::pointer_position_state_seal::Seal for $event<T> {
            fn as_web_sys_mouse_event(&self) -> &web_sys::MouseEvent {
                self.as_ref()
            }
        }

        impl<T> $crate::ui::PointerPositionState for $event<T> {}
    };
}

pub(crate) use impl_mouse_event_traits;
