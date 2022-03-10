use std::borrow::Cow;
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll, Waker};

use futures::stream::Stream;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::event::Event;
use crate::finalization_registry::FinalizationRegistry;

thread_local! {
    static ON_EVENT_REGISTRY: FinalizationRegistry = {
        let callback = |held_value: JsValue| {
            // This is obviously not great, but I cannot currently find another way to get a usize
            // back from a JsValue.
            let big_int: BigInt = held_value.unchecked_into();
            let string: String = ToString::to_string(&big_int);
            let ptr_bits = usize::from_str(&string).unwrap_throw();

            // This is safe because registration only ever happens after the stream has been pinned,
            // and it unregisters on drop, so the pointer is always valid for the entire
            // registration window.
            let internal = unsafe { &mut *<* mut Internal<JsValue>>::from_bits(ptr_bits) };

            internal.terminated = true;

            if let Some(waker) = internal.state.waker.take() {
                waker.wake();
            }
        };

        let boxed = Box::new(callback) as Box<dyn FnMut(JsValue)>;
        let closure = Closure::wrap(boxed);
        let registry = FinalizationRegistry::new(&closure);

        closure.forget();

        registry
    };
}

struct Internal<T> {
    target: WeakRef,
    event_type: Cow<'static, str>,
    callback: Option<Closure<dyn FnMut(web_sys::Event)>>,
    use_capture: bool, // We need this to drop properly
    terminated: bool,
    state: CallbackState<T>,
}

impl<T> Internal<T> {
    fn uninitialized(target: web_sys::EventTarget, event_type: Cow<'static, str>) -> Self {
        Internal {
            target: WeakRef::new(target.as_ref()),
            event_type,
            callback: None,
            use_capture: false,
            terminated: false,
            state: CallbackState::uninitialized(),
        }
    }

    fn is_uninitialized(&self) -> bool {
        self.callback.is_none()
    }
}

impl<T> Internal<T>
where
    T: Event + 'static,
{
    fn initialize(self: Pin<&mut Self>, options: &EventIteratorOptions) {
        // Use `get_unchecked_mut` here to avoid having to restrict `T` to `Unpin`
        let internal = unsafe { self.get_unchecked_mut() };

        if internal.callback.is_some() {
            panic!("Cannot initialize an event stream twice.");
        }

        let target = internal.target.deref();

        if target.is_undefined() {
            internal.terminated = true;

            return;
        }

        let state_ptr = (&mut internal.state) as *mut CallbackState<T>;

        let callback = move |event| {
            // We should not be dropping events here if this gets run with wasm-bindgen-futures
            // `spawn_local`, as invoking the waker will immediately queue running the task as a
            // micro-task on the current thread/workers event queue, whereas all user events get
            // queued as macro tasks: this means that there's always a call to `poll_next`
            // before the next event gets processed and we get away with only buffering 1 event.
            //
            // This implementation is not meant to be tied to wasm-bindgen-futures specifically,
            // however, as this stream is not `Send`, potential executor implementations should be
            // limited to the following 3 execution patterns:
            //
            // 1.  Immediately poll the task synchronously when the waker is called.
            // 2.  Schedule a micro-task on the event loop to poll later (wasm-bindgen-futures
            //     current approach).
            // 3.  Schedule a macro-task on the event loop to poll later.
            //
            // Only 3. would be problematic and I think it can be argued that any reasonable
            // implementation would favor 2. over 3.

            // We know the state_ptr will always deref successfully because of Pin's guarantees.
            let CallbackState { next, waker } = unsafe { &mut *state_ptr };

            if let Some(waker) = waker.take() {
                next.replace(T::from_web_sys_event_unchecked(event));

                waker.wake();
            }
        };

        let boxed = Box::new(callback) as Box<dyn FnMut(web_sys::Event)>;
        let closure = Closure::wrap(boxed);

        let use_capture = match options.phase {
            Phase::Capture => true,
            Phase::Bubble => false,
        };

        let mut add_event_lister_options = web_sys::AddEventListenerOptions::new();

        add_event_lister_options.capture(use_capture);
        add_event_lister_options.passive(options.passive);

        let target: web_sys::EventTarget = target.unchecked_into();

        target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &internal.event_type,
                closure.as_ref().unchecked_ref(),
                &add_event_lister_options,
            )
            .unwrap_throw();

        // Hang on to the closure so we don't drop it while the listener is still active on the
        // target.
        internal.callback = Some(closure);
        internal.use_capture = use_capture;

        let ptr = internal as *mut Internal<T>;
        let ptr_bits = ptr.to_bits();

        ON_EVENT_REGISTRY.with(|r| {
            r.register_with_unregister_token(target.as_ref(), &ptr_bits.into(), target.as_ref());
        });
    }

    fn refresh_waker(&mut self, waker: Waker) {
        self.state.waker = Some(waker)
    }

    fn next(&mut self) -> Option<T> {
        self.state.next.take()
    }
}

impl<T> Drop for Internal<T> {
    fn drop(&mut self) {
        let target = self.target.deref();

        if !target.is_undefined() {
            ON_EVENT_REGISTRY.with(|r| r.unregister(target.as_ref()));

            if let Some(callback) = &self.callback {
                let target: web_sys::EventTarget = target.unchecked_into();

                target
                    .remove_event_listener_with_callback_and_bool(
                        &self.event_type,
                        callback.as_ref().unchecked_ref(),
                        self.use_capture,
                    )
                    .unwrap_throw();
            }
        }
    }
}

struct CallbackState<T> {
    waker: Option<Waker>,
    next: Option<T>,
}

impl<T> CallbackState<T> {
    fn uninitialized() -> Self {
        CallbackState {
            waker: None,
            next: None,
        }
    }
}

#[must_use = "streams do nothing unless polled or spawned"]
pub struct OnEvent<T> {
    internal: Internal<T>,
}

impl<T> OnEvent<T> {
    pub(crate) fn new(target: web_sys::EventTarget, event_type: Cow<'static, str>) -> Self {
        OnEvent {
            internal: Internal::uninitialized(target, event_type),
        }
    }

    pub fn with_options(self, options: EventIteratorOptions) -> OnEventWithOptions<T> {
        OnEventWithOptions::new(self, options)
    }
}

impl<T> Stream for OnEvent<T>
where
    T: Event + 'static,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Use `get_unchecked_mut` here to avoid having to restrict `T` to `Unpin`
        let on_event = unsafe { &mut self.get_unchecked_mut() };

        if on_event.internal.terminated {
            return Poll::Ready(None);
        }

        // If the stream hasn't been initialized yet, initialize it
        if on_event.internal.is_uninitialized() {
            // We dont move internal, so this is safe.
            unsafe {
                Pin::new_unchecked(&mut on_event.internal)
                    .initialize(&EventIteratorOptions::default());
            };
        }

        // Set a new waker to keep the async iterator alive (or set the initial waker if we've just
        // initialized).
        on_event.internal.refresh_waker(cx.waker().clone());

        // Return the most recent event, if any.
        if let Some(event) = on_event.internal.next() {
            Poll::Ready(Some(event))
        } else {
            Poll::Pending
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Phase {
    Bubble,
    Capture,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct EventIteratorOptions {
    pub phase: Phase,
    pub passive: bool,
}

impl Default for EventIteratorOptions {
    fn default() -> Self {
        EventIteratorOptions {
            phase: Phase::Bubble,
            passive: true,
        }
    }
}

#[must_use = "streams do nothing unless polled or spawned"]
pub struct OnEventWithOptions<T> {
    internal: Internal<T>,
    options: EventIteratorOptions,
}

impl<T> OnEventWithOptions<T> {
    pub(crate) fn new(on_event: OnEvent<T>, options: EventIteratorOptions) -> Self {
        OnEventWithOptions {
            internal: on_event.internal,
            options,
        }
    }
}

impl<T> Stream for OnEventWithOptions<T>
where
    T: Event + 'static,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Use `get_unchecked_mut` here to avoid having to restrict `T` to `Unpin`
        let on_event = unsafe { self.get_unchecked_mut() };

        if on_event.internal.terminated {
            return Poll::Ready(None);
        }

        // If the stream hasn't been initialized yet, initialize it
        if on_event.internal.is_uninitialized() {
            let options = on_event.options;

            // We dont move internal, so this is safe.
            unsafe { Pin::new_unchecked(&mut on_event.internal).initialize(&options) }
        }

        // Set a new waker to keep the async iterator alive (or set the initial waker if we've just
        // initialized).
        on_event.internal.refresh_waker(cx.waker().clone());

        // Return the most recent event, if any.
        if let Some(event) = on_event.internal.next() {
            Poll::Ready(Some(event))
        } else {
            Poll::Pending
        }
    }
}

macro_rules! typed_event_iterator {
    ($iterator:ident, $iterator_with_options:ident, $event:ident, $name:tt) => {
        #[must_use = "streams do nothing unless polled or spawned"]
        pub struct $iterator<T> {
            inner: $crate::event::OnEvent<$event<T>>,
        }

        impl<T> $iterator<T> {
            pub(crate) fn new(target: &web_sys::EventTarget) -> Self {
                $iterator {
                    inner: $crate::event::OnEvent::new(
                        target.clone(),
                        std::borrow::Cow::Borrowed($name),
                    ),
                }
            }

            pub fn with_options(
                self,
                options: $crate::event::EventIteratorOptions,
            ) -> $iterator_with_options<T> {
                $iterator_with_options {
                    inner: $crate::event::OnEventWithOptions::new(self.inner, options),
                }
            }
        }

        impl<T> futures::stream::Stream for $iterator<T>
        where
            T: $crate::event::EventTarget + 'static,
        {
            type Item = $event<T>;

            fn poll_next(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<Self::Item>> {
                unsafe { self.map_unchecked_mut(|s| &mut s.inner).poll_next(cx) }
            }
        }

        #[must_use = "streams do nothing unless polled or spawned"]
        pub struct $iterator_with_options<T> {
            inner: $crate::event::OnEventWithOptions<$event<T>>,
        }

        impl<T> futures::stream::Stream for $iterator_with_options<T>
        where
            T: $crate::event::EventTarget + 'static,
        {
            type Item = $event<T>;

            fn poll_next(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<Self::Item>> {
                unsafe { self.map_unchecked_mut(|s| &mut s.inner).poll_next(cx) }
            }
        }
    };
}

use crate::weak_ref::WeakRef;
use js_sys::BigInt;
pub(crate) use typed_event_iterator;
