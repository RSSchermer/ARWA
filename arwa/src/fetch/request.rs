use crate::fetch::{Body, BodySource, Headers, RequestMethod};
use crate::security::ReferrerPolicy;
use crate::url::ContextualUrl;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;
use url::Url;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};

// Note: decided to duplicate the various web_sys enums, because though at first glance they seem
// identical, it allows us to implement Default and it gives us the ability to attach documentation.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequestCache {
    Default,
    NoStore,
    Reload,
    NoCache,
    ForceCache,
    OnlyIfCached,
}

impl Default for RequestCache {
    fn default() -> Self {
        RequestCache::Default
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequestCredentials {
    SameOrigin,
    Omit,
    Include,
}

impl Default for RequestCredentials {
    fn default() -> Self {
        RequestCredentials::SameOrigin
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequestDestination {
    Unspecified,
    Audio,
    AudioWorklet,
    Document,
    Embed,
    Font,
    Image,
    Manifest,
    Object,
    PaintWorklet,
    Report,
    Script,
    SharedWorker,
    Style,
    Track,
    Video,
    Worker,
    Xslt,
}

impl Default for RequestDestination {
    fn default() -> Self {
        RequestDestination::Unspecified
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequestMode {
    SameOrigin,
    NoCors,
    Cors,
    Navigate,
}

impl Default for RequestMode {
    fn default() -> Self {
        RequestMode::Cors
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RequestRedirect {
    Follow,
    Error,
    Manual,
}

impl Default for RequestRedirect {
    fn default() -> Self {
        RequestRedirect::Follow
    }
}

pub enum RequestReferrer {
    NoReferrer,
    Client,
    Url(Url),
}

impl Default for RequestReferrer {
    fn default() -> Self {
        RequestReferrer::Client
    }
}

pub struct RequestDescriptor<'a> {
    pub method: RequestMethod<'a>,
    pub headers: Option<&'a Headers>,
    pub body: Option<BodySource<'a>>,
    pub mode: RequestMode,
    pub credentials: RequestCredentials,
    pub cache: RequestCache,
    pub redirect: RequestRedirect,
    pub referrer: RequestReferrer,
    pub referrer_policy: ReferrerPolicy,
    pub integrity: Option<&'a str>,
}

impl Default for RequestDescriptor {
    fn default() -> Self {
        RequestDescriptor {
            method: RequestMethod::default(),
            headers: None,
            body: None,
            mode: RequestMode::default(),
            credentials: RequestCredentials::default(),
            cache: RequestCache::default(),
            redirect: RequestRedirect::default(),
            referrer: RequestReferrer::default(),
            referrer_policy: ReferrerPolicy::default(),
            integrity: None,
        }
    }
}

#[derive(Clone)]
pub struct RequestInitError {
    inner: js_sys::TypeError,
}

impl RequestInitError {
    fn new(inner: js_sys::TypeError) -> Self {
        RequestInitError { inner }
    }
}

#[derive(Clone)]
pub struct Request {
    inner: web_sys::Request,
}

impl Request {
    pub fn init(url: ContextualUrl, descriptor: RequestDescriptor) -> Self {
        create_request_internal(url, descriptor).unwrap_throw()
    }

    pub fn try_init(
        url: ContextualUrl,
        descriptor: RequestDescriptor,
    ) -> Result<Self, RequestInitError> {
        create_request_internal(url, descriptor)
            .map_err(|err| CreateRequestError::new(err.unchecked_into()))
    }

    pub fn method(&self) -> RequestMethod<'static> {
        RequestMethod::from_string_unchecked(self.inner.method())
    }

    pub fn url(&self) -> Url {
        Url::parse(self.inner.url().as_ref()).unwrap()
    }

    pub fn headers(&self) -> Headers {
        self.inner.headers().into()
    }

    pub fn body(&self) -> Body {
        Body::request(self.inner.clone())
    }

    pub fn cache(&self) -> RequestCache {
        match self.inner.cache() {
            web_sys::RequestCache::Default => RequestCache::Default,
            web_sys::RequestCache::NoStore => RequestCache::NoStore,
            web_sys::RequestCache::Reload => RequestCache::Reload,
            web_sys::RequestCache::NoCache => RequestCache::NoCache,
            web_sys::RequestCache::ForceCache => RequestCache::ForceCache,
            web_sys::RequestCache::OnlyIfCached => RequestCache::OnlyIfCached,
        }
    }

    pub fn credentials(&self) -> RequestCredentials {
        match self.inner.credentials() {
            web_sys::RequestCredentials::SameOrigin => RequestCredentials::SameOrigin,
            web_sys::RequestCredentials::Omit => RequestCredentials::Omit,
            web_sys::RequestCredentials::Include => RequestCredentials::Include,
        }
    }

    pub fn destination(&self) -> RequestDestination {
        match self.inner.destination() {
            web_sys::RequestDestination::None => RequestDestination::Unspecified,
            web_sys::RequestDestination::Audio => RequestDestination::Audio,
            web_sys::RequestDestination::Audioworklet => RequestDestination::AudioWorklet,
            web_sys::RequestDestination::Document => RequestDestination::Document,
            web_sys::RequestDestination::Embed => RequestDestination::Embed,
            web_sys::RequestDestination::Font => RequestDestination::Font,
            web_sys::RequestDestination::Image => RequestDestination::Image,
            web_sys::RequestDestination::Manifest => RequestDestination::Manifest,
            web_sys::RequestDestination::Object => RequestDestination::Object,
            web_sys::RequestDestination::Paintworklet => RequestDestination::PaintWorklet,
            web_sys::RequestDestination::Report => RequestDestination::Report,
            web_sys::RequestDestination::Script => RequestDestination::Script,
            web_sys::RequestDestination::Sharedworker => RequestDestination::SharedWorker,
            web_sys::RequestDestination::Style => RequestDestination::Style,
            web_sys::RequestDestination::Track => RequestDestination::Track,
            web_sys::RequestDestination::Video => RequestDestination::Video,
            web_sys::RequestDestination::Worker => RequestDestination::Worker,
            web_sys::RequestDestination::Xslt => RequestDestination::Xslt,
        }
    }

    pub fn integrity(&self) -> Option<String> {
        let integrity = self.inner.integrity();

        if integrity.is_empty() {
            None
        } else {
            Some(integrity)
        }
    }

    pub fn mode(&self) -> RequestMode {
        match self.inner.mode() {
            web_sys::RequestMode::SameOrigin => RequestMode::SameOrigin,
            web_sys::RequestMode::NoCors => RequestMode::NoCors,
            web_sys::RequestMode::Cors => RequestMode::Cors,
            web_sys::RequestMode::Navigate => RequestMode::Navigate,
        }
    }

    pub fn redirect(&self) -> RequestRedirect {
        match self.inner.redirect() {
            web_sys::RequestRedirect::Follow => RequestRedirect::Follow,
            web_sys::RequestRedirect::Error => RequestRedirect::Error,
            web_sys::RequestRedirect::Manual => RequestRedirect::Manual,
        }
    }

    pub fn referrer(&self) -> RequestReferrer {
        match self.inner.referrer().as_ref() {
            "" => RequestReferrer::NoReferrer,
            "about:client" => RequestReferrer::Client,
            url => RequestReferrer::Url(Url::parse(url).unwrap()),
        }
    }

    pub fn referrer_policy(&self) -> ReferrerPolicy {
        match self.inner.referror_policy() {
            web_sys::ReferrerPolicy::None => ReferrerPolicy::Default,
            web_sys::ReferrerPolicy::v => ReferrerPolicy::NoReferrer,
            web_sys::ReferrerPolicy::NoReferrerWhenDowngrade => {
                ReferrerPolicy::NoReferrerWhenDowngrade
            }
            web_sys::ReferrerPolicy::Origin => ReferrerPolicy::Origin,
            web_sys::ReferrerPolicy::OriginWhenCrossOrigin => ReferrerPolicy::OriginWhenCrossOrigin,
            web_sys::ReferrerPolicy::UnsafeUrl => ReferrerPolicy::UnsafeUrl,
            web_sys::ReferrerPolicy::SameOrigin => ReferrerPolicy::SameOrigin,
            web_sys::ReferrerPolicy::StrictOrigin => ReferrerPolicy::StrictOrigin,
            web_sys::ReferrerPolicy::StrictOriginWhenCrossOrigin => {
                ReferrerPolicy::StrictOriginWhenCrossOrigin
            }
        }
    }
}

impl From<web_sys::Request> for Request {
    fn from(inner: web_sys::Request) -> Self {
        Request { inner }
    }
}

impl From<Request> for web_sys::Request {
    fn from(request: Request) -> Self {
        request.inner
    }
}

impl AsRef<web_sys::Request> for Request {
    fn as_ref(&self) -> &web_sys::Request {
        &self.inner
    }
}

impl_common_wrapper_traits!(Request);

fn create_request_internal(
    url: ContextualUrl,
    descriptor: RequestDescriptor,
) -> Result<Request, JsValue> {
    let RequestDescriptor {
        method,
        headers,
        body,
        mode,
        credentials,
        cache,
        redirect,
        referrer,
        referrer_policy,
        integrity,
    } = descriptor;

    let mut init = web_sys::RequestInit::new();

    init.method(method.as_ref());

    if let Some(headers) = headers {
        init.headers(headers.as_ref())
    }

    // TODO: test when bytes drop, might have to wrap body in ManuallyDrop to ensure it happens
    // after the Request is created (which will make a copy of the byte slice).
    if let Some(body) = body {
        match body {
            BodySource::String(string) => init.body(Some(&JsValue::from_str(&string))),
            BodySource::Blob(blob) => init.body(Some(blob.as_ref())),
            BodySource::Bytes(bytes) => init.body(Some(js_sys::Uint8Array::view(bytes).as_ref())),
        }
    }

    match mode {
        RequestMode::SameOrigin => init.mode(web_sys::RequestMode::SameOrigin),
        RequestMode::NoCors => init.mode(web_sys::RequestMode::NoCors),
        RequestMode::Cors => init.mode(web_sys::RequestMode::Cors),
        RequestMode::Navigate => init.mode(web_sys::RequestMode::Navigate),
    }

    match credentials {
        RequestCredentials::SameOrigin => init.credentials(web_sys::RequestCredentials::SameOrigin),
        RequestCredentials::Omit => init.credentials(web_sys::RequestCredentials::Omit),
        RequestCredentials::Include => init.credentials(web_sys::RequestCredentials::Include),
    }

    match cache {
        RequestCache::Default => init.credentials(web_sys::RequestCache::Default),
        RequestCache::NoStore => init.credentials(web_sys::RequestCache::NoStore),
        RequestCache::Reload => init.credentials(web_sys::RequestCache::Reload),
        RequestCache::NoCache => init.credentials(web_sys::RequestCache::NoCache),
        RequestCache::ForceCache => init.credentials(web_sys::RequestCache::ForceCache),
        RequestCache::OnlyIfCached => init.credentials(web_sys::RequestCache::OnlyIfCached),
    }

    match redirect {
        RequestRedirect::Follow => init.redirect(web_sys::RequestRedirect::Follow),
        RequestRedirect::Error => init.redirect(web_sys::RequestRedirect::Error),
        RequestRedirect::Manual => init.redirect(web_sys::RequestRedirect::Manual),
    }

    match referrer {
        RequestReferrer::NoReferrer => init.referrer(""),
        RequestReferrer::Client => init.referrer("about:client"),
        RequestReferrer::Url(url) => init.referrer(url.as_ref()),
    }

    match referrer_policy {
        ReferrerPolicy::Default => init.referrer_policy(web_sys::ReferrerPolicy::None),
        ReferrerPolicy::NoReferrer => init.referrer_policy(web_sys::ReferrerPolicy::NoReferrer),
        ReferrerPolicy::NoReferrerWhenDowngrade => {
            init.referrer_policy(web_sys::ReferrerPolicy::NoReferrerWhenDowngrade)
        }
        ReferrerPolicy::SameOrigin => init.referrer_policy(web_sys::ReferrerPolicy::SameOrigin),
        ReferrerPolicy::Origin => init.referrer_policy(web_sys::ReferrerPolicy::Origin),
        ReferrerPolicy::StrictOrigin => init.referrer_policy(web_sys::ReferrerPolicy::StrictOrigin),
        ReferrerPolicy::OriginWhenCrossOrigin => {
            init.referrer_policy(web_sys::ReferrerPolicy::OriginWhenCrossOrigin)
        }
        ReferrerPolicy::StrictOriginWhenCrossOrigin => {
            init.referrer_policy(web_sys::ReferrerPolicy::StrictOriginWhenCrossOrigin)
        }
        ReferrerPolicy::UnsafeUrl => init.referrer_policy(web_sys::ReferrerPolicy::UnsafeUrl),
    }

    if let Some(integrity) = integrity {
        init.integrity(integrity);
    }

    web_sys::Request::new_with_str_and_init(url.as_ref(), &init).map(|r| r.into())
}
