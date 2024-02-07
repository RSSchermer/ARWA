use js_sys::Uint32Array;
use wasm_bindgen::{throw_val, JsCast, UnwrapThrowExt};

use crate::connection::{connection_status_seal, ConnectionStatus};
use crate::geolocation::Geolocation;
use crate::lang::LanguageTag;
use crate::navigator::{navigator_seal, Navigator};
use crate::storage_manager::StorageManager;
use crate::worker::service::ServiceWorkerContainer;
use crate::{dom_exception_wrapper, impl_common_wrapper_traits, impl_js_cast};

pub struct ProtocolHandler<'a> {
    pub scheme: &'a str,
    pub url: &'a str,
}

dom_exception_wrapper!(RegisterProtocolHandlerError);

pub struct WindowNavigator {
    inner: web_sys::Navigator,
}

impl WindowNavigator {
    pub fn geolocation(&self) -> Option<Geolocation> {
        self.inner.geolocation().ok().map(|g| g.into())
    }

    pub fn max_touch_points(&self) -> u32 {
        self.inner.max_touch_points() as u32
    }

    pub fn register_protocol_handler(&self, protocol_handler: ProtocolHandler) {
        let ProtocolHandler { scheme, url } = protocol_handler;

        if let Err(err) = self.inner.register_protocol_handler(scheme, url, "") {
            throw_val(err)
        }
    }

    pub fn try_register_protocol_handler(
        &self,
        protocol_handler: ProtocolHandler,
    ) -> Result<(), RegisterProtocolHandlerError> {
        let ProtocolHandler { scheme, url } = protocol_handler;

        self.inner
            .register_protocol_handler(scheme, url, "")
            .map_err(|err| RegisterProtocolHandlerError::new(err.unchecked_into()))
    }

    pub fn service_worker(&self) -> ServiceWorkerContainer {
        self.inner.service_worker().into()
    }

    pub fn vibrate<T>(&self, pattern: T) -> bool
    where
        T: VibrationPattern,
    {
        pattern.vibrate(self)
    }
}

impl navigator_seal::Seal for WindowNavigator {}

impl Navigator for WindowNavigator {
    fn language(&self) -> Option<LanguageTag> {
        self.inner
            .language()
            .and_then(|l| LanguageTag::parse(l.as_ref()).ok())
    }

    fn hardware_concurrency(&self) -> u32 {
        self.inner.hardware_concurrency() as u32
    }

    fn user_agent(&self) -> String {
        self.inner.user_agent().unwrap_throw()
    }

    fn storage(&self) -> StorageManager {
        StorageManager::from(self.inner.storage())
    }
}

impl connection_status_seal::Seal for WindowNavigator {}

impl ConnectionStatus for WindowNavigator {
    fn online(&self) -> bool {
        self.inner.on_line()
    }
}

impl From<web_sys::Navigator> for WindowNavigator {
    fn from(inner: web_sys::Navigator) -> Self {
        WindowNavigator { inner }
    }
}

impl AsRef<web_sys::Navigator> for WindowNavigator {
    fn as_ref(&self) -> &web_sys::Navigator {
        &self.inner
    }
}

impl_common_wrapper_traits!(WindowNavigator);
impl_js_cast!(WindowNavigator, Navigator);

pub trait VibrationPattern: vibration_pattern_seal::Seal {
    fn vibrate(self, navigator: &WindowNavigator) -> bool;
}

mod vibration_pattern_seal {
    pub trait Seal {}

    impl Seal for u32 {}
    impl Seal for &'_ [u32] {}
}

impl VibrationPattern for u32 {
    fn vibrate(self, navigator: &WindowNavigator) -> bool {
        let navigator: &web_sys::Navigator = navigator.as_ref();

        navigator.vibrate_with_duration(self)
    }
}

impl VibrationPattern for &'_ [u32] {
    fn vibrate(self, navigator: &WindowNavigator) -> bool {
        let navigator: &web_sys::Navigator = navigator.as_ref();

        unsafe {
            let view = Uint32Array::view(self);

            navigator.vibrate_with_pattern(view.as_ref())
        }
    }
}
