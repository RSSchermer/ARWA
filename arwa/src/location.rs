use wasm_bindgen::JsCast;

use crate::console::{Write, Writer};
use crate::error::{LocationAssignError, SecurityError, SyntaxError};

pub struct Location {
    inner: web_sys::Location,
}

impl Location {
    pub fn href(&self) -> Result<String, SecurityError> {
        self.inner.href().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    pub fn protocol(&self) -> Result<String, SecurityError> {
        self.inner.protocol().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    pub fn host(&self) -> Result<String, SecurityError> {
        self.inner.host().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    pub fn hostname(&self) -> Result<String, SecurityError> {
        self.inner.hostname().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    pub fn port(&self) -> Result<String, SecurityError> {
        self.inner.port().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    pub fn pathname(&self) -> Result<String, SecurityError> {
        self.inner.pathname().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    pub fn search(&self) -> Result<String, SecurityError> {
        self.inner.search().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    pub fn hash(&self) -> Result<String, SecurityError> {
        self.inner.hash().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    pub fn origin(&self) -> Result<String, SecurityError> {
        self.inner.origin().map_err(|e| {
            let e: web_sys::DomException = e.unchecked_into();

            SecurityError::new(e)
        })
    }

    // Forgoing setters for now for the above properties in favor of assign/replace, given the
    // awkwardness of the errors.

    // TODO: can we eliminate SyntaxErrors here by using a typed Url value that wraps web_sys::Url
    // (the parsing happens at Url creation, rather than here), with acceptable cost?

    pub fn assign(&self, url: &str) -> Result<(), LocationAssignError> {
        self.inner.assign(url).map_err(|err| {
            let err: web_sys::DomException = err.unchecked_into();

            match &*err.name() {
                "SecurityError" => SecurityError::new(err).into(),
                "SyntaxError" => SyntaxError::new(err.unchecked_into()).into(),
                _ => unreachable!(),
            }
        })
    }

    pub fn replace(&self, url: &str) -> Result<(), SyntaxError> {
        self.inner
            .assign(url)
            .map_err(|err| SyntaxError::new(err.unchecked_into()))
    }

    pub fn reload(&self) -> Result<(), SecurityError> {
        self.inner
            .reload()
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }

    pub fn reload_forced(&self) -> Result<(), SecurityError> {
        self.inner
            .reload_with_forceget(true)
            .map_err(|err| SecurityError::new(err.unchecked_into()))
    }
}

impl From<web_sys::Location> for Location {
    fn from(inner: web_sys::Location) -> Self {
        Location { inner }
    }
}

impl From<Location> for web_sys::Location {
    fn from(location: Location) -> Self {
        location.inner
    }
}

impl AsRef<web_sys::Location> for Location {
    fn as_ref(&self) -> &web_sys::Location {
        &self.inner
    }
}

impl Write for Location {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}
