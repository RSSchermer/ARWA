// TODO: implement
pub struct CssStyleDeclaration {
    inner: web_sys::CssStyleDeclaration,
}

impl From<web_sys::CssStyleDeclaration> for CssStyleDeclaration {
    fn from(inner: web_sys::CssStyleDeclaration) -> Self {
        CssStyleDeclaration { inner }
    }
}

impl AsRef<web_sys::CssStyleDeclaration> for CssStyleDeclaration {
    fn as_ref(&self) -> &web_sys::CssStyleDeclaration {
        &self.inner
    }
}
