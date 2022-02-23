use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

use crate::geolocation::{Position, PositionError};

struct CallbackState {
    result: Option<Result<Position, PositionError>>,
    waker: Option<Waker>,
}

impl CallbackState {
    fn uninitialized() -> Self {
        CallbackState {
            result: None,
            waker: None,
        }
    }
}

pub struct CurrentPosition {
    geolocation: web_sys::Geolocation,
    options: Option<web_sys::PositionOptions>,
    success: Option<Closure<dyn FnMut(JsValue)>>,
    error: Option<Closure<dyn FnMut(JsValue)>>,
    callback_state: CallbackState,
}

impl CurrentPosition {
    pub(crate) fn new(
        geolocation: web_sys::Geolocation,
        options: web_sys::PositionOptions,
    ) -> Self {
        CurrentPosition {
            geolocation,
            options: Some(options),
            success: None,
            error: None,
            callback_state: CallbackState::uninitialized(),
        }
    }
}

impl Future for CurrentPosition {
    type Output = Result<Position, PositionError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(current) = self.callback_state.result.take() {
            Poll::Ready(current)
        } else {
            // Initialize if we haven't already
            if let Some(options) = self.options.take() {
                let state_ptr = &mut self.callback_state as *mut CallbackState;

                let success = Closure::wrap(Box::new(move |value: JsValue| {
                    // Safe because of Pin
                    unsafe {
                        if let Some(waker) = (*state_ptr).waker.take() {
                            let position: web_sys::Position = value.unchecked_into();

                            (*state_ptr).result.replace(Ok(position.into()));
                            waker.wake();
                        }
                    }
                }) as Box<dyn FnMut(JsValue)>);

                let error = Closure::wrap(Box::new(move |err: JsValue| {
                    // Safe because of Pin
                    unsafe {
                        if let Some(waker) = (*state_ptr).waker.take() {
                            let err: web_sys::PositionError = err.unchecked_into();

                            (*state_ptr).result.replace(Err(PositionError::new(err)));
                            waker.wake();
                        }
                    }
                }) as Box<dyn FnMut(JsValue)>);

                self.callback_state.waker = Some(cx.waker().clone());

                // No indication in the spec that this can fail, unwrap for now.
                self.geolocation
                    .get_current_position_with_error_callback_and_options(
                        success.as_ref().unchecked_ref(),
                        Some(error.as_ref().unchecked_ref()),
                        &options,
                    )
                    .unwrap_throw();

                // Hang on to callbacks for the lifetime of the stream so they won't be dropped
                // while they may still get called.
                self.success = Some(success);
                self.error = Some(error);
            }

            Poll::Pending
        }
    }
}
