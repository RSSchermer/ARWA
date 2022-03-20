use std::fmt::Write;
use std::str::FromStr;

use delegate::delegate;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::cssom::{link_style_seal, CssStyleSheet, LinkStyle};
use crate::dom::impl_try_from_element;
use crate::html::{
    impl_extendable_element, impl_html_element_traits, impl_known_element, LinkRelationshipTypes,
};
use crate::lang::LanguageTag;
use crate::media_type::MediaType;
use crate::url::Url;

pub enum PotentialRequestDestination {
    Fetch,
    Audio,
    AudioWorklet,
    Document,
    Embed,
    Font,
    Frame,
    IFrame,
    Image,
    Manifest,
    Object,
    PaintWorklet,
    Report,
    Script,
    ServiceWorker,
    SharedWorker,
    Style,
    Track,
    Video,
    Worker,
    Xslt,
}

#[derive(Clone)]
pub struct HtmlLinkElement {
    inner: web_sys::HtmlLinkElement,
}

impl HtmlLinkElement {
    delegate! {
        target self.inner {
            pub fn disabled(&self) -> bool;

            pub fn set_disabled(&self, disabled: bool);
        }
    }

    pub fn href(&self) -> Option<Url> {
        Url::parse(self.inner.href().as_ref()).ok()
    }

    pub fn set_href(&self, href: &Url) {
        self.inner.set_href(href.as_ref());
    }

    pub fn href_lang(&self) -> Option<LanguageTag> {
        LanguageTag::parse(self.inner.hreflang().as_ref()).ok()
    }

    pub fn set_href_lang(&self, hreflang: Option<&LanguageTag>) {
        self.inner
            .set_hreflang(hreflang.map(|l| l.as_ref()).unwrap_or(""))
    }

    pub fn as_destination(&self) -> Option<PotentialRequestDestination> {
        // Note: the `as` attribute is lower-cased by the host.
        match self.inner.as_().as_ref() {
            "fetch" => Some(PotentialRequestDestination::Fetch),
            "audio" => Some(PotentialRequestDestination::Audio),
            "audioworklet" => Some(PotentialRequestDestination::AudioWorklet),
            "document" => Some(PotentialRequestDestination::Document),
            "embed" => Some(PotentialRequestDestination::Embed),
            "font" => Some(PotentialRequestDestination::Font),
            "frame" => Some(PotentialRequestDestination::Frame),
            "iframe" => Some(PotentialRequestDestination::IFrame),
            "image" => Some(PotentialRequestDestination::Image),
            "manifest" => Some(PotentialRequestDestination::Manifest),
            "object" => Some(PotentialRequestDestination::Object),
            "paintworklet" => Some(PotentialRequestDestination::PaintWorklet),
            "report" => Some(PotentialRequestDestination::Report),
            "script" => Some(PotentialRequestDestination::Script),
            "serviceworker" => Some(PotentialRequestDestination::ServiceWorker),
            "sharedworker" => Some(PotentialRequestDestination::SharedWorker),
            "style" => Some(PotentialRequestDestination::Style),
            "track" => Some(PotentialRequestDestination::Track),
            "video" => Some(PotentialRequestDestination::Video),
            "worker" => Some(PotentialRequestDestination::Worker),
            "xslt" => Some(PotentialRequestDestination::Xslt),
            _ => None,
        }
    }

    pub fn set_as_destination(&self, as_destination: &str) {
        self.inner.set_as(as_destination);
    }

    pub fn media_type(&self) -> Option<MediaType> {
        MediaType::parse(self.inner.type_().as_ref()).ok()
    }

    pub fn set_media_type(&self, media_type: Option<&MediaType>) {
        self.inner
            .set_type(media_type.map(|m| m.as_ref()).unwrap_or(""));
    }

    pub fn rel(&self) -> LinkRelationshipTypes {
        LinkRelationshipTypes::new(self.inner.clone())
    }

    pub fn sizes(&self) -> LinkIconSizes {
        LinkIconSizes::from_serialized(self.inner.sizes().value())
    }

    // TODO: media
}

impl link_style_seal::Seal for HtmlLinkElement {}

impl LinkStyle for HtmlLinkElement {
    fn sheet(&self) -> Option<CssStyleSheet> {
        self.inner
            .sheet()
            .map(|s| CssStyleSheet::from(s.unchecked_into::<web_sys::CssStyleSheet>()))
    }
}

impl From<web_sys::HtmlLinkElement> for HtmlLinkElement {
    fn from(inner: web_sys::HtmlLinkElement) -> Self {
        HtmlLinkElement { inner }
    }
}

impl AsRef<web_sys::HtmlLinkElement> for HtmlLinkElement {
    fn as_ref(&self) -> &web_sys::HtmlLinkElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlLinkElement);
impl_try_from_element!(HtmlLinkElement);
impl_known_element!(HtmlLinkElement, "LINK");
impl_extendable_element!(HtmlLinkElement, "link");

pub enum LinkIconSizes {
    Any,
    Listed(Vec<LinkIconSize>),
}

impl LinkIconSizes {
    fn from_serialized(mut serialized: String) -> Self {
        serialized.make_ascii_lowercase();

        let serialized = serialized.trim();

        if serialized == "any" {
            LinkIconSizes::Any
        } else {
            let mut res = Vec::new();

            for token in serialized.split_ascii_whitespace() {
                if let Some(link_size) = LinkIconSize::from_token(token) {
                    res.push(link_size);
                }
            }

            LinkIconSizes::Listed(res)
        }
    }

    pub fn serialize(&self) -> String {
        match self {
            LinkIconSizes::Any => "any".to_string(),
            LinkIconSizes::Listed(listed) => {
                let mut result = String::new();

                for LinkIconSize(width, height) in listed {
                    write!(&mut result, "{}x{}", width, height).unwrap_throw();
                }

                result
            }
        }
    }
}

pub struct LinkIconSize(u32, u32);

impl LinkIconSize {
    fn from_token(token: &str) -> Option<Self> {
        token.find('x').and_then(|pos| {
            if pos == token.len() - 1 {
                None
            } else {
                let maybe_width = u32::from_str(&token[..pos]);
                let maybe_height = u32::from_str(&token[(pos + 1)..]);

                match (maybe_width, maybe_height) {
                    (Ok(width), Ok(height)) => Some(LinkIconSize(width, height)),
                    _ => None,
                }
            }
        })
    }
}
