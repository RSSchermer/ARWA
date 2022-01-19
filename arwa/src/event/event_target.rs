use crate::event::event::{DynamicEvent, Event, TypedEvent};
use crate::event::on_event::OnEvent;
use std::marker;

pub mod event_target_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_event_target(&self) -> &web_sys::EventTarget;
    }
}

pub trait EventTarget: event_target_seal::Seal {
    fn on_event(&self, type_name: &str) -> OnEvent<DynamicEvent<Self>> {
        OnEvent::new(self.as_web_sys_event_target(), type_name.to_string().into())
    }

    fn on_typed_event<T>(&self) -> OnEvent<T>
    where
        T: TypedEvent<CurrentTarget = Self>,
    {
        OnEvent::new(self.as_web_sys_event_target(), T::TYPE_NAME.into())
    }

    fn dispatch_event<T>(&self, event: &T)
    where
        T: Event,
    {
        // Shouldn't error here, if we verify at event initialization that event is valid.
        self.as_web_sys_event_target()
            .dispatch_event(event.as_web_sys_event())
            .unwrap();
    }
}

macro_rules! impl_event_target_traits {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl AsRef<web_sys::EventTarget> for $tpe {
            fn as_ref(&self) -> &web_sys::EventTarget {
                &self.inner
            }
        }

        impl $crate::event::event_target::event_target_seal::Seal for $tpe {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                self.as_ref()
            }
        }

        impl $crate::EventTarget for $tpe {}

        impl TryFrom<$crate::event::DynamicEventTarget> for $tpe {
            type Error = $crate::InvalidCast<$tpe>;

            fn try_from(value: $crate::event::DynamicEventTarget) -> Result<Self, Self::Error> {
                let value: web_sys::EventTarget = value.into();

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|e| e.into())
                    .map_err(|e| $crate::InvalidCast(e.into()))
            }
        }

        $crate::impl_common_wrapper_traits!($tpe);
    };
}

pub(crate) use impl_event_target_traits;

pub struct DynamicEventTarget {
    inner: web_sys::EventTarget,
}

impl event_target_seal::Seal for DynamicEventTarget {
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
