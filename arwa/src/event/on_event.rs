use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use futures::Stream;
use gloo_events::{EventListener, EventListenerOptions, EventListenerPhase};
use crate::event::{Phase, EventStreamOptions};

// TODO: we are leaking when we queue infinite event streams as tasks and all other references to
// the actual event target node get dropped (and the node is removed from the DOM). The can
// eventually be resolved once the [WeakReferences TC39 proposal](https://github.com/tc39/proposal-weakrefs)
// is accepted (it's stage 3 as of this writing). For now, the solution is to ensure that a stream
// is not infinite, e.g. by using the `stream_cancel::TakeUntil` combinator. Note that the same
// problem exists for e.g. a raw `gloo_events::EventListener`, the difference being that for the
// stream case one gives away ownership to the executor (which will keep it alive until it
// finishes), thus requiring additional effort to trigger a drop (by making it finish).

pub(super) struct OnEvent<T> {
    target: web_sys::EventTarget,
    event_type: &'static str,
    // Note: the actual event listener should be deregistered when the `gloo_events::EventListener`
    // is dropped. This means that if the stream completes (even though the event stream itself is
    // an infinite stream it can be cut short by a combinator), then the event listener should be
    // properly removed when the async runtime drops the task without leaking.
    listener: Option<EventListener>,
    next: Rc<RefCell<Option<T>>>,
}

impl<T> OnEvent<T> {
    pub(super) fn new(target: web_sys::EventTarget, event_type: &'static str) -> Self {
        OnEvent {
            target,
            event_type,
            listener: None,
            next: Rc::new(RefCell::new(None)),
        }
    }
}

impl<T> Stream for OnEvent<T>
where
    T: FromEvent + 'static,
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.listener.is_none() {
            let next = self.next.clone();
            let waker = cx.waker().clone();

            self.listener = Some(EventListener::new(
                &self.target,
                self.event_type,
                move |event| {
                    // We should not be dropping events here if this gets run with wasm-bindgen-futures
                    // `spawn_local`, as invoking the waker will immediately queue running the task as a
                    // micro-task on the current thread/workers event queue, whereas all user events get
                    // queued as macro tasks: this means that there's always a call to `poll_next`
                    // before the next event gets processed and we get away with only buffering 1 event.
                    //
                    // However, can this behaviour be assumed for all async runtimes that people may use
                    // in the browser?
                    //
                    // The following consideration perhaps helps: this stream is not `Send`, it has to
                    // be run on the local thread. This should only leave 3 options for the waker
                    // implementation:
                    //
                    // 1.  Poll synchronously/immediately upon calling the waker.
                    // 2.  Schedule a micro-task on the local thread's event loop to poll later
                    //     (wasm-bindgen-futures current approach).
                    // 3.  Schedule a macro-task on the local thread's event loop to poll later.
                    //
                    // Only 3. would be problematic and I think it can be argued that any reasonable
                    // implementation would favor 2. over 3.

                    next.borrow_mut().replace(T::from_event(event.clone()));

                    waker.wake_by_ref();
                },
            ));
        }

        if let Some(event) = self.next.borrow_mut().take() {
            Poll::Ready(Some(event))
        } else {
            Poll::Pending
        }
    }
}

pub(super) struct OnEventWithOptions<T> {
    target: web_sys::EventTarget,
    event_type: &'static str,
    // Note: the actual event listener should be deregistered when the `gloo_events::EventListener`
    // is dropped. This means that if the stream completes (even though the event stream itself is
    // an infinite stream it can be cut short by a combinator), then the event listener should be
    // properly removed when the async runtime drops the task without leaking.
    listener: Option<EventListener>,
    next: Rc<RefCell<Option<T>>>,
    phase: Phase,
    passive: bool
}

impl<T> OnEventWithOptions<T> {
    pub(super) fn new(on_event: OnEvent<T>, options: EventStreamOptions) -> Self {
        let OnEvent {
            target,
            event_type,
            next,
            ..
        } = on_event;

        let EventStreamOptions {
            phase, passive
        } = options;

        OnEventWithOptions {
            target,
            event_type,
            listener: None,
            next,
            phase,
            passive
        }
    }
}

impl<T> Stream for OnEventWithOptions<T>
    where
        T: FromEvent + 'static,
{
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.listener.is_none() {
            let next = self.next.clone();
            let waker = cx.waker().clone();

            let event_listener_phase = match self.phase {
                Phase::Bubble => EventListenerPhase::Bubble,
                Phase::Capture => EventListenerPhase::Capture,
            };

            self.listener = Some(EventListener::new_with_options(
                &self.target,
                self.event_type,
                EventListenerOptions {
                    phase: event_listener_phase,
                    passive: self.passive
                },
                move |event| {
                    next.borrow_mut().replace(T::from_event(event.clone()));

                    waker.wake_by_ref();
                },
            ));
        }

        if let Some(event) = self.next.borrow_mut().take() {
            Poll::Ready(Some(event))
        } else {
            Poll::Pending
        }
    }
}

pub(super) trait FromEvent {
    fn from_event(event: web_sys::Event) -> Self;
}
