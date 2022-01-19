use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone)]
pub struct Headers {
    inner: web_sys::Headers,
}

impl Headers {
    pub fn new() -> Self {
        Headers::from(web_sys::Headers::new().unwrap_throw())
    }

    pub fn contains_key(&self, header_name: &str) -> bool {
        self.inner.has(header_name).unwrap_or(false)
    }

    pub fn get(&self, header_name: &str) -> Option<String> {
        self.inner.get(header_name).ok().flatten()
    }

    pub fn set(&self, header_name: &str, value: &str) {
        self.inner.set(header_name, value).unwrap_throw()
    }

    pub fn try_set(&self, header_name: &str, value: &str) -> Result<(), InvalidHeaderName> {
        self.inner
            .set(header_name, value)
            .map_err(|err| InvalidHeaderName::new(err.unchecked_into()))
    }

    pub fn append(&self, header_name: &str, value: &str) {
        self.inner.append(header_name, value).unwrap_throw()
    }

    pub fn try_append(&self, header_name: &str, value: &str) -> Result<(), InvalidHeaderName> {
        self.inner
            .append(header_name, value)
            .map_err(|err| InvalidHeaderName::new(err.unchecked_into()))
    }

    pub fn remove(&self, header_name: &str) {
        self.inner.delete(header_name).unwrap_throw()
    }

    pub fn try_remove(&self, header_name: &str, value: &str) -> Result<(), InvalidHeaderName> {
        self.inner
            .delete(header_name)
            .map_err(|err| InvalidHeaderName::new(err.unchecked_into()))
    }

    // TODO: iterators currently not in web-sys
}

impl From<web_sys::Headers> for Headers {
    fn from(inner: web_sys::Headers) -> Self {
        Headers { inner }
    }
}

impl AsRef<web_sys::Headers> for Headers {
    fn as_ref(&self) -> &web_sys::Headers {
        &self.inner
    }
}

impl_common_wrapper_traits!(Headers);

#[derive(Clone)]
pub struct InvalidHeaderName {
    inner: web_sys::TypeError,
}

impl InvalidHeaderName {
    fn new(inner: web_sys::TypeError) -> Self {
        InvalidHeaderName { inner }
    }
}
