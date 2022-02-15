use crate::lang::LanguageTag;
use crate::media::{TextTrack, TextTrackReadyState};
use crate::url::{AbsoluteOrRelativeUrl, Url};

#[derive(Clone)]
pub struct HtmlTrackElement {
    inner: web_sys::HtmlTrackElement,
}

impl HtmlTrackElement {
    delegate! {
        target self.inner {
            pub fn label(&self) -> String;

            pub fn set_label(&self, label: &str);

            pub fn default(&self) -> bool;

            pub fn set_default(&self, default: bool);
        }
    }

    pub fn src(&self) -> Option<Url> {
        Url::parse(self.inner.src()).ok()
    }

    pub fn set_src<T>(&self, src: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_src(src.as_str());
    }

    pub fn src_lang(&self) -> Option<LanguageTag> {
        LanguageTag::parse(self.inner.srclang()).ok()
    }

    pub fn set_src_lang(&self, src_lang: Option<&LanguageTag>) {
        self.inner
            .set_srclang(src_lang.map(|l| l.as_ref()).unwrap_or(""))
    }

    pub fn ready_state(&self) -> TextTrackReadyState {
        match self.inner.ready_state() {
            0 => TextTrackReadyState::None,
            1 => TextTrackReadyState::Loading,
            2 => TextTrackReadyState::Loaded,
            3 => TextTrackReadyState::Error,
            _ => unreachable!(),
        }
    }

    pub fn track(&self) -> Option<TextTrack> {
        self.inner.track().map(|t| t.into())
    }
}

impl From<web_sys::HtmlTrackElement> for HtmlTrackElement {
    fn from(inner: web_sys::HtmlTrackElement) -> Self {
        HtmlTrackElement { inner }
    }
}

impl AsRef<web_sys::HtmlTrackElement> for HtmlTrackElement {
    fn as_ref(&self) -> &web_sys::HtmlTrackElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlTrackElement);
impl_try_from_element!(HtmlTrackElement);
impl_known_element!(HtmlTrackElement, "TRACK");
