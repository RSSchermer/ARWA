use std::async_iter::AsyncIterator;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::geolocation::{Position, PositionError};

struct CallbackState {
    next: Option<Result<Position, PositionError>>,
    waker: Option<Waker>,
}

impl CallbackState {
    fn uninitialized() -> Self {
        CallbackState {
            next: None,
            waker: None,
        }
    }
}

pub struct WatchPosition {
    geolocation: web_sys::Geolocation,
    options: Option<web_sys::PositionOptions>,
    success: Option<Closure<dyn FnMut(JsValue)>>,
    error: Option<Closure<dyn FnMut(JsValue)>>,
    watch_id: Option<i32>,
    callback_state: CallbackState,
}

impl WatchPosition {
    pub(crate) fn new(
        geolocation: web_sys::Geolocation,
        options: web_sys::PositionOptions,
    ) -> Self {
        WatchPosition {
            geolocation,
            options: Some(options),
            success: None,
            error: None,
            watch_id: None,
            callback_state: CallbackState::uninitialized(),
        }
    }
}

impl AsyncIterator for WatchPosition {
    type Item = Result<Position, PositionError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Initialize if we haven't already
        if let Some(options) = self.options.take() {
            let state_ptr = &mut self.callback_state as *mut CallbackState;

            let success = Closure::wrap(Box::new(move |value: JsValue| {
                // Safe because of Pin
                unsafe {
                    if let Some(waker) = (*state_ptr).waker.take() {
                        let position: web_sys::Position = value.unchecked_into();

                        (*state_ptr).next.replace(Ok(position.into()));
                        waker.wake();
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);

            let error = Closure::wrap(Box::new(move |err: JsValue| {
                // Safe because of Pin
                unsafe {
                    if let Some(waker) = (*state_ptr).waker.take() {
                        let err: web_sys::PositionError = err.unchecked_into();

                        (*state_ptr).next.replace(Err(PositionError::new(err)));
                        waker.wake();
                    }
                }
            }) as Box<dyn FnMut(JsValue)>);

            // No indication in the spec that this can fail, unwrap for now.
            let watch_id = self
                .geolocation
                .watch_position_with_error_callback_and_options(
                    success.as_ref().unchecked_ref(),
                    Some(error.as_ref().unchecked_ref()),
                    &options,
                )
                .unwrap_throw();

            self.watch_id = Some(watch_id);

            // Hang on to callbacks for the lifetime of the stream so they won't be dropped
            // while they may still get called.
            self.success = Some(success);
            self.error = Some(error);
        }

        // Refresh waker
        self.callback_state.waker = Some(cx.waker().clone());

        if let Some(current) = self.callback_state.next.take() {
            Poll::Ready(Some(current))
        } else {
            Poll::Pending
        }
    }
}

impl Drop for WatchPosition {
    fn drop(&mut self) {
        if let Some(watch_id) = self.watch_id {
            self.geolocation.clear_watch(watch_id);
        }
    }
}
