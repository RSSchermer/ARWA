use std::convert::TryFrom;

use crate::dom::event_target::EventTarget;
use crate::event::event_target::{DynamicEventTarget, EventTarget};
use crate::event::on_event::FromEvent;
use std::marker;
use wasm_bindgen::JsCast;

pub(crate) mod event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn from_web_sys_event_unchecked(event: web_sys::Event) -> Self;

        #[doc(hidden)]
        fn as_web_sys_event(&self) -> web_sys::Event;
    }
}

pub trait Event: event_seal::Seal {
    type CurrentTarget: EventTarget;

    fn bubbles(&self) -> bool {
        self.as_web_sys_event().bubbles()
    }

    fn cancelable(&self) -> bool {
        self.as_web_sys_event().cancelable()
    }

    fn composed(&self) -> bool {
        self.as_web_sys_event().composed()
    }

    fn is_trusted(&self) -> bool {
        self.as_web_sys_event().is_trusted()
    }

    fn default_prevented(&self) -> bool {
        self.as_web_sys_event().default_prevented()
    }

    fn event_phase(&self) -> EventPhase {
        match self.as_web_sys_event().event_phase() {
            web_sys::Event::NONE => EventPhase::None,
            web_sys::Event::CAPTURING_PHASE => EventPhase::CapturingPhase,
            web_sys::Event::AT_TARGET => EventPhase::AtTarget,
            web_sys::Event::BUBBLING_PHASE => EventPhase::BubblingPhase,
            _ => unreachable!(),
        }
    }

    fn type_name(&self) -> String {
        self.as_web_sys_event().type_()
    }

    fn target(&self) -> Option<DynamicEventTarget> {
        self.as_web_sys_event().target().map(|t| t.into())
    }

    fn current_target(&self) -> Option<Self::CurrentTarget> {
        self.as_web_sys_event().current_target().map(|t| t.into())
    }

    fn composed_path(&self) -> ComposedPath {
        ComposedPath {
            inner: self.as_web_sys_event().composed_path(),
        }
    }

    fn prevent_default(&self) {
        self.as_web_sys_event().prevent_default()
    }

    fn stop_propagation(&self) {
        self.as_web_sys_event().stop_propagation()
    }

    fn stop_immediate_propagation(&self) {
        self.as_web_sys_event().stop_immediate_propagation()
    }
}

pub trait TypedEvent: Event {
    const TYPE_NAME: &'static str;
}

macro_rules! impl_typed_event_traits {
    ($tpe:ident, $web_sys_tpe:ident, $tpe_name:literal) => {
        impl<T> AsRef<web_sys::Event> for $tpe<T> {
            fn as_ref(&self) -> &web_sys::Event {
                &self.inner
            }
        }

        impl<T> $crate::event::event::event_seal::Seal for $tpe<T>
        where
            T: $crate::event::EventTarget,
        {
            fn as_web_sys_event(&self) -> &web_sys::Event {
                self.as_ref()
            }

            fn from_web_sys_event_unchecked(event: web_sys::Event) -> Self {
                $tpe {
                    inner: event.unchecked_into(),
                    _marker: marker::PhantomData,
                }
            }
        }

        impl<T> $crate::event::Event for $tpe<T>
        where
            T: $crate::event::EventTarget,
        {
            type CurrentTarget = T;
        }

        impl<T> $crate::event::TypedEvent for $tpe<T>
        where
            T: $crate::event::EventTarget,
        {
            const TYPE_NAME: &'static str = $tpe_name;
        }

        impl<T> TryFrom<$crate::event::DynamicEvent<T>> for $tpe<T> {
            type Error = $crate::InvalidCast<$tpe<T>>;

            fn try_from(value: DynamicEvent) -> Result<Self, Self::Error> {
                let value: web_sys::Event = value.into();

                if &value.type_() != $tpe_name {
                    return Err($crate::InvalidCast(value.into()));
                }

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| $crate::InvalidCast(e.into()))
            }
        }

        $crate::impl_common_wrapper_traits!($tpe);
    };
}

pub(crate) use impl_typed_event_traits;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventPhase {
    None,
    CapturingPhase,
    AtTarget,
    BubblingPhase,
}

unchecked_cast_array_wrapper!(
    DynamicEventTarget,
    web_sys::EventTarget,
    ComposedPath,
    ComposedPathIter
);

pub struct DynamicEvent<T> {
    inner: web_sys::Event,
    _marker: marker::PhantomData<T>,
}

impl<T> Clone for DynamicEvent<T> {
    fn clone(&self) -> Self {
        DynamicEvent {
            inner: self.inner.clone(),
            _marker: marker::PhantomData,
        }
    }
}

impl<T> From<web_sys::Event> for DynamicEvent<T> {
    fn from(inner: web_sys::Event) -> Self {
        DynamicEvent {
            inner,
            _marker: marker::PhantomData,
        }
    }
}

impl<T> AsRef<web_sys::Event> for DynamicEvent<T> {
    fn as_ref(&self) -> &web_sys::Event {
        &self.inner
    }
}

impl<T> event_seal::Seal for DynamicEvent<T> {
    fn from_web_sys_event_unchecked(event: web_sys::Event) -> Self {
        event.into()
    }

    fn as_web_sys_event(&self) -> &web_sys::Event {
        self.as_ref()
    }
}

impl<T> Event for DynamicEvent<T> {
    type CurrentTarget = T;
}

impl_common_wrapper_traits!(DynamicEvent);
