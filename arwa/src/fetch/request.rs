use wasm_bindgen::{throw_val, JsCast, JsValue, UnwrapThrowExt};

use crate::fetch::{Body, BodySource, Headers, RequestMethod};
use crate::security::ReferrerPolicy;
use crate::url::Url;
use crate::{impl_common_wrapper_traits, impl_js_cast, type_error_wrapper};

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

impl RequestCache {
    fn from_web_sys(cache: web_sys::RequestCache) -> Self {
        match cache {
            web_sys::RequestCache::Default => RequestCache::Default,
            web_sys::RequestCache::NoStore => RequestCache::NoStore,
            web_sys::RequestCache::Reload => RequestCache::Reload,
            web_sys::RequestCache::NoCache => RequestCache::NoCache,
            web_sys::RequestCache::ForceCache => RequestCache::ForceCache,
            web_sys::RequestCache::OnlyIfCached => RequestCache::OnlyIfCached,
            _ => unreachable!(),
        }
    }

    fn to_web_sys(&self) -> web_sys::RequestCache {
        match self {
            RequestCache::Default => web_sys::RequestCache::Default,
            RequestCache::NoStore => web_sys::RequestCache::NoStore,
            RequestCache::Reload => web_sys::RequestCache::Reload,
            RequestCache::NoCache => web_sys::RequestCache::NoCache,
            RequestCache::ForceCache => web_sys::RequestCache::ForceCache,
            RequestCache::OnlyIfCached => web_sys::RequestCache::OnlyIfCached,
        }
    }
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

impl RequestCredentials {
    fn from_web_sys(credentials: web_sys::RequestCredentials) -> Self {
        match credentials {
            web_sys::RequestCredentials::SameOrigin => RequestCredentials::SameOrigin,
            web_sys::RequestCredentials::Omit => RequestCredentials::Omit,
            web_sys::RequestCredentials::Include => RequestCredentials::Include,
            _ => unreachable!(),
        }
    }

    pub(crate) fn to_web_sys(&self) -> web_sys::RequestCredentials {
        match self {
            RequestCredentials::SameOrigin => web_sys::RequestCredentials::SameOrigin,
            RequestCredentials::Omit => web_sys::RequestCredentials::Omit,
            RequestCredentials::Include => web_sys::RequestCredentials::Include,
        }
    }
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

impl RequestDestination {
    fn from_web_sys(destination: web_sys::RequestDestination) -> Self {
        match destination {
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
            _ => unreachable!(),
        }
    }
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

impl RequestMode {
    fn from_web_sys(mode: web_sys::RequestMode) -> Self {
        match mode {
            web_sys::RequestMode::SameOrigin => RequestMode::SameOrigin,
            web_sys::RequestMode::NoCors => RequestMode::NoCors,
            web_sys::RequestMode::Cors => RequestMode::Cors,
            web_sys::RequestMode::Navigate => RequestMode::Navigate,
            _ => unreachable!(),
        }
    }

    fn to_web_sys(&self) -> web_sys::RequestMode {
        match self {
            RequestMode::SameOrigin => web_sys::RequestMode::SameOrigin,
            RequestMode::NoCors => web_sys::RequestMode::NoCors,
            RequestMode::Cors => web_sys::RequestMode::Cors,
            RequestMode::Navigate => web_sys::RequestMode::Navigate,
        }
    }
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

impl RequestRedirect {
    fn from_web_sys(redirect: web_sys::RequestRedirect) -> Self {
        match redirect {
            web_sys::RequestRedirect::Follow => RequestRedirect::Follow,
            web_sys::RequestRedirect::Error => RequestRedirect::Error,
            web_sys::RequestRedirect::Manual => RequestRedirect::Manual,
            _ => unreachable!(),
        }
    }

    fn to_web_sys(&self) -> web_sys::RequestRedirect {
        match self {
            RequestRedirect::Follow => web_sys::RequestRedirect::Follow,
            RequestRedirect::Error => web_sys::RequestRedirect::Error,
            RequestRedirect::Manual => web_sys::RequestRedirect::Manual,
        }
    }
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

impl RequestReferrer {
    fn from_str(referrer: &str) -> Self {
        match referrer {
            "" => RequestReferrer::NoReferrer,
            "about:client" => RequestReferrer::Client,
            url => RequestReferrer::Url(Url::parse(url).unwrap_throw()),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            RequestReferrer::NoReferrer => "",
            RequestReferrer::Client => "about:client",
            RequestReferrer::Url(url) => url.as_ref(),
        }
    }
}

impl Default for RequestReferrer {
    fn default() -> Self {
        RequestReferrer::Client
    }
}

pub struct RequestDescriptor<'a> {
    pub method: RequestMethod,
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

impl Default for RequestDescriptor<'static> {
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

type_error_wrapper!(RequestInitError);

#[derive(Clone)]
pub struct Request {
    inner: web_sys::Request,
}

impl Request {
    pub fn init(url: &Url, descriptor: RequestDescriptor) -> Self {
        match create_request_internal(url.as_ref(), descriptor) {
            Ok(request) => request,
            Err(err) => throw_val(err),
        }
    }

    pub fn try_init(url: &Url, descriptor: RequestDescriptor) -> Result<Self, RequestInitError> {
        create_request_internal(url.as_ref(), descriptor)
            .map_err(|err| RequestInitError::new(err.unchecked_into()))
    }

    pub fn method(&self) -> RequestMethod {
        RequestMethod::trusted(self.inner.method())
    }

    pub fn url(&self) -> Url {
        Url::parse(self.inner.url().as_ref()).unwrap_throw()
    }

    pub fn headers(&self) -> Headers {
        self.inner.headers().into()
    }

    pub fn body(&self) -> Body {
        Body::request(Clone::clone(&self.inner))
    }

    pub fn cache(&self) -> RequestCache {
        RequestCache::from_web_sys(self.inner.cache())
    }

    pub fn credentials(&self) -> RequestCredentials {
        RequestCredentials::from_web_sys(self.inner.credentials())
    }

    pub fn destination(&self) -> RequestDestination {
        RequestDestination::from_web_sys(self.inner.destination())
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
        RequestMode::from_web_sys(self.inner.mode())
    }

    pub fn redirect(&self) -> RequestRedirect {
        RequestRedirect::from_web_sys(self.inner.redirect())
    }

    pub fn referrer(&self) -> RequestReferrer {
        RequestReferrer::from_str(self.inner.referrer().as_ref())
    }

    pub fn referrer_policy(&self) -> ReferrerPolicy {
        ReferrerPolicy::from_web_sys(self.inner.referrer_policy())
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
impl_js_cast!(Request);

fn create_request_internal(url: &str, descriptor: RequestDescriptor) -> Result<Request, JsValue> {
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
        init.headers(headers.as_ref());
    }

    if let Some(body) = body {
        match body {
            BodySource::String(string) => init.body(Some(&JsValue::from_str(&string))),
            BodySource::Blob(blob) => init.body(Some(blob.as_ref())),
            BodySource::Bytes(bytes) => unsafe {
                // TODO: test when bytes drop, might have to wrap body in ManuallyDrop to ensure it
                // happens after the Request is created (which per the spec will make a copy of the
                // byte slice).
                init.body(Some(js_sys::Uint8Array::view(bytes).as_ref()))
            },
        };
    }

    init.mode(mode.to_web_sys());
    init.credentials(credentials.to_web_sys());
    init.cache(cache.to_web_sys());
    init.redirect(redirect.to_web_sys());
    init.referrer(referrer.as_str());
    init.referrer_policy(referrer_policy.to_web_sys());

    if let Some(integrity) = integrity {
        init.integrity(integrity);
    }

    web_sys::Request::new_with_str_and_init(url, &init).map(|r| r.into())
}
