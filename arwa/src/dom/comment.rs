use crate::dom::impl_character_data_traits;

/// Represents a comment node in a document, that is typically not displayed to the user.
#[derive(Clone)]
pub struct Comment {
    inner: web_sys::Comment,
}

impl AsRef<web_sys::Comment> for Comment {
    fn as_ref(&self) -> &web_sys::Comment {
        &self.inner
    }
}

impl From<web_sys::Comment> for Comment {
    fn from(inner: web_sys::Comment) -> Self {
        Comment { inner }
    }
}

impl_character_data_traits!(Comment);
