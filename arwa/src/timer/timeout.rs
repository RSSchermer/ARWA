use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::timer::Duration;

enum TimerContext {
    Window(web_sys::Window),
    WorkerGlobalScope(web_sys::WorkerGlobalScope),
}

impl TimerContext {
    fn spawn(&self, callback: &Closure<dyn FnMut()>, milliseconds: u32) -> i32 {
        match self {
            TimerContext::Window(window) => window
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    milliseconds as i32,
                )
                .unwrap_throw(),
            TimerContext::WorkerGlobalScope(worker) => worker
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    milliseconds as i32,
                )
                .unwrap_throw(),
        }
    }

    fn clear(&self, timer_id: i32) {
        match self {
            TimerContext::Window(window) => window.clear_timeout_with_handle(timer_id),
            TimerContext::WorkerGlobalScope(worker) => worker.clear_timeout_with_handle(timer_id),
        }
    }
}

struct CallbackState {
    waker: Option<Waker>,
    next: Option<()>,
}

impl CallbackState {
    fn uninitialized() -> Self {
        CallbackState {
            waker: None,
            next: None,
        }
    }
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct Timeout {
    duration: Duration,
    context: TimerContext,
    callback: Option<Closure<dyn FnMut()>>,
    callback_state: CallbackState,
    timer_id: Option<i32>,
}

impl Timeout {
    pub(crate) fn window_context(window: web_sys::Window, duration: Duration) -> Self {
        Timeout {
            duration,
            context: TimerContext::Window(window),
            callback: None,
            callback_state: CallbackState::uninitialized(),
            timer_id: None,
        }
    }

    pub(crate) fn worker_context(worker: web_sys::WorkerGlobalScope, duration: Duration) -> Self {
        Timeout {
            duration,
            context: TimerContext::WorkerGlobalScope(worker),
            callback: None,
            callback_state: CallbackState::uninitialized(),
            timer_id: None,
        }
    }
}

impl Future for Timeout {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let milliseconds = match self.duration {
            Duration::Infinity => return Poll::Pending,
            Duration::Milliseconds(milliseconds) => milliseconds,
        };

        // Initialize if not initialized
        if self.timer_id.is_none() {
            let state_ptr = (&mut self.callback_state) as *mut CallbackState;

            let callback = move || {
                // This is safe because of Pin
                let CallbackState { next, waker } = unsafe { &mut *state_ptr };

                if let Some(waker) = waker.take() {
                    next.replace(());

                    waker.wake();
                }
            };

            let boxed = Box::new(callback) as Box<dyn FnMut()>;
            let closure = Closure::wrap(boxed);

            let timer_id = self.context.spawn(&closure, milliseconds);

            self.timer_id = Some(timer_id);

            // Hold on to the callback so that it wont drop prematurely.
            self.callback = Some(closure);
        }

        self.callback_state.waker = Some(cx.waker().clone());

        if self.callback_state.next.take().is_some() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

impl Drop for Timeout {
    fn drop(&mut self) {
        if let Some(timer_id) = self.timer_id {
            self.context.clear(timer_id);
        }
    }
}
