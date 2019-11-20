use futures::Stream;
use futures::task::{Context, Poll};
use std::pin::Pin;
use web_sys::{window, Event, EventTarget};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;

pub struct HtmlButtonElement {
    inner: web_sys::HtmlButtonElement
}

impl From<web_sys::HtmlButtonElement> for HtmlButtonElement {
    fn from(inner: web_sys::HtmlButtonElement) -> Self {
        HtmlButtonElement {
            inner
        }
    }
}

impl HtmlButtonElement {
    pub fn on_click(&self) -> OnClick {
        OnClick::new( self.inner.clone().into())
    }
}

struct OnEvent<T> {
    target: EventTarget,
    event_type: &'static str,
    // Note: the actual event listener should be deregistered when the `gloo_events::EventListener`
    // is dropped. This means that if the stream completes (even though the event stream itself is
    // an infinite stream it can be cut short by a combinator), then the event listener should be
    // properly removed when the async runtime drops the task without leaking.
    listener: Option<EventListener>,
    next: Rc<RefCell<Option<T>>>
}

impl<T> OnEvent<T> {
    fn new(target: EventTarget, event_type: &'static str) -> Self {
        OnEvent {
            target,
            event_type,
            listener: None,
            next: Rc::new(RefCell::new(None))
        }
    }
}

impl<T> Stream for OnEvent<T> where T: FromEvent + 'static {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.listener.is_none() {
            let next = self.next.clone();
            let waker = cx.waker().clone();

            self.listener = Some(EventListener::new(&self.target, self.event_type, move |event| {
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
            }));
        }

        if let Some(event) = self.next.borrow_mut().take() {
            Poll::Ready(Some(event))
        } else {
            Poll::Pending
        }
    }
}

trait FromEvent {
    fn from_event(event: Event) -> Self;
}

#[must_use = "streams do nothing unless polled or spawned"]
pub struct OnClick {
    inner: OnEvent<MouseEvent>
}

impl OnClick {
    fn new(target: EventTarget) -> Self {
        OnClick {
            inner: OnEvent::new(target, "click")
        }
    }
}

impl Stream for OnClick {
    type Item = MouseEvent;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        unsafe {
            self.map_unchecked_mut(|s| &mut s.inner).poll_next(cx)
        }
    }
}

pub struct MouseEvent {
    inner: web_sys::MouseEvent
}

impl FromEvent for MouseEvent {
    fn from_event(event: Event) -> Self {
        MouseEvent {
            inner: event.unchecked_into()
        }
    }
}
