use crate::fetch::RequestCredentials;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WorkerType {
    Classic,
    Module,
}

impl Default for WorkerType {
    fn default() -> Self {
        WorkerType::Classic
    }
}

pub struct WorkerOptions<'a> {
    pub worker_type: WorkerType,
    pub credentials: RequestCredentials,
    pub name: Option<&'a str>,
}

impl WorkerOptions {
    pub(crate) fn into_web_sys_worker_options(self) -> web_sys::WorkerOptions {
        let WorkerOptions {
            worker_type,
            credentials,
            name,
        } = self;

        let mut opts = web_sys::WorkerOptions;

        match worker_type {
            WorkerType::Classic => opts.type_(web_sys::WorkerType::Classic),
            WorkerType::Module => opts.type_(web_sys::WorkerType::Module),
        }

        match credentials {
            RequestCredentials::SameOrigin => {
                init.credentials(web_sys::RequestCredentials::SameOrigin)
            }
            RequestCredentials::Omit => init.credentials(web_sys::RequestCredentials::Omit),
            RequestCredentials::Include => init.credentials(web_sys::RequestCredentials::Include),
        }

        if let Some(name) = name {
            opts.name(name);
        }

        opts
    }
}

impl Default for WorkerOptions {
    fn default() -> Self {
        WorkerOptions {
            worker_type: WorkerType::Classic,
            credentials: RequestCredentials::Omit,
            name: None,
        }
    }
}
