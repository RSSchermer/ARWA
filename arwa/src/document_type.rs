use delegate::delegate;

use crate::console::{Write, Writer};
use crate::Node;

pub struct DocumentType {
    inner: web_sys::DocumentType,
}

impl DocumentType {
    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn public_id(&self) -> String;

            pub fn system_id(&self) -> String;
        }
    }
}

impl From<web_sys::DocumentType> for DocumentType {
    fn from(inner: web_sys::DocumentType) -> Self {
        DocumentType { inner }
    }
}

impl AsRef<web_sys::DocumentType> for DocumentType {
    fn as_ref(&self) -> &web_sys::DocumentType {
        &self.inner
    }
}

impl AsRef<web_sys::Node> for DocumentType {
    fn as_ref(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}

impl Write for DocumentType {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl Node for DocumentType {}
