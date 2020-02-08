use delegate::delegate;

use crate::console::{Write, Writer};
use crate::Node;

pub struct Attribute {
    inner: web_sys::Attr,
}

impl Attribute {
    pub(crate) fn new(inner: web_sys::Attr) -> Self {
        Attribute { inner }
    }

    delegate! {
        target self.inner {
            pub fn name(&self) -> String;

            pub fn namespace_uri(&self) -> Option<String>;

            pub fn local_name(&self) -> String;

            pub fn prefix(&self) -> Option<String>;

            pub fn value(&self) -> String;

            pub fn set_value(&self, value: &str);
        }
    }
}

impl AsRef<web_sys::Attr> for Attribute {
    fn as_ref(&self) -> &web_sys::Attr {
        &self.inner
    }
}

impl AsRef<web_sys::Node> for Attribute {
    fn as_ref(&self) -> &web_sys::Node {
        self.inner.as_ref()
    }
}

impl Write for Attribute {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}

impl Node for Attribute {}
