use std::any::{Any, TypeId};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::mem;
use std::mem::MaybeUninit;
use std::rc::Rc;

use js_sys::{Promise, Uint8Array};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::event::event::{DynamicEvent, TypedEvent};
use crate::event::on_event::OnEvent;
use crate::js_serialize::{js_deserialize, js_serialize};
use crate::event::type_id_event_name::type_id_to_event_name;
use crate::event::CustomEventData;
use crate::finalization_registry::FinalizationRegistry;
use crate::impl_common_wrapper_traits;

thread_local! {
    static TYPED_CUSTOM_EVENT_REGISTRY: FinalizationRegistry = {
        let callback = |held_value: JsValue| {
            // Reconstruct the Box<dyn Any> that holds the data, then drop it.

            let serialized: Uint8Array = held_value.unchecked_into();

            let mut custom_event_data = MaybeUninit::<CustomEventData>::uninit();
            let ptr = custom_event_data.as_mut_ptr() as *mut ();

            js_deserialize(&wasm_bindgen::memory(), ptr, &serialized);

            let custom_event_data = unsafe { custom_event_data.assume_init() };

            unsafe {
                mem::drop(Box::from_raw(custom_event_data.to_dyn_any_ptr()));
            }
        };

        let boxed = Box::new(callback) as Box<dyn FnMut(JsValue)>;
        let closure = Closure::wrap(boxed);
        let registry = FinalizationRegistry::new(&closure);

        closure.forget();

        registry
    };
}

// EventTarget.dispatchEvent seems to execute all listeners immediately as part of the task in which
// it was called ("synchronously"). This can lead to unexpected behavior, since spawning event
// streams with `spawn_local` will only register the listener as part of a micro-task. To resolve
// this, we move the event dispatch to a micro-task as well. To avoid having to allocate a new
// micro-task closure for every dispatch, we instead use a thread-local dispatch queue. The queue
// here was modified from the queue used by wasm_bindgen_futures.
//
// The trade-off here is the lack of a boolean return value that indicates whether any of the event
// handlers called `preventDefault`. I'm honestly not sure if that is an acceptable trade-off. One
// alternative would be to not dispatch immediately, but instead return a `Dispatch` future, that
// then has to be spawned before it does anything. That future would resolve with the boolean return
// value of the dispatchEvent function. The other alternative would obviously be to not delay the
// dispatch and accept the above mentioned unexpected behavior. We could also offer multiple
// solutions, e.g. a `dispatch_event` function that does not delay and a `dispatch_event_async`
// function that queues a micro-task.

struct DispatchTask {
    target: web_sys::EventTarget,
    event: web_sys::Event,
}

struct DispatchQueueState {
    tasks: RefCell<VecDeque<DispatchTask>>,
    is_spinning: Cell<bool>,
}

impl DispatchQueueState {
    fn run_all(&self) {
        debug_assert!(self.is_spinning.get());

        loop {
            let DispatchTask { target, event } = match self.tasks.borrow_mut().pop_front() {
                Some(task) => task,
                None => break,
            };

            target.dispatch_event(&event).unwrap_throw();
        }

        self.is_spinning.set(false);
    }
}

struct DispatchQueue {
    state: Rc<DispatchQueueState>,
    promise: Promise,
    closure: Closure<dyn FnMut(JsValue)>,
}

impl DispatchQueue {
    fn push_task(&self, task: DispatchTask) {
        self.state.tasks.borrow_mut().push_back(task);

        if !self.state.is_spinning.replace(true) {
            let _ = self.promise.then(&self.closure);
        }
    }
}

impl DispatchQueue {
    fn new() -> Self {
        let state = Rc::new(DispatchQueueState {
            is_spinning: Cell::new(false),
            tasks: RefCell::new(VecDeque::new()),
        });

        Self {
            promise: Promise::resolve(&JsValue::undefined()),

            closure: {
                let state = Rc::clone(&state);

                // This closure will only be called on the next microtask event
                // tick
                Closure::wrap(Box::new(move |_| state.run_all()))
            },

            state,
        }
    }
}

thread_local! {
    static DISPATCH_QUEUE: DispatchQueue = DispatchQueue::new();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EventOptions {
    pub cancelable: bool,
    pub bubbles: bool,
    pub composed: bool,
}

impl Default for EventOptions {
    fn default() -> Self {
        EventOptions {
            cancelable: false,
            bubbles: false,
            composed: false,
        }
    }
}

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
        OnEvent::new(
            self.as_web_sys_event_target().clone(),
            T::EVENT_TYPE.to_cow(),
        )
    }

    fn dispatch_event(&self, event_type: &str, event_data: &JsValue, options: EventOptions) {
        let EventOptions {
            cancelable,
            bubbles,
            composed,
        } = options;

        let mut init = web_sys::CustomEventInit::new();

        init.cancelable(cancelable);
        init.bubbles(bubbles);
        init.composed(composed);
        init.detail(event_data);

        let event =
            web_sys::CustomEvent::new_with_event_init_dict(event_type, &init).unwrap_throw();

        let task = DispatchTask {
            target: self.as_web_sys_event_target().clone(),
            event: event.into(),
        };

        DISPATCH_QUEUE.with(|queue| queue.push_task(task));
    }

    fn dispatch_typed_event<D>(&self, event_data: D, options: EventOptions)
    where
        D: 'static,
    {
        let type_id = TypeId::of::<D>();

        let event_type = type_id_to_event_name(type_id);
        let event_data = Box::new(event_data) as Box<dyn Any>;
        let data_ptr = Box::into_raw(event_data);
        let (address, metadata) = data_ptr.to_raw_parts();

        let mut custom_event_data = CustomEventData { address, metadata };
        let ptr = &mut custom_event_data as *mut CustomEventData as *mut ();
        let event_data = js_serialize(
            &wasm_bindgen::memory(),
            ptr,
            mem::size_of::<CustomEventData>() as u32,
        );

        // Explicitly drop here so that it doesn't drop early
        mem::drop(custom_event_data);

        let EventOptions {
            cancelable,
            bubbles,
            composed,
        } = options;

        let mut init = web_sys::CustomEventInit::new();

        init.cancelable(cancelable);
        init.bubbles(bubbles);
        init.composed(composed);
        init.detail(event_data.as_ref());

        let event =
            web_sys::CustomEvent::new_with_event_init_dict(&event_type, &init).unwrap_throw();

        TYPED_CUSTOM_EVENT_REGISTRY.with(|r| {
            r.register_with_unregister_token(event.as_ref(), event_data.as_ref(), event.as_ref())
        });

        let task = DispatchTask {
            target: self.as_web_sys_event_target().clone(),
            event: event.into(),
        };

        DISPATCH_QUEUE.with(|queue| queue.push_task(task));
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
        impl TryFrom<$crate::event::DynamicEventTarget> for $tpe {
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
