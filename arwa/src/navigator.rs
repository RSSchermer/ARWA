use delegate::delegate;
use js_sys::Uint32Array;
use wasm_bindgen::JsCast;

use crate::error::{RegisterProtocolHandlerError, SecurityError, SyntaxError};
use crate::Geolocation;

pub struct ProtocolHandler<'a, 'b, 'c> {
    pub scheme: &'a str,
    pub url: &'b str,
    pub title: &'c str,
}

pub struct Navigator {
    inner: web_sys::Navigator,
}

impl Navigator {
    delegate! {
        target self.inner {
            pub fn language(&self) -> Option<String>;
        }
    }

    pub fn geolocation(&self) -> Option<Geolocation> {
        self.inner.geolocation().ok().map(|g| g.into())
    }

    pub fn hardware_concurrency(&self) -> u32 {
        self.inner.hardware_concurrency() as u32
    }

    pub fn max_touch_points(&self) -> u32 {
        self.inner.max_touch_points() as u32
    }

    pub fn online(&self) -> bool {
        self.inner.on_line()
    }

    pub fn user_agent(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.user_agent().unwrap()
    }

    pub fn register_protocol_handler(
        &self,
        protocol_handler: ProtocolHandler,
    ) -> Result<(), RegisterProtocolHandlerError> {
        let ProtocolHandler { scheme, url, title } = protocol_handler;

        self.inner
            .register_protocol_handler(scheme, url, title)
            .map_err(|err| match err.dyn_into::<js_sys::SyntaxError>() {
                Ok(syntax_error) => SyntaxError::new(syntax_error).into(),
                Err(err) => {
                    let err: web_sys::DomException = err.unchecked_into();

                    SecurityError::new(err).into()
                }
            })
    }

    pub fn vibrate<T>(&self, pattern: T) -> bool
    where
        T: VibrationPattern,
    {
        pattern.vibrate(self)
    }
}

impl From<web_sys::Navigator> for Navigator {
    fn from(inner: web_sys::Navigator) -> Self {
        Navigator { inner }
    }
}

impl AsRef<web_sys::Navigator> for Navigator {
    fn as_ref(&self) -> &web_sys::Navigator {
        &self.inner
    }
}

pub trait VibrationPattern: vibration_pattern_seal::Seal {
    fn vibrate(self, navigator: &Navigator) -> bool;
}

mod vibration_pattern_seal {
    pub trait Seal {}

    impl Seal for u32 {}
    impl Seal for &'_ [u32] {}
}

impl VibrationPattern for u32 {
    fn vibrate(self, navigator: &Navigator) -> bool {
        let navigator: &web_sys::Navigator = navigator.as_ref();

        navigator.vibrate_with_duration(self)
    }
}

impl VibrationPattern for &'_ [u32] {
    fn vibrate(self, navigator: &Navigator) -> bool {
        let navigator: &web_sys::Navigator = navigator.as_ref();

        unsafe {
            let view = Uint32Array::view(self);

            navigator.vibrate_with_pattern(view.as_ref())
        }
    }
}
