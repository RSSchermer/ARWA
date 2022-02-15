use arwa::console::{Write, Writer};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextTrackReadyState {
    None,
    Loading,
    Loaded,
    Error,
}

// TODO: implement
pub struct TextTrack {
    inner: web_sys::TextTrack,
}

impl From<web_sys::TextTrack> for TextTrack {
    fn from(inner: web_sys::TextTrack) -> Self {
        TextTrack { inner }
    }
}

impl AsRef<web_sys::TextTrack> for TextTrack {
    fn as_ref(&self) -> &web_sys::TextTrack {
        &self.inner
    }
}

impl_common_wrapper_traits!(TextTrack, web_sys::TextTrack);
