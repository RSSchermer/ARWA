use crate::impl_common_wrapper_traits;
use crate::url::Url;

#[derive(Clone)]
pub struct WorkerLocation {
    inner: web_sys::WorkerLocation,
}

impl WorkerLocation {
    pub fn to_url(&self) -> Option<Url> {
        Url::parse(self.inner.href().as_ref()).ok()
    }
}

impl From<web_sys::WorkerLocation> for WorkerLocation {
    fn from(inner: web_sys::WorkerLocation) -> Self {
        WorkerLocation { inner }
    }
}

impl AsRef<web_sys::WorkerLocation> for WorkerLocation {
    fn as_ref(&self) -> &web_sys::WorkerLocation {
        &self.inner
    }
}

impl_common_wrapper_traits!(WorkerLocation);
