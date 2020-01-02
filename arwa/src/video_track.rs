use delegate::delegate;

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
        target self.inner {
            pub fn id(&self) -> String;

            pub fn label(&self) -> String;

            pub fn language(&self) -> String;

            pub fn selected(&self) -> bool;

            pub fn set_selected(&self, selected: bool);
        }
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
