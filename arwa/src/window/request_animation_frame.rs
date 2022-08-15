use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

#[must_use = "futures do nothing unless polled or spawned"]
pub struct RequestAnimationFrame {
    context: web_sys::Window,
    time: Option<f64>,
    handle: Option<i32>,
    callback: Option<Closure<dyn FnMut(JsValue)>>,
}

impl RequestAnimationFrame {
    pub(crate) fn new(context: web_sys::Window) -> Self {
        RequestAnimationFrame {
            context,
            time: None,
            handle: None,
            callback: None,
        }
    }
}

impl Future for RequestAnimationFrame {
    type Output = f64;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(time) = self.time {
            return Poll::Ready(time);
        }

        if self.handle.is_none() {
            // Initialize
            let time_ptr = &mut self.time as *mut Option<f64>;
            let mut waker = Some(cx.waker().clone());

            let callback = Closure::wrap(Box::new(move |t: JsValue| {
                let t = t.as_f64().unwrap_throw();

                if let Some(waker) = waker.take() {
                    // Safe because of Pin
                    unsafe {
                        *time_ptr = Some(t);
                    }

                    waker.wake();
                }
            }) as Box<dyn FnMut(JsValue)>);

            let handle = self
                .context
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .unwrap_throw();

            self.handle = Some(handle);

            // Hold on to callback to prevent it from being dropped prematurely.
            self.callback = Some(callback);
        }

        Poll::Pending
    }
}

impl Drop for RequestAnimationFrame {
    fn drop(&mut self) {
        // Only cancel if the animation frame hasn't already occurred
        if self.time.is_none() {
            if let Some(handle) = self.handle.take() {
                self.context.cancel_animation_frame(handle).unwrap_throw();
            }
        }
    }
}
