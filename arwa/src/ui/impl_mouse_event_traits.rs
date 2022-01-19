macro_rules! impl_mouse_event_traits {
    ($event:ident, $web_sys_event:ident, $name:ident) => {
        impl<T> AsRef<web_sys::MouseEvent> for $event<T> {
            fn as_ref(&self) -> &web_sys::MouseEvent {
                self.inner.as_ref()
            }
        }

        impl<T> $crate::ui::modifier_state_seal::Seal for $event<T> {}

        impl<T> $crate::ui::ModifierState for $event<T> {
            fn get_modifier_state(&self, key: &str) -> bool {
                self.as_ref().get_modifier_state(key)
            }

            fn alt_key(&self) -> bool {
                self.as_ref().alt_key()
            }

            fn ctrl_key(&self) -> bool {
                self.as_ref().ctrl_key()
            }

            fn shift_key(&self) -> bool {
                self.as_ref().shift_key()
            }

            fn meta_key(&self) -> bool {
                self.as_ref().meta_key()
            }
        }

        impl<T> $crate::ui::pointer_button_state::Seal {
            fn as_web_sys_mouse_event(&self) -> &web_sys::MouseEvent {
                self.as_ref()
            }
        }

        impl<T> $crate::ui::PointerButtonState for $event<T> {}

        impl<T> $crate::ui::pointer_position_state::Seal {
            fn as_web_sys_mouse_event(&self) -> &web_sys::MouseEvent {
                self.as_ref()
            }
        }

        impl<T> $crate::ui::PointerPositionState for $event<T> {}
    };
}

pub(crate) use impl_mouse_event_traits;
