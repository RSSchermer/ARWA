use crate::console::{Write, Writer};

pub struct SubtleCrypto {
    inner: web_sys::SubtleCrypto,
}

// TODO: implement

impl From<web_sys::SubtleCrypto> for SubtleCrypto {
    fn from(inner: web_sys::SubtleCrypto) -> Self {
        SubtleCrypto { inner }
    }
}

impl AsRef<web_sys::SubtleCrypto> for SubtleCrypto {
    fn as_ref(&self) -> &web_sys::SubtleCrypto {
        &self.inner
    }
}

impl Write for SubtleCrypto {
    fn write(&self, writer: &mut Writer) {
        writer.write_1(self.inner.as_ref())
    }
}
