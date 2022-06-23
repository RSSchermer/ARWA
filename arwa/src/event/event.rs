use std::marker;

use web_sys::EventTarget as WebSysEventTarget;

use crate::event::{DynamicEventTarget, EventTarget};
use crate::unchecked_cast_array::unchecked_cast_array;

pub(crate) mod event_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn from_web_sys_event_unchecked(event: web_sys::Event) -> Self;

        #[doc(hidden)]
        fn as_web_sys_event(&self) -> &web_sys::Event;
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
        self.as_web_sys_event()
            .current_target()
            .map(|t| Self::CurrentTarget::from_web_sys_event_target_unchecked(t))
    }

    fn composed_path(&self) -> ComposedPath {
        ComposedPath::new(self.as_web_sys_event().composed_path())
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

pub(crate) enum EventTypeInternal {
    Str(&'static str),
    TypeId(TypeId),
}

pub struct EventType {
    pub(crate) internal: EventTypeInternal,
}

impl EventType {
    pub(crate) fn to_cow(&self) -> Cow<'static, str> {
        match self.internal {
            EventTypeInternal::Str(str) => str.into(),
            EventTypeInternal::TypeId(type_id) => type_id_to_event_name(type_id).into(),
        }
    }
}

pub trait TypedEvent: Event {
    const EVENT_TYPE: EventType;
}

macro_rules! impl_typed_event_traits {
    ($tpe:ident, $web_sys_tpe:ident, $tpe_name:literal) => {
        impl<T> AsRef<web_sys::Event> for $tpe<T> {
            fn as_ref(&self) -> &web_sys::Event {
                &self.inner
            }
        }

        impl<T> $crate::event::event_seal::Seal for $tpe<T>
        where
            T: $crate::event::EventTarget,
        {
            fn as_web_sys_event(&self) -> &web_sys::Event {
                self.as_ref()
            }

            fn from_web_sys_event_unchecked(event: web_sys::Event) -> Self {
                use wasm_bindgen::JsCast;

                $tpe {
                    inner: event.unchecked_into(),
                    _marker: std::marker::PhantomData,
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
            const EVENT_TYPE: $crate::event::EventType = $crate::event::EventType {
                internal: $crate::event::EventTypeInternal::Str($tpe_name),
            };
        }

        impl<T> TryFrom<$crate::event::DynamicEvent<T>> for $tpe<T> {
            type Error = $crate::InvalidCast<$crate::event::DynamicEvent<T>, $tpe<T>>;

            fn try_from(value: $crate::event::DynamicEvent<T>) -> Result<Self, Self::Error> {
                use wasm_bindgen::JsCast;

                let value: web_sys::Event = value.into();

                if &value.type_() != $tpe_name {
                    return Err($crate::InvalidCast::new($crate::event::DynamicEvent::from(
                        value,
                    )));
                }

                value
                    .dyn_into::<web_sys::$web_sys_tpe>()
                    .map(|inner| $tpe {
                        inner,
                        _marker: std::marker::PhantomData,
                    })
                    .map_err(|e| $crate::InvalidCast::new($crate::event::DynamicEvent::from(e)))
            }
        }

        impl<T> AsRef<js_sys::Object> for $tpe<T> {
            fn as_ref(&self) -> &js_sys::Object {
                self.inner.as_ref()
            }
        }

        impl<T> AsRef<wasm_bindgen::JsValue> for $tpe<T> {
            fn as_ref(&self) -> &wasm_bindgen::JsValue {
                self.inner.as_ref()
            }
        }

        impl<T> Into<wasm_bindgen::JsValue> for $tpe<T> {
            fn into(self) -> wasm_bindgen::JsValue {
                self.inner.into()
            }
        }

        impl<T> $crate::console::ToArgument for $tpe<T> {
            fn to_argument(&self) -> $crate::console::Argument {
                let as_js_value: &wasm_bindgen::JsValue = self.as_ref();

                $crate::console::ToArgument::to_argument(as_js_value)
            }
        }
    };
    ($tpe:ident, $tpe_name:literal) => {
        $crate::event::impl_typed_event_traits!($tpe, $tpe, $tpe_name);
    };
}

use crate::console::{Argument, ToArgument};
use crate::event::event_target_seal::Seal;
use crate::event::type_id_event_name::type_id_to_event_name;
pub(crate) use impl_typed_event_traits;
use std::any::TypeId;
use std::borrow::Cow;
use wasm_bindgen::JsValue;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EventPhase {
    None,
    CapturingPhase,
    AtTarget,
    BubblingPhase,
}

unchecked_cast_array!(DynamicEventTarget, WebSysEventTarget, ComposedPath);

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

impl<T> From<DynamicEvent<T>> for web_sys::Event {
    fn from(event: DynamicEvent<T>) -> Self {
        event.inner
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

impl<T> Event for DynamicEvent<T>
where
    T: EventTarget,
{
    type CurrentTarget = T;
}

impl<T> AsRef<js_sys::Object> for DynamicEvent<T> {
    fn as_ref(&self) -> &js_sys::Object {
        self.inner.as_ref()
    }
}

impl<T> AsRef<wasm_bindgen::JsValue> for DynamicEvent<T> {
    fn as_ref(&self) -> &wasm_bindgen::JsValue {
        self.inner.as_ref()
    }
}

impl<T> ToArgument for DynamicEvent<T> {
    fn to_argument(&self) -> Argument {
        let as_js_value: &JsValue = self.as_ref();

        ToArgument::to_argument(as_js_value)
    }
}
