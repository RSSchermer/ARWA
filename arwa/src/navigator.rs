use web_sys::ServiceWorkerContainer;

pub(crate) mod navigator_seal {
    pub trait Seal {}
}

pub trait Navigator: navigator_seal::Seal {
    fn hardware_concurrency(&self) -> u32;

    fn language(&self) -> Option<String>;

    fn user_agent(&self) -> String;

    fn service_worker(&self) -> ServiceWorkerContainer;

    // TODO: serial

    // TODO: user_agent_data
}
