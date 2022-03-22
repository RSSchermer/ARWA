use delegate::delegate;

use crate::dom::impl_character_data_traits;
use crate::dom_exception_wrapper;

#[derive(Clone)]
pub struct ProcessingInstruction {
    inner: web_sys::ProcessingInstruction,
}

impl ProcessingInstruction {
    delegate! {
        to self.inner {
            pub fn target(&self) -> String;
        }
    }
}

impl AsRef<web_sys::ProcessingInstruction> for ProcessingInstruction {
    fn as_ref(&self) -> &web_sys::ProcessingInstruction {
        &self.inner
    }
}

impl From<web_sys::ProcessingInstruction> for ProcessingInstruction {
    fn from(inner: web_sys::ProcessingInstruction) -> Self {
        ProcessingInstruction { inner }
    }
}

impl_character_data_traits!(ProcessingInstruction);

dom_exception_wrapper!(ProcessingInstructionError);
