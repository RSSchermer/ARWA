use delegate::delegate;

use crate::impl_common_wrapper_traits;
use crate::lang::LanguageTag;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum VideoTrackKind {
    Alternative,
    Captions,
    Main,
    Sign,
    Subtitles,
    Commentary,
}

pub struct VideoTrack {
    inner: web_sys::VideoTrack,
}

impl VideoTrack {
    delegate! {
        to self.inner {
            pub fn id(&self) -> String;

            pub fn label(&self) -> String;

            pub fn selected(&self) -> bool;

            pub fn set_selected(&self, selected: bool);
        }
    }

    pub fn language(&self) -> Option<LanguageTag> {
        LanguageTag::parse(self.inner.language().as_ref()).ok()
    }

    pub fn kind(&self) -> Option<VideoTrackKind> {
        match &*self.inner.kind() {
            "alternative" => Some(VideoTrackKind::Alternative),
            "captions" => Some(VideoTrackKind::Captions),
            "main" => Some(VideoTrackKind::Main),
            "sign" => Some(VideoTrackKind::Sign),
            "subtitles" => Some(VideoTrackKind::Subtitles),
            "commentary" => Some(VideoTrackKind::Commentary),
            _ => None,
        }
    }
}

impl From<web_sys::VideoTrack> for VideoTrack {
    fn from(inner: web_sys::VideoTrack) -> Self {
        VideoTrack { inner }
    }
}

impl AsRef<web_sys::VideoTrack> for VideoTrack {
    fn as_ref(&self) -> &web_sys::VideoTrack {
        &self.inner
    }
}

impl_common_wrapper_traits!(VideoTrack);
