use crate::fetch::RequestCredentials;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WorkerType {
    Classic,
    Module,
}

impl WorkerType {
    fn to_web_sys(&self) -> web_sys::WorkerType {
        match self {
            WorkerType::Classic => web_sys::WorkerType::Classic,
            WorkerType::Module => web_sys::WorkerType::Module,
        }
    }
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

impl WorkerOptions<'_> {
    pub(crate) fn into_web_sys_worker_options(self) -> web_sys::WorkerOptions {
        let WorkerOptions {
            worker_type,
            credentials,
            name,
        } = self;

        let mut opts = web_sys::WorkerOptions::new();

        opts.type_(worker_type.to_web_sys());
        opts.credentials(credentials.to_web_sys());

        if let Some(name) = name {
            opts.name(name);
        }

        opts
    }
}

impl Default for WorkerOptions<'static> {
    fn default() -> Self {
        WorkerOptions {
            worker_type: WorkerType::Classic,
            credentials: RequestCredentials::Omit,
            name: None,
        }
    }
}
