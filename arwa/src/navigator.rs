use crate::lang::LanguageTag;
use crate::storage_manager::StorageManager;

pub(crate) mod navigator_seal {
    pub trait Seal {}
}

pub trait Navigator: navigator_seal::Seal {
    fn hardware_concurrency(&self) -> u32;

    fn language(&self) -> Option<LanguageTag>;

    fn user_agent(&self) -> String;

    fn storage(&self) -> StorageManager;

    // TODO: serial

    // TODO: user_agent_data
}
