use std::borrow::Cow;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use futures::Stream;
use wasm_bindgen::closure::Closure;

use crate::event::Event;
use wasm_bindgen::JsCast;

// TODO: currently these spawn infinite tasks (unless combined with some terminating combinator
// e.g. `Once` or `TakeUntil`). However, if the reference to the EventTarget held by a stream is
// the only remaining reference, then we know the stream can be fused, the closure can be dropped,
// and we can return `Ready(None)` the next time the stream is polled. We should be able to achieve
// this through a combination of the new `WeakRef` and `FinalizationRegistry` web-APIs, where we
// only hold on to a `WeakRef` to the event target and register a finalizer on the event target that
// will clean up the closure and set the stream to a "terminated" state. However, as of yet,
// wasm-bindgen does not expose these APIs.

struct Internal<T> {
    target: web_sys::EventTarget,
    event_type: Cow<'static, str>,
    callback: Option<Closure<dyn FnMut(&web_sys::Event)>>,
    use_capture: bool, // We need this to drop properly
    state: CallbackState<T>,
}

impl<T> Internal<T>
where
    T: Event,
{
    fn uninitialized(target: web_sys::EventTarget, event_type: Cow<'static, str>) -> Self {
        Internal {
            target,
            event_type,
            callback: None,
            use_capture: false,
            state: CallbackState::uninitialized(),
        }
    }

    fn is_uninitialized(&self) -> bool {
        self.callback.is_none()
    }

    fn initialize(mut self: Pin<&mut Self>, options: &EventStreamOptions) {
        if self.callback.is_some() {
            panic!("Cannot initialize an event stream twice.");
        }

        let state_ptr = (&mut self.state) as *mut CallbackState<T>;

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
                next.replace(T::from_event(event.clone()));

                waker.wake();
            }
        };

        let boxed = Box::new(callback) as Box<dyn FnMut(&web_sys::Event)>;
        let closure = Closure::wrap(boxed);

        let use_capture = match options.phase {
            Phase::Capture => true,
            Phase::Bubble => false,
        };

        let mut add_event_lister_options = web_sys::AddEventListenerOptions::new();

        add_event_lister_options.capture(use_capture);
        add_event_lister_options.passive(options.passive);

        self.target
            .add_event_listener_with_callback_and_add_event_listener_options(
                &self.event_type,
                callback.as_ref().unchecked_ref(),
                options,
            )
            .unwrap_throw();

        // Hang on to the closure so we don't drop it while the listener is still active on the
        // target.
        self.callback = Some(closure);
        self.use_capture = use_capture;
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
        if let Some(callback) = &self.callback {
            self.target
                .remove_event_listener_with_callback_and_bool(
                    &self.event_type,
                    callback.as_ref().unchecked_ref(),
                    self.use_capture,
                )
                .unwrap_throw();
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

    pub fn with_options(self, options: EventStreamOptions) -> OnEventWithOptions<T> {
        OnEventWithOptions::new(self, options)
    }
}

impl<T> Stream for OnEvent<T>
where
    T: Event + 'static,
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // If the stream hasn't been initialized yet, initialize it
        if self.internal.is_uninitialized() {
            unsafe {
                self.map_unchecked_mut(|v| &mut v.internal)
                    .initialize(&EventStreamOptions::default())
            }
        }

        // Set a new waker to keep the Stream alive (or set the initial waker if we've just
        // initialized).
        self.internal.refresh_waker(cx.waker().clone());

        // Return the most recent event, if any.
        if let Some(event) = self.internal.next() {
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
pub struct EventStreamOptions {
    pub phase: Phase,
    pub passive: bool,
}

impl Default for EventStreamOptions {
    fn default() -> Self {
        EventStreamOptions {
            phase: Phase::Bubble,
            passive: true,
        }
    }
}

#[must_use = "streams do nothing unless polled or spawned"]
pub struct OnEventWithOptions<T> {
    internal: Internal<T>,
    options: EventStreamOptions,
}

impl<T> OnEventWithOptions<T> {
    pub(crate) fn new(on_event: OnEvent<T>, options: EventStreamOptions) -> Self {
        OnEventWithOptions {
            internal: on_event.internal,
            options,
        }
    }
}

impl<T> Stream for OnEventWithOptions<T>
where
    T: FromEvent + 'static,
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // If the stream hasn't been initialized yet, initialize it
        if self.internal.is_uninitialized() {
            unsafe {
                self.map_unchecked_mut(|v| &mut v.internal)
                    .initialize(&self.options)
            }
        }

        // Set a new waker to keep the Stream alive (or set the initial waker if we've just
        // initialized).
        self.internal.refresh_waker(cx.waker().clone());

        // Return the most recent event, if any.
        if let Some(event) = self.internal.next() {
            Poll::Ready(Some(event))
        } else {
            Poll::Pending
        }
    }
}

macro_rules! typed_event_stream {
    ($stream:ident, $stream_with_options:ident, $event:ident, $name:tt) => {
        #[must_use = "streams do nothing unless polled or spawned"]
        pub struct $stream<T> {
            inner: $crate::event::OnEvent<$event<T>>,
        }

        impl<T> $stream<T> {
            pub(crate) fn new(target: web_sys::EventTarget) -> Self {
                $stream {
                    inner: $crate::event::OnEvent::new(target, $name),
                }
            }

            pub fn with_options(
                self,
                options: $crate::event::EventStreamOptions,
            ) -> $stream_with_options<T> {
                $stream_with_options {
                    inner: $crate::event::OnEventWithOptions::new(self.inner, options),
                }
            }
        }

        impl<T> std::stream::Stream for $stream<T> {
            type Item = $event<T>;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                unsafe { self.map_unchecked_mut(|s| &mut s.inner).poll_next(cx) }
            }
        }

        #[must_use = "streams do nothing unless polled or spawned"]
        pub struct $stream_with_options<T> {
            inner: $crate::event::OnEventWithOptions<$event<T>>,
        }

        impl<T> std::stream::Stream for $stream_with_options<T> {
            type Item = $event<T>;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
                unsafe { self.map_unchecked_mut(|s| &mut s.inner).poll_next(cx) }
            }
        }
    };
}
