use crate::worker::{WorkerLocation, WorkerNavigator};

pub(crate) mod worker_global_scope_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_web_sys_worker_global_scope(&self) -> &web_sys::WorkerGlobalScope;
    }
}

pub trait WorkerGlobalScope: worker_global_scope_seal::Seal {
    fn navigator(&self) -> WorkerNavigator {
        WorkerNavigator::from(self.as_web_sys_worker_global_scope().navigator())
    }

    fn location(&self) -> WorkerLocation {
        WorkerLocation::from(self.as_web_sys_worker_global_scope().location())
    }
}

macro_rules! impl_worker_global_scope_traits {
    ($tpe:ident, $web_sys_tpe:ident) => {
        impl $crate::worker::worker_global_scope_seal::Seal for $tpe {
            fn as_web_sys_worker_global_scope(&self) -> &web_sys::WorkerGlobalScope {
                self.inner.as_ref()
            }
        }

        impl $crate::worker::WorkerGlobalScope for $tpe {}

        impl $crate::execution::execution_event_target_seal::Seal for $tpe {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                self.inner.as_ref()
            }
        }

        impl $crate::execution::ExecutionEventTarget for $tpe {}

        impl $crate::connection::connection_event_target_seal::Seal for $tpe {
            fn as_web_sys_event_target(&self) -> &web_sys::EventTarget {
                self.inner.as_ref()
            }
        }

        impl crate::connection::ConnectionEventTarget for $tpe {}

        impl $crate::timer::timer_context_seal::Seal for $tpe {}

        impl $crate::timer::TimerContext for $tpe {
            fn interval(&self, duration: $crate::timer::Duration) -> $crate::timer::Interval {
                $crate::timer::Interval::worker_context(
                    self.as_web_sys_worker_global_scope().clone(),
                    duration,
                )
            }

            fn timeout(&self, duration: $crate::timer::Duration) -> $crate::timer::Timeout {
                $crate::timer::Timeout::worker_context(
                    self.as_web_sys_worker_global_scope().clone(),
                    duration,
                )
            }
        }

        impl $crate::security::security_context_seal::Seal for $tpe {}

        impl $crate::security::SecurityContext for $tpe {
            fn is_secure_context(&self) -> bool {
                self.as_web_sys_worker_global_scope().is_secure_context()
            }

            fn origin(&self) -> String {
                self.as_web_sys_worker_global_scope().origin()
            }
        }

        impl $crate::fetch::fetch_context_seal::Seal for $tpe {}

        impl $crate::fetch::FetchContext for $tpe {
            fn fetch(&self, request: &$crate::fetch::Request) -> $crate::fetch::Fetch {
                Fetch::worker_context(
                    self.as_web_sys_worker_global_scope().clone(),
                    request.as_ref().clone(),
                )
            }
        }

        impl_event_target_traits!($tpe);
        impl_try_from_event_targets!($tpe, $web_sys_tpe);
    };
}

pub(crate) use impl_worker_global_scope_traits;
