use delegate::delegate;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AudioTrackKind {
    Alternative,
    Descriptions,
    Main,
    MainDescriptions,
    Translation,
    Commentary,
}

pub struct AudioTrack {
    inner: web_sys::AudioTrack,
}

impl AudioTrack {
    delegate! {
        target self.inner {
            pub fn id(&self) -> String;

            pub fn label(&self) -> String;

            pub fn language(&self) -> String;

            pub fn enabled(&self) -> bool;

            pub fn set_enabled(&self, enabled: bool);
        }
    }

    pub fn kind(&self) -> Option<AudioTrackKind> {
        match &*self.inner.kind() {
            "alternative" => Some(AudioTrackKind::Alternative),
            "descriptions" => Some(AudioTrackKind::Descriptions),
            "main" => Some(AudioTrackKind::Main),
            "main-desc" => Some(AudioTrackKind::MainDescriptions),
            "translation" => Some(AudioTrackKind::Translation),
            "commentary" => Some(AudioTrackKind::Commentary),
            _ => None,
        }
    }
}

impl From<web_sys::AudioTrack> for AudioTrack {
    fn from(inner: web_sys::AudioTrack) -> Self {
        AudioTrack { inner }
    }
}

impl AsRef<web_sys::AudioTrack> for AudioTrack {
    fn as_ref(&self) -> &web_sys::AudioTrack {
        &self.inner
    }
}
