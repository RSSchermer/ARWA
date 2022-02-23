use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::security::SecurityError;
use crate::url::{AbsoluteOrRelativeUrl, Url};

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
        Url::parse(self.inner.href().unwrap_throw().as_ref()).unwrap_throw()
    }

    pub fn try_to_url(&self) -> Result<Url, SecurityError> {
        // Note: assuming Location.href is always a valid URL.
        self.inner
            .href()
            .map(|href| Url::parse(href.as_ref()).unwrap_throw())
            .map_err(|e| SecurityError::new(e.unchecked_into()))
    }

    pub fn assign<T>(&self, url: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.assign(url.as_str()).unwrap_throw()
    }

    pub fn try_assign<T>(&self, url: T) -> Result<(), SecurityError>
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner
            .assign(url.as_str())
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    // TODO: the spec explicitly says that replace does not do security checks, whereas MDN implies
    // that it does. Figure out which is source is correct when it comes to actual browser behavior.

    pub fn replace<T>(&self, url: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.assign(url.as_str()).unwrap_throw();
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
