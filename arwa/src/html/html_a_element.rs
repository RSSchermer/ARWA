use crate::collection::{Collection, Sequence};
use crate::html::LinkTypes;
use crate::media_type::MediaType;
use crate::security::ReferrerPolicy;
use crate::url::{AbsoluteOrRelativeUrl, Url};

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
        Url::parse(self.inner.href()).ok()
    }

    pub fn set_href<T>(&self, href: T)
    where
        T: AbsoluteOrRelativeUrl,
    {
        self.inner.set_href(href.as_str());
    }

    pub fn ping(&self) -> Vec<Url> {
        let mut iter = self.inner.ping().split_ascii_whitespace();
        let mut result = Vec::new();

        for candidate in iter {
            if let Ok(parsed) = Url::parse(candidate) {
                result.push(parsed)
            }
        }

        result
    }

    pub fn set_ping<I>(&self, ping: I)
    where
        I: Iterator,
        I::Item: AbsoluteOrRelativeUrl,
    {
        let serialized: String = ping.map(|url| url.as_str()).intersperse(" ").collect();

        self.inner.set_ping(&serialized);
    }

    pub fn text(&self) -> String {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.text().unwrap()
    }

    pub fn set_text(&self, text: &str) {
        // No indication in the spec that this can fail, unwrap for now.
        self.inner.set_text(text).unwrap();
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
        HtmlAnchorElement { inner }
    }
}

impl AsRef<web_sys::HtmlAnchorElement> for HtmlAElement {
    fn as_ref(&self) -> Self {
        &self.inner
    }
}

impl_html_element_traits!(HtmlAElement);
impl_try_from_element!(HtmlAElement, web_sys::HtmlAnchorElement);
impl_known_element!(HtmlAElement, "A");
