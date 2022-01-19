use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::error::{LocationAssignError, SecurityError, SyntaxError};
use crate::security::SecurityError;
use url::Url;

pub struct WindowLocation {
    inner: web_sys::Location,
}

impl WindowLocation {
    // Prefer an interface that exposes getting the URL, then manipulate the URL object, then set
    // it back with `assign` or `replace`, over setting individual parts of the URL through the
    // location interface. Avoids many methods that would all have to deal with SecurityErrors and
    // avoids having to deal with syntax errors here.

    pub fn to_url(&self) -> Url {
        // Note: assuming Location.href is always a valid URL.
        self.inner
            .href()
            .map(|href| Url::parse(self.inner.href().as_ref()).unwrap())
            .map_err(|e| {
                let e: web_sys::DomException = e.unchecked_into();

                SecurityError::new(e)
            })
    }

    pub fn try_to_url(&self) -> Result<String, SecurityError> {
        // Note: assuming Location.href is always a valid URL.
        self.inner
            .href()
            .map(|href| Url::parse(self.inner.href().as_ref()).unwrap())
            .map_err(|e| {
                let e: web_sys::DomException = e.unchecked_into();

                SecurityError::new(e)
            })
    }

    pub fn assign(&self, url: &Url) {
        self.inner.assign(url).unwrap_throw()
    }

    pub fn try_assign(&self, url: &Url) -> Result<(), SecurityError> {
        self.inner.assign(url).map_err(|err| {
            let err: web_sys::DomException = err.unchecked_into();

            match &*err.name() {
                "SecurityError" => SecurityError::new(err).into(),
                _ => unreachable!(),
            }
        })
    }

    // TODO: the spec explicitly says that replace does not do security checks, whereas MDN implies
    // that it does. Figure out which is source is correct when it comes to actual browser behavior.

    pub fn replace(&self, url: &Url) {
        self.inner.assign(url).unwrap_throw()
    }

    pub fn reload(&self) {
        self.inner.reload().unwrap_throw()
    }

    pub fn try_reload(&self) -> Result<(), SecurityError> {
        self.inner
            .reload()
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    pub fn reload_forced(&self) {
        self.inner.reload_with_forceget(true).unwrap_throw()
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
