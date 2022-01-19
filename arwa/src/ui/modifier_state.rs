pub(crate) mod modifier_state_seal {
    pub trait Seal {}
}

pub trait ModifierState: modifier_state_seal::Seal {
    fn get_modifier_state(&self, key: &str) -> bool;

    fn alt_key(&self) -> bool;

    fn ctrl_key(&self) -> bool;

    fn shift_key(&self) -> bool;

    fn meta_key(&self) -> bool;
}
