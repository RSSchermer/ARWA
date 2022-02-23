use std::convert::TryFrom;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use delegate::delegate;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::dom::impl_try_from_element;
use crate::file::Blob;
use crate::html::{impl_html_element_traits, impl_known_element};
use crate::security::SecurityError;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct InvalidImageQuality(f64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ImageQuality(f64);

impl TryFrom<f64> for ImageQuality {
    type Error = InvalidImageQuality;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < 0.0 || value > 1.0 {
            Err(InvalidImageQuality(value))
        } else {
            Ok(ImageQuality(value))
        }
    }
}

impl Default for ImageQuality {
    fn default() -> Self {
        ImageQuality(0.92)
    }
}

impl From<ImageQuality> for f64 {
    fn from(image_quality: ImageQuality) -> Self {
        image_quality.0
    }
}

#[derive(Clone)]
pub struct HtmlCanvasElement {
    inner: web_sys::HtmlCanvasElement,
}

impl HtmlCanvasElement {
    delegate! {
        target self.inner {
            pub fn width(&self) -> u32;

            pub fn set_width(&self, width: u32);

            pub fn height(&self) -> u32;

            pub fn set_height(&self, height: u32);
        }
    }

    // TODO: it's unclear from the WHATWG spec if there can also be an encoding error, or if that
    // would only result in an empty string, needs to be experimented with.
    pub fn to_data_url(
        &self,
        media_type: &str,
        quality: ImageQuality,
    ) -> Result<String, SecurityError> {
        let quality: f64 = quality.into();

        self.inner
            .to_data_url_with_type_and_encoder_options(media_type, &quality.into())
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    pub fn to_blob(&self, media_type: &str, quality: ImageQuality) -> CanvasToBlob {
        CanvasToBlob {
            init: Some(CanvasToBlobInit {
                canvas_element: self.inner.clone(),
                media_type: media_type.to_string(),
                quality,
            }),
            result: None,
        }
    }

    // TODO: figure our what to do about obtaining contexts. Leave that to external crates?
}

impl From<web_sys::HtmlCanvasElement> for HtmlCanvasElement {
    fn from(inner: web_sys::HtmlCanvasElement) -> Self {
        HtmlCanvasElement { inner }
    }
}

impl AsRef<web_sys::HtmlCanvasElement> for HtmlCanvasElement {
    fn as_ref(&self) -> &web_sys::HtmlCanvasElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlCanvasElement);
impl_try_from_element!(HtmlCanvasElement);
impl_known_element!(HtmlCanvasElement, "CANVAS");

struct CanvasToBlobInit {
    canvas_element: web_sys::HtmlCanvasElement,
    media_type: String,
    quality: ImageQuality,
}

#[must_use = "futures do nothing unless polled or spawned"]
pub struct CanvasToBlob {
    init: Option<CanvasToBlobInit>,
    result: Option<Option<Blob>>,
}

impl Future for CanvasToBlob {
    type Output = Result<Option<Blob>, SecurityError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(init) = self.init.take() {
            let CanvasToBlobInit {
                canvas_element,
                media_type,
                quality,
            } = init;

            let mut rc = Rc::new(None);
            let mut rc_clone = rc.clone();

            let mut waker = Some(cx.waker().clone());
            let ptr = &mut self.result as *mut Option<Option<Blob>>;

            let closure = Closure::wrap(Box::new(move |value: JsValue| {
                if let Some(waker) = waker.take() {
                    let res = if value.is_null() {
                        None
                    } else {
                        Some(Blob::from(value.unchecked_into::<web_sys::Blob>()))
                    };

                    // Safe because of Pin
                    unsafe {
                        *ptr = Some(res);
                    }

                    waker.wake();

                    // We know this is the only remaining reference, so this is safe
                    let option_closure = unsafe { Rc::get_mut_unchecked(&mut rc_clone).take() };

                    mem::drop(option_closure);
                }
            }) as Box<dyn FnMut(JsValue)>);

            let quality: f64 = quality.into();

            match canvas_element.to_blob_with_type_and_encoder_options(
                closure.as_ref().unchecked_ref(),
                &media_type,
                &quality.into(),
            ) {
                Ok(_) => {
                    // Make sure the closure doesn't drop until it gets called once. In the `Ok`
                    // case, it should be guaranteed that the host will call the closure.

                    // There is exactly one other reference, and we know it won't get dereferenced
                    // at any time in the current task, so this is safe
                    unsafe {
                        *Rc::get_mut_unchecked(&mut rc) = Some(closure);
                    }
                }
                Err(err) => {
                    // Just let the closure drop immediately and return the error.

                    return Poll::Ready(Err(SecurityError::new(err.unchecked_into())));
                }
            }

            mem::drop(rc);
        }

        if let Some(result) = self.result.take() {
            Poll::Ready(Ok(result))
        } else {
            Poll::Pending
        }
    }
}
