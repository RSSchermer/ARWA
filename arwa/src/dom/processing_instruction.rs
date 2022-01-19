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

impl_character_data_traits!(ProcessingInstruction, web_sys::ProcessingInstruction);

#[derive(Clone)]
pub struct ProcessingInstructionError {
    inner: web_sys::DomException,
}

impl ProcessingInstructionError {
    pub(crate) fn new(inner: web_sys::DomException) -> Self {
        ProcessingInstructionError { inner }
    }
}
