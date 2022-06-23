use delegate::delegate;

use crate::lang::LanguageTag;
use crate::{impl_common_wrapper_traits, impl_js_cast};

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
        to self.inner {
            pub fn id(&self) -> String;

            pub fn label(&self) -> String;

            pub fn enabled(&self) -> bool;

            pub fn set_enabled(&self, enabled: bool);
        }
    }

    pub fn language(&self) -> Option<LanguageTag> {
        LanguageTag::parse(self.inner.language().as_ref()).ok()
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

impl_common_wrapper_traits!(AudioTrack);
impl_js_cast!(AudioTrack);
