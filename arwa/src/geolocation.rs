use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use delegate::delegate;
use futures::Stream;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::error::PositionError;

pub use web_sys::PositionOptions;

pub struct Geolocation {
    inner: web_sys::Geolocation,
}

impl Geolocation {
    pub fn current_position(&self, options: PositionOptions) -> CurrentPosition {
        CurrentPosition {
            geolocation: self.inner.clone(),
            options: Some(options),
            success: None,
            error: None,
            ready: Rc::new(RefCell::new(None)),
        }
    }

    pub fn watch_position(&self, options: PositionOptions) -> WatchPosition {
        WatchPosition {
            geolocation: self.inner.clone(),
            options: Some(options),
            success: None,
            error: None,
            watch_id: None,
            next: Rc::new(RefCell::new(None)),
        }
    }
}

impl From<web_sys::Geolocation> for Geolocation {
    fn from(inner: web_sys::Geolocation) -> Self {
        Geolocation { inner }
    }
}

impl AsRef<web_sys::Geolocation> for Geolocation {
    fn as_ref(&self) -> &web_sys::Geolocation {
        &self.inner
    }
}

pub struct CurrentPosition {
    geolocation: web_sys::Geolocation,
    options: Option<PositionOptions>,
    success: Option<Closure<dyn FnMut(JsValue)>>,
    error: Option<Closure<dyn FnMut(JsValue)>>,
    ready: Rc<RefCell<Option<Result<Position, PositionError>>>>,
}

impl Future for CurrentPosition {
    type Output = Result<Position, PositionError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let current_position = self.get_mut();

        if let Some(current) = current_position.ready.borrow_mut().take() {
            Poll::Ready(current)
        } else {
            if let Some(options) = current_position.options.take() {
                // Initialize
                let ready = current_position.ready.clone();
                let waker = cx.waker().clone();

                let success = Closure::wrap(Box::new(move |value: JsValue| {
                    let position: web_sys::Position = value.unchecked_into();

                    ready.borrow_mut().replace(Ok(position.into()));
                    waker.wake_by_ref();
                }) as Box<dyn FnMut(JsValue)>);

                let ready = current_position.ready.clone();
                let waker = cx.waker().clone();

                let error = Closure::wrap(Box::new(move |err: JsValue| {
                    let err: web_sys::PositionError = err.unchecked_into();

                    ready.borrow_mut().replace(Err(PositionError::new(err)));
                    waker.wake_by_ref();
                }) as Box<dyn FnMut(JsValue)>);

                // No indication in the spec that this can fail, unwrap for now.
                current_position
                    .geolocation
                    .get_current_position_with_error_callback_and_options(
                        success.as_ref().unchecked_ref(),
                        Some(error.as_ref().unchecked_ref()),
                        &options,
                    )
                    .unwrap();

                // Hang on to callbacks for the lifetime of the stream so they won't be dropped
                // while they may still get called.
                current_position.success = Some(success);
                current_position.error = Some(error);
            }

            Poll::Pending
        }
    }
}

pub struct WatchPosition {
    geolocation: web_sys::Geolocation,
    options: Option<PositionOptions>,
    success: Option<Closure<dyn FnMut(JsValue)>>,
    error: Option<Closure<dyn FnMut(JsValue)>>,
    watch_id: Option<i32>,
    next: Rc<RefCell<Option<Result<Position, PositionError>>>>,
}

impl Stream for WatchPosition {
    type Item = Result<Position, PositionError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let watch_position = self.get_mut();

        if let Some(current) = watch_position.next.borrow_mut().take() {
            Poll::Ready(Some(current))
        } else {
            if let Some(options) = watch_position.options.take() {
                // Initialize
                let next = watch_position.next.clone();
                let waker = cx.waker().clone();

                let success = Closure::wrap(Box::new(move |value: JsValue| {
                    let position: web_sys::Position = value.unchecked_into();

                    next.borrow_mut().replace(Ok(position.into()));
                    waker.wake_by_ref();
                }) as Box<dyn FnMut(JsValue)>);

                let next = watch_position.next.clone();
                let waker = cx.waker().clone();

                let error = Closure::wrap(Box::new(move |err: JsValue| {
                    let err: web_sys::PositionError = err.unchecked_into();

                    next.borrow_mut().replace(Err(PositionError::new(err)));
                    waker.wake_by_ref();
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

pub struct Position {
    inner: web_sys::Position,
}

impl Position {
    pub fn coordinates(&self) -> Coordinates {
        self.inner.coords().into()
    }

    pub fn time_stamp(&self) -> u64 {
        self.inner.timestamp() as u64
    }
}

impl From<web_sys::Position> for Position {
    fn from(inner: web_sys::Position) -> Self {
        Position { inner }
    }
}

impl AsRef<web_sys::Position> for Position {
    fn as_ref(&self) -> &web_sys::Position {
        &self.inner
    }
}

pub struct Coordinates {
    inner: web_sys::Coordinates,
}

impl Coordinates {
    delegate! {
        target self.inner {
            pub fn latitude(&self) -> f64;

            pub fn longitude(&self) -> f64;

            pub fn altitude(&self) -> Option<f64>;

            pub fn accuracy(&self) -> f64;

            pub fn altitude_accuracy(&self) -> Option<f64>;

            pub fn heading(&self) -> Option<f64>;

            pub fn speed(&self) -> Option<f64>;
        }
    }
}

impl From<web_sys::Coordinates> for Coordinates {
    fn from(inner: web_sys::Coordinates) -> Self {
        Coordinates { inner }
    }
}

impl AsRef<web_sys::Coordinates> for Coordinates {
    fn as_ref(&self) -> &web_sys::Coordinates {
        &self.inner
    }
}
