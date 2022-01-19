use crate::geolocation::{Position, PositionError};
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

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

impl Stream for WatchPosition {
    type Item = Result<Position, PositionError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let watch_position = self.get_mut();

        // Initialize if we haven't already
        if let Some(options) = watch_position.options.take() {
            let state_ptr = &mut current_position.callback_state as *mut CallbackState;

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
            let watch_id = watch_position
                .geolocation
                .watch_position_with_error_callback_and_options(
                    success.as_ref().unchecked_ref(),
                    Some(error.as_ref().unchecked_ref()),
                    &options,
                )
                .unwrap();

            watch_position.watch_id = Some(watch_id);

            // Hang on to callbacks for the lifetime of the stream so they won't be dropped
            // while they may still get called.
            watch_position.success = Some(success);
            watch_position.error = Some(error);
        }

        // Refresh waker
        self.callback_state.waker = Some(cx.waker().clone());

        if let Some(current) = watch_position.callback_state.next.take() {
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
