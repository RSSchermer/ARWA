use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::event::event::{DynamicEvent, Event, TypedEvent};
use crate::event::on_event::OnEvent;
use crate::impl_common_wrapper_traits;

pub mod event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn from_web_sys_event_target_unchecked(event_target: web_sys::EventTarget) -> Self;

        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait EventTarget: event_target_seal::Seal + Sized {
    fn on_event(&self, type_name: &str) -> OnEvent<DynamicEvent<Self>> {
        OnEvent::new(
            self.as_web_sys_event_target().clone(),
            type_name.to_string().into(),
        )
    }

    fn on_typed_event<T>(&self) -> OnEvent<T>
    where
        T: TypedEvent<CurrentTarget = Self>,
    {
        OnEvent::new(self.as_web_sys_event_target().clone(), T::TYPE_NAME.into())
    }

    fn dispatch_event<T>(&self, event: &T)
    where
        T: Event,
    {
        // Shouldn't error here, if we verify at event initialization that event is valid.
        self.as_web_sys_event_target()
            .dispatch_event(event.as_web_sys_event())
            .unwrap_throw();
    }
}

macro_rules! impl_event_target_traits {
    ($tpe:ident) => {
        impl AsRef<web_sys::EventTarget> for $tpe {
            fn as_ref(&self) -> &web_sys::EventTarget {
                &self.inner
            }
        }

        impl $crate::event::event_target_seal::Seal for $tpe {
            fn from_web_sys_event_target_unchecked(event_target: web_sys::EventTarget) -> Self {
                use wasm_bindgen::JsCast;

                $tpe {
                    inner: event_target.unchecked_into(),
                }
            }

            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                self.as_ref()
            }
        }

        impl $crate::event::EventTarget for $tpe {}

        $crate::impl_common_wrapper_traits!($tpe);
    };
}

pub(crate) use impl_event_target_traits;

macro_rules! impl_try_from_event_target {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl std::convert::TryFrom<$crate::event::DynamicEventTarget> for $tpe {
            type Error = $crate::InvalidCast<$crate::event::DynamicEventTarget, $tpe>;

            fn try_from(value: $crate::event::DynamicEventTarget) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::EventTarget = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| $crate::InvalidCast::new(e.into()))
            }
        }
    };
    ($tpe:ident) => {
        $crate::event::impl_try_from_event_target!($tpe, $tpe);
    };
}

pub(crate) use impl_try_from_event_target;

pub struct DynamicEventTarget {
    inner: web_sys::EventTarget,
}

impl event_target_seal::Seal for DynamicEventTarget {
    fn from_web_sys_event_target_unchecked(inner: web_sys::EventTarget) -> Self {
        DynamicEventTarget { inner }
    }

    fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
        self.inner.unchecked_ref()
    }
}

impl EventTarget for DynamicEventTarget {}

impl AsRef<web_sys::EventTarget> for DynamicEventTarget {
    fn as_ref(&self) -> &web_sys::EventTarget {
        event_target_seal::Seal::as_web_sys_event_target(self)
    }
}

impl From<web_sys::EventTarget> for DynamicEventTarget {
    fn from(inner: web_sys::EventTarget) -> Self {
        DynamicEventTarget { inner }
    }
}

impl From<DynamicEventTarget> for web_sys::EventTarget {
    fn from(wrapper: DynamicEventTarget) -> Self {
        wrapper.inner
    }
}

impl_common_wrapper_traits!(DynamicEventTarget);
