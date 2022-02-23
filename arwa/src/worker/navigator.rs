use wasm_bindgen::UnwrapThrowExt;

use crate::connection::{connection_status_seal, ConnectionStatus};
use crate::impl_common_wrapper_traits;
use crate::lang::LanguageTag;
use crate::navigator::{navigator_seal, Navigator};

#[derive(Clone)]
pub struct WorkerNavigator {
    inner: web_sys::WorkerNavigator,
}

impl navigator_seal::Seal for WorkerNavigator {}

impl Navigator for WorkerNavigator {
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
}

impl connection_status_seal::Seal for WorkerNavigator {}

impl ConnectionStatus for WorkerNavigator {
    fn online(&self) -> bool {
        self.inner.on_line()
    }
}

impl From<web_sys::WorkerNavigator> for WorkerNavigator {
    fn from(inner: web_sys::WorkerNavigator) -> Self {
        WorkerNavigator { inner }
    }
}

impl AsRef<web_sys::WorkerNavigator> for WorkerNavigator {
    fn as_ref(&self) -> &web_sys::WorkerNavigator {
        &self.inner
    }
}

impl_common_wrapper_traits!(WorkerNavigator);
