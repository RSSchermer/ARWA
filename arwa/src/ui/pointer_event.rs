use crate::event::{impl_typed_event_traits, DynamicEventTarget};
use crate::ui::impl_mouse_event_traits;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
}

pub(crate) mod pointer_event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_pointer_event(&self) -> &web_sys::PointerEvent;
    }
}

pub trait PointerEvent: pointer_event_seal::Seal {
    fn pointer_id(&self) -> i32 {
        self.as_web_sys_pointer_event().pointer_id()
    }

    fn pointer_type(&self) -> PointerType {
        match &*self.as_web_sys_pointer_event().pointer_type() {
            "mouse" => PointerType::Mouse,
            "pen" => PointerType::Pen,
            "touch" => PointerType::Touch,
            _ => unreachable!(),
        }
    }

    fn is_primary(&self) -> bool {
        self.as_web_sys_pointer_event().is_primary()
    }
}

macro_rules! pointer_event {
    ($event:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $event<T> {
            inner: web_sys::PointerEvent,
            _marker: std::marker::PhantomData<T>,
        }

        impl<T> AsRef<web_sys::PointerEvent> for $event<T> {
            fn as_ref(&self) -> &web_sys::PointerEvent {
                &self.inner
            }
        }

        impl<T> pointer_event_seal::Seal for $event<T> {
            fn as_web_sys_pointer_event(&self) -> &web_sys::PointerEvent {
                self.as_ref()
            }
        }

        impl<T> PointerEvent for $event<T> {}

        impl<T> $crate::ui::pointer_contact_state_seal::Seal for $event<T> {
            fn as_web_sys_pointer_event(&self) -> &web_sys::PointerEvent {
                self.as_ref()
            }
        }

        impl<T> $crate::ui::PointerContactState for $event<T> {}

        impl_typed_event_traits!($event, PointerEvent, $name);
        impl_mouse_event_traits!($event);
    };
}

macro_rules! pointer_button_event {
    ($event:ident, $name:literal) => {
        pointer_event!($event, $name);

        impl<T> $crate::ui::pointer_button_event_seal::Seal for $event<T> {
            fn as_web_sys_mouse_event(&self) -> &web_sys::MouseEvent {
                self.as_ref()
            }
        }

        impl<T> $crate::ui::PointerButtonEvent for $event<T> {}
    };
}

pointer_button_event!(ClickEvent, "click");
pointer_button_event!(AuxClickEvent, "auxclick");
pointer_button_event!(ContextMenuEvent, "contextmenu");
pointer_button_event!(PointerDownEvent, "pointerdown");
pointer_button_event!(PointerUpEvent, "pointerup");

pointer_event!(PointerCancelEvent, "pointercancel");
pointer_event!(GotPointerCaptureEvent, "gotpointercapture");
pointer_event!(LostPointerCaptureEvent, "lostpointercapture");

pointer_event!(PointerMoveEvent, "pointermove");

impl<T> PointerMoveEvent<T> {
    // TODO: get web_sys to return f64 values, see pointer_position_state.rs

    pub fn movement_x(&self) -> f64 {
        self.inner.movement_x() as f64
    }

    pub fn movement_y(&self) -> f64 {
        self.inner.movement_y() as f64
    }
}

pointer_event!(PointerEnterEvent, "pointerenter");

impl<T> PointerEnterEvent<T> {
    pub fn exited_target(&self) -> Option<DynamicEventTarget> {
        self.inner.related_target().map(|target| target.into())
    }
}

pointer_event!(PointerLeaveEvent, "pointerleave");

impl<T> PointerLeaveEvent<T> {
    pub fn entered_target(&self) -> Option<DynamicEventTarget> {
        self.inner.related_target().map(|target| target.into())
    }
}

pointer_event!(PointerOverEvent, "pointerover");

impl<T> PointerOverEvent<T> {
    pub fn exited_target(&self) -> Option<DynamicEventTarget> {
        self.inner.related_target().map(|target| target.into())
    }
}

pointer_event!(PointerOutEvent, "pointerout");

impl<T> PointerOutEvent<T> {
    pub fn entered_target(&self) -> Option<DynamicEventTarget> {
        self.inner.related_target().map(|target| target.into())
    }
}
