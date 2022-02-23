use delegate::delegate;
use wasm_bindgen::UnwrapThrowExt;

use crate::dom::impl_try_from_element;
use crate::html::{impl_html_element_traits, impl_known_element, AnchorRelationshipTypes};
use crate::media_type::MediaType;
use crate::security::ReferrerPolicy;
use crate::url::{absolute_url_or_relative_url_seal::Seal, AbsoluteOrRelativeUrl, Url};

#[derive(Clone)]
pub struct HtmlAElement {
    inner: web_sys::HtmlAnchorElement,
}

impl HtmlAElement {
    delegate! {
        target self.inner {
            pub fn target(&self) -> String;

            pub fn set_target(&self, target: &str);

            pub fn download(&self) -> String;

            pub fn set_download(&self, download: &str);
        }
    }

    pub fn href(&self) -> Option<Url> {
        Url::parse(self.inner.href().as_ref()).ok()
    }

    pub fn set_href<T>(&self, href: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_href(href.as_str());
    }

    pub fn ping(&self) -> Vec<Url> {
        let ping = self.inner.ping();

        let iter = ping.split_ascii_whitespace();
        let mut result = Vec::new();

        for candidate in iter {
            if let Ok(parsed) = Url::parse(candidate) {
                result.push(parsed)
            }
        }

        result
    }

    pub fn set_ping<I>(&self, mut ping: I)
    where
        I: Iterator,
        I::Item: AbsoluteOrRelativeUrl,
    {
        let mut serialized = String::new();

        if let Some(url) = ping.next() {
            serialized.push_str(url.as_str());
        }

        for url in ping {
            serialized.push(' ');
            serialized.push_str(url.as_str());
        }

        self.inner.set_ping(&serialized);
    }

    pub fn text(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.text().unwrap_throw()
    }

    pub fn set_text(&self, text: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.set_text(text).unwrap_throw();
    }

    pub fn media_type(&self) -> Option<MediaType> {
        MediaType::parse(self.inner.type_().as_ref()).ok()
    }

    pub fn set_media_type(&self, media_type: Option<&MediaType>) {
        self.inner
            .set_type(media_type.map(|m| m.as_ref()).unwrap_or(""));
    }

    pub fn referrer_policy(&self) -> ReferrerPolicy {
        ReferrerPolicy::from_str(&self.inner.referrer_policy())
    }

    pub fn set_referrer_policy(&self, referrer_policy: ReferrerPolicy) {
        self.inner.set_referrer_policy(referrer_policy.as_str())
    }

    pub fn rel(&self) -> AnchorRelationshipTypes {
        AnchorRelationshipTypes::new(self.inner.clone())
    }
}

impl From<web_sys::HtmlAnchorElement> for HtmlAElement {
    fn from(inner: web_sys::HtmlAnchorElement) -> Self {
        HtmlAElement { inner }
    }
}

impl AsRef<web_sys::HtmlAnchorElement> for HtmlAElement {
    fn as_ref(&self) -> &web_sys::HtmlAnchorElement {
        &self.inner
    }
}

impl_html_element_traits!(HtmlAElement);
impl_try_from_element!(HtmlAElement, HtmlAnchorElement);
impl_known_element!(HtmlAElement, HtmlAnchorElement, "A");
