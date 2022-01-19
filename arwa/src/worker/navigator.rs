use crate::connection::{
    connection_event_target_seal, connection_status_seal, ConnectionEventTarget, ConnectionStatus,
};
use crate::navigator::{navigator_seal, Navigator};

#[derive(Clone)]
pub struct WorkerNavigator {
    inner: web_sys::WorkerNavigator,
}

impl navigator_seal::Seal for WorkerNavigator {}

impl Navigator for WorkerNavigator {
    delegate! {
        to self.inner {
            fn language(&self) -> Option<String>;
        }
    }

    fn hardware_concurrency(&self) -> u32 {
        self.inner.hardware_concurrency() as u32
    }

    fn user_agent(&self) -> String {
        self.inner.user_agent().unwrap()
    }
}

impl connection_status_seal::Seal for WorkerNavigator {}

impl ConnectionStatus for WorkerNavigator {
    fn online(&self) -> bool {
        self.inner.online()
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
