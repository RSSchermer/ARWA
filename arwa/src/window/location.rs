use wasm_bindgen::{throw_val, JsCast, UnwrapThrowExt};

use crate::security::SecurityError;
use crate::url::Url;
use crate::{impl_common_wrapper_traits, impl_js_cast};

pub struct WindowLocation {
    inner: web_sys::Location,
}

impl WindowLocation {
    // Prefer an interface that exposes getting the URL, then manipulate the URL object, then set
    // it back with `assign` or `replace`, over setting individual parts of the URL through the
    // location interface. Avoids many methods that would all have to deal with SecurityErrors and
    // avoids having to deal with syntax errors here.

    pub fn to_url(&self) -> Url {
        // Location.href is always a valid URL.
        match Url::parse(self.inner.href().unwrap_throw().as_ref()) {
            Ok(url) => url,
            Err(err) => throw_val(err.into()),
        }
    }

    pub fn try_to_url(&self) -> Result<Url, SecurityError> {
        // Note: assuming Location.href is always a valid URL.
        self.inner
            .href()
            .map(|href| Url::parse(href.as_ref()).unwrap_throw())
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    pub fn assign(&self, url: &Url) {
        if let Err(err) = self.inner.assign(url.as_ref()) {
            throw_val(err)
        }
    }

    pub fn try_assign(&self, url: &Url) -> Result<(), SecurityError> {
        self.inner
            .assign(url.as_ref())
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    // TODO: the spec explicitly says that replace does not do security checks, whereas MDN implies
    // that it does. Figure out which is source is correct when it comes to actual browser behavior.

    pub fn replace(&self, url: &Url) {
        self.inner.assign(url.as_ref()).unwrap_throw();
    }

    pub fn reload(&self) {
        if let Err(err) = self.inner.reload() {
            throw_val(err)
        }
    }

    pub fn try_reload(&self) -> Result<(), SecurityError> {
        self.inner
            .reload()
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    pub fn reload_forced(&self) {
        if let Err(err) = self.inner.reload_with_forceget(true) {
            throw_val(err)
        }
    }

    pub fn try_reload_forced(&self) -> Result<(), SecurityError> {
        self.inner
            .reload_with_forceget(true)
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }
}

impl From<web_sys::Location> for WindowLocation {
    fn from(inner: web_sys::Location) -> Self {
        WindowLocation { inner }
    }
}

impl From<WindowLocation> for web_sys::Location {
    fn from(location: WindowLocation) -> Self {
        location.inner
    }
}

impl AsRef<web_sys::Location> for WindowLocation {
    fn as_ref(&self) -> &web_sys::Location {
        &self.inner
    }
}

impl_common_wrapper_traits!(WindowLocation);
impl_js_cast!(WindowLocation, Location);
