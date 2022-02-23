use crate::lang::LanguageTag;

pub(crate) mod navigator_seal {
    pub trait Seal {}
}

pub trait Navigator: navigator_seal::Seal {
    fn hardware_concurrency(&self) -> u32;

    fn language(&self) -> Option<LanguageTag>;

    fn user_agent(&self) -> String;

    // TODO: serial

    // TODO: user_agent_data
}
