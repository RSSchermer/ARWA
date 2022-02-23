use std::borrow::Cow;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

use lazycell::LazyCell;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::type_error_wrapper;

pub(crate) mod absolute_url_or_relative_url_seal {
    pub trait Seal {
        #[doc(hidden)]
        fn as_str(&self) -> &str;
    }
}

pub trait AbsoluteOrRelativeUrl: absolute_url_or_relative_url_seal::Seal {}

impl absolute_url_or_relative_url_seal::Seal for Url {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}
impl AbsoluteOrRelativeUrl for Url {}

impl absolute_url_or_relative_url_seal::Seal for RelativeUrl {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}
impl AbsoluteOrRelativeUrl for RelativeUrl {}

#[doc(hidden)]
#[derive(Clone)]
pub struct StaticallyParsedUrl {
    #[doc(hidden)]
    pub raw: &'static str,
    #[doc(hidden)]
    pub scheme_end: usize,
    #[doc(hidden)]
    pub username_range: Option<Range<usize>>,
    #[doc(hidden)]
    pub password_range: Option<Range<usize>>,
    #[doc(hidden)]
    pub host_range: Option<Range<usize>>,
    #[doc(hidden)]
    pub port: Option<u16>,
    #[doc(hidden)]
    pub path_range: Option<Range<usize>>,
    #[doc(hidden)]
    pub query_range: Option<Range<usize>>,
    #[doc(hidden)]
    pub fragment_range: Option<Range<usize>>,
    #[doc(hidden)]
    pub origin: Origin<'static>,
}

impl StaticallyParsedUrl {
    fn scheme(&self) -> &str {
        &self.raw[0..self.scheme_end]
    }

    fn username(&self) -> Option<&str> {
        self.username_range.as_ref().map(|r| &self.raw[r.clone()])
    }

    fn password(&self) -> Option<&str> {
        self.password_range.as_ref().map(|r| &self.raw[r.clone()])
    }

    fn host(&self) -> Option<&str> {
        self.host_range.as_ref().map(|r| &self.raw[r.clone()])
    }

    fn port(&self) -> Option<u16> {
        self.port
    }

    fn port_or_known_default(&self) -> Option<u16> {
        let port = self.port();

        if port.is_some() {
            return port;
        }

        match self.scheme() {
            "http" | "ws" => Some(80),
            "https" | "wss" => Some(443),
            "ftp" => Some(21),
            _ => None,
        }
    }

    fn path(&self) -> Option<&str> {
        self.path_range.as_ref().map(|r| &self.raw[r.clone()])
    }

    fn query(&self) -> Option<&str> {
        self.query_range.as_ref().map(|r| &self.raw[r.clone()])
    }

    fn fragment(&self) -> Option<&str> {
        self.fragment_range.as_ref().map(|r| &self.raw[r.clone()])
    }

    fn origin(&self) -> Origin {
        self.origin
    }
}

impl AsRef<str> for StaticallyParsedUrl {
    fn as_ref(&self) -> &str {
        self.raw
    }
}

#[derive(Clone, Copy)]
enum OriginInternal {
    Opaque,
    Tuple,
}

struct ParsedDynamicCache {
    scheme: LazyCell<String>,
    username: LazyCell<String>,
    password: LazyCell<String>,
    host: LazyCell<String>,
    port: LazyCell<Option<u16>>,
    path: LazyCell<String>,
    query: LazyCell<String>,
    fragment: LazyCell<String>,
    origin: LazyCell<OriginInternal>,
}

impl ParsedDynamicCache {
    fn uninitialized() -> Self {
        ParsedDynamicCache {
            scheme: LazyCell::new(),
            username: LazyCell::new(),
            password: LazyCell::new(),
            host: LazyCell::new(),
            port: LazyCell::new(),
            path: LazyCell::new(),
            query: LazyCell::new(),
            fragment: LazyCell::new(),
            origin: LazyCell::new(),
        }
    }
}

struct DynamicallyParsedUrl {
    raw: String,
    parsed: web_sys::Url,
    cache: ParsedDynamicCache,
}

impl DynamicallyParsedUrl {
    fn scheme(&self) -> &str {
        self.cache.scheme.borrow_with(|| self.parsed.protocol())
    }

    fn username(&self) -> Option<&str> {
        let username = self.cache.username.borrow_with(|| self.parsed.username());

        if username.is_empty() {
            None
        } else {
            Some(username)
        }
    }

    fn password(&self) -> Option<&str> {
        let password = self.cache.password.borrow_with(|| self.parsed.password());

        if password.is_empty() {
            None
        } else {
            Some(password)
        }
    }

    fn host(&self) -> Option<&str> {
        let host = self.cache.host.borrow_with(|| self.parsed.hostname());

        if host.is_empty() {
            None
        } else {
            Some(host)
        }
    }

    fn port(&self) -> Option<u16> {
        let port = self
            .cache
            .port
            .borrow_with(|| u16::from_str(self.parsed.port().as_ref()).ok());

        *port
    }

    fn port_or_known_default(&self) -> Option<u16> {
        let port = self.port();

        if port.is_some() {
            return port;
        }

        match self.scheme() {
            "http" | "ws" => Some(80),
            "https" | "wss" => Some(443),
            "ftp" => Some(21),
            _ => None,
        }
    }

    fn path(&self) -> Option<&str> {
        let path = self.cache.path.borrow_with(|| self.parsed.pathname());

        if path.is_empty() {
            None
        } else {
            Some(path)
        }
    }

    fn query(&self) -> Option<&str> {
        let query = self.cache.query.borrow_with(|| self.parsed.search());

        if query.is_empty() {
            None
        } else {
            Some(query)
        }
    }

    fn fragment(&self) -> Option<&str> {
        let fragment = self.cache.fragment.borrow_with(|| self.parsed.hash());

        if fragment.is_empty() {
            None
        } else {
            Some(fragment)
        }
    }

    fn origin(&self) -> Origin {
        let origin = self.cache.origin.borrow_with(|| {
            if self.parsed.origin().is_empty() {
                OriginInternal::Opaque
            } else {
                OriginInternal::Tuple
            }
        });

        match origin {
            OriginInternal::Opaque => Origin::Opaque,
            OriginInternal::Tuple => {
                let scheme = self.scheme();
                let host = self.host().unwrap_throw();
                let port = self.port_or_known_default().unwrap_throw();

                Origin::Tuple(scheme, host, port)
            }
        }
    }
}

impl AsRef<str> for DynamicallyParsedUrl {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}

#[derive(Clone, Copy)]
pub enum Origin<'a> {
    Opaque,
    Tuple(&'a str, &'a str, u16),
}

enum UrlInternal {
    Dynamic(DynamicallyParsedUrl),
    Static(StaticallyParsedUrl),
}

impl From<DynamicallyParsedUrl> for UrlInternal {
    fn from(url: DynamicallyParsedUrl) -> Self {
        UrlInternal::Dynamic(url)
    }
}

impl From<StaticallyParsedUrl> for UrlInternal {
    fn from(url: StaticallyParsedUrl) -> Self {
        UrlInternal::Static(url)
    }
}

pub struct Url {
    internal: UrlInternal,
}

impl Url {
    pub fn parse(url: &str) -> Result<Self, InvalidUrl> {
        web_sys::Url::new(&url)
            .map(|parsed| Url {
                internal: DynamicallyParsedUrl {
                    raw: url.to_string(),
                    parsed,
                    cache: ParsedDynamicCache::uninitialized(),
                }
                .into(),
            })
            .map_err(|err| InvalidUrl::new(err.unchecked_into()))
    }

    /// Only meant to be called by the accompanying proc-macro, not part of the public API.
    #[doc(hidden)]
    pub fn from_statically_parsed(parsed: StaticallyParsedUrl) -> Self {
        Url {
            internal: parsed.into(),
        }
    }

    pub fn scheme(&self) -> &str {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.scheme(),
            UrlInternal::Static(url) => url.scheme(),
        }
    }

    pub fn username(&self) -> Option<&str> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.username(),
            UrlInternal::Static(url) => url.username(),
        }
    }

    pub fn password(&self) -> Option<&str> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.password(),
            UrlInternal::Static(url) => url.password(),
        }
    }

    pub fn host(&self) -> Option<&str> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.host(),
            UrlInternal::Static(url) => url.host(),
        }
    }

    pub fn port(&self) -> Option<u16> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.port(),
            UrlInternal::Static(url) => url.port(),
        }
    }

    pub fn port_or_known_default(&self) -> Option<u16> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.port_or_known_default(),
            UrlInternal::Static(url) => url.port_or_known_default(),
        }
    }

    pub fn path(&self) -> Option<&str> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.path(),
            UrlInternal::Static(url) => url.path(),
        }
    }

    pub fn path_segments(&self) -> impl Iterator<Item = &str> {
        self.path().unwrap_or("").split('/')
    }

    pub fn query(&self) -> Option<&str> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.query(),
            UrlInternal::Static(url) => url.query(),
        }
    }

    pub fn query_pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        query_pairs(self.query())
    }

    pub fn fragment(&self) -> Option<&str> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.fragment(),
            UrlInternal::Static(url) => url.fragment(),
        }
    }

    pub fn origin(&self) -> Origin {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.origin(),
            UrlInternal::Static(url) => url.origin(),
        }
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.as_ref(),
            UrlInternal::Static(url) => url.as_ref(),
        }
    }
}

impl From<web_sys::Url> for Url {
    fn from(parsed: web_sys::Url) -> Self {
        Url {
            internal: UrlInternal::Dynamic(DynamicallyParsedUrl {
                raw: parsed.to_string().into(),
                parsed,
                cache: ParsedDynamicCache::uninitialized(),
            }),
        }
    }
}

impl PartialEq for Url {
    fn eq(&self, other: &Url) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl PartialEq<str> for Url {
    fn eq(&self, s: &str) -> bool {
        self.as_ref() == s
    }
}

impl<'a> PartialEq<&'a str> for Url {
    #[inline]
    fn eq(&self, s: &&'a str) -> bool {
        self == *s
    }
}

impl<'a> PartialEq<Url> for &'a str {
    #[inline]
    fn eq(&self, url: &Url) -> bool {
        url == self
    }
}

impl PartialEq<Url> for str {
    #[inline]
    fn eq(&self, url: &Url) -> bool {
        url == self
    }
}

impl fmt::Debug for Url {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

impl fmt::Display for Url {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_ref(), f)
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct StaticallyParsedRelativeUrl {
    #[doc(hidden)]
    pub raw: &'static str,
    #[doc(hidden)]
    pub path_range: Option<Range<usize>>,
    #[doc(hidden)]
    pub query_range: Option<Range<usize>>,
    #[doc(hidden)]
    pub fragment_range: Option<Range<usize>>,
}

impl StaticallyParsedRelativeUrl {
    fn path(&self) -> Option<&str> {
        self.path_range.as_ref().map(|r| &self.raw[r.clone()])
    }

    fn query(&self) -> Option<&str> {
        self.query_range.as_ref().map(|r| &self.raw[r.clone()])
    }

    fn fragment(&self) -> Option<&str> {
        self.fragment_range.as_ref().map(|r| &self.raw[r.clone()])
    }
}

impl AsRef<str> for StaticallyParsedRelativeUrl {
    fn as_ref(&self) -> &str {
        self.raw
    }
}

struct RelativeParsedDynamicCache {
    query: LazyCell<String>,
    fragment: LazyCell<String>,
}

impl RelativeParsedDynamicCache {
    fn uninitialized() -> Self {
        RelativeParsedDynamicCache {
            query: LazyCell::new(),
            fragment: LazyCell::new(),
        }
    }
}

pub struct DynamicallyParsedRelativeUrl {
    raw: String,
    path_end: usize,
    parsed: web_sys::Url,
    cache: RelativeParsedDynamicCache,
}

impl DynamicallyParsedRelativeUrl {
    fn path(&self) -> Option<&str> {
        if self.path_end > 0 {
            Some(&self.raw[0..self.path_end])
        } else {
            None
        }
    }

    fn query(&self) -> Option<&str> {
        let query = self.cache.query.borrow_with(|| self.parsed.search());

        if query.is_empty() {
            None
        } else {
            Some(query)
        }
    }

    fn fragment(&self) -> Option<&str> {
        let fragment = self.cache.fragment.borrow_with(|| self.parsed.hash());

        if fragment.is_empty() {
            None
        } else {
            Some(fragment)
        }
    }
}

impl AsRef<str> for DynamicallyParsedRelativeUrl {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}

enum RelativeUrlInternal {
    Static(StaticallyParsedRelativeUrl),
    Dynamic(DynamicallyParsedRelativeUrl),
}

impl From<StaticallyParsedRelativeUrl> for RelativeUrlInternal {
    fn from(url: StaticallyParsedRelativeUrl) -> Self {
        RelativeUrlInternal::Static(url)
    }
}

impl From<DynamicallyParsedRelativeUrl> for RelativeUrlInternal {
    fn from(url: DynamicallyParsedRelativeUrl) -> Self {
        RelativeUrlInternal::Dynamic(url)
    }
}

pub struct RelativeUrl {
    internal: RelativeUrlInternal,
}

impl RelativeUrl {
    pub fn parse(relative_url: &str) -> Result<RelativeUrl, InvalidUrl> {
        web_sys::Url::new_with_base(&relative_url, "http://dummy")
            .map(|parsed| RelativeUrl {
                internal: DynamicallyParsedRelativeUrl {
                    raw: relative_url.to_string(),
                    path_end: relative_url.find(&['?', '#']).unwrap_or(relative_url.len()),
                    parsed,
                    cache: RelativeParsedDynamicCache::uninitialized(),
                }
                .into(),
            })
            .map_err(|err| InvalidUrl::new(err.unchecked_into()))
    }

    /// Only meant to be called by the accompanying proc-macro, not part of the public API.
    #[doc(hidden)]
    pub fn from_statically_parsed(parsed: DynamicallyParsedRelativeUrl) -> Self {
        RelativeUrl {
            internal: parsed.into(),
        }
    }

    pub fn path(&self) -> Option<&str> {
        match &self.internal {
            RelativeUrlInternal::Dynamic(url) => url.path(),
            RelativeUrlInternal::Static(url) => url.path(),
        }
    }

    pub fn path_segments(&self) -> impl Iterator<Item = &str> {
        self.path().unwrap_or("").split('/')
    }

    pub fn query(&self) -> Option<&str> {
        match &self.internal {
            RelativeUrlInternal::Dynamic(url) => url.query(),
            RelativeUrlInternal::Static(url) => url.query(),
        }
    }

    pub fn query_pairs(&self) -> impl Iterator<Item = (&str, &str)> {
        query_pairs(self.query())
    }

    pub fn fragment(&self) -> Option<&str> {
        match &self.internal {
            RelativeUrlInternal::Dynamic(url) => url.fragment(),
            RelativeUrlInternal::Static(url) => url.fragment(),
        }
    }
}

impl AsRef<str> for RelativeUrl {
    fn as_ref(&self) -> &str {
        match &self.internal {
            RelativeUrlInternal::Static(url) => url.as_ref(),
            RelativeUrlInternal::Dynamic(url) => url.as_ref(),
        }
    }
}

impl PartialEq for RelativeUrl {
    fn eq(&self, other: &RelativeUrl) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl PartialEq<str> for RelativeUrl {
    fn eq(&self, s: &str) -> bool {
        self.as_ref() == s
    }
}

impl<'a> PartialEq<&'a str> for RelativeUrl {
    #[inline]
    fn eq(&self, s: &&'a str) -> bool {
        self == *s
    }
}

impl<'a> PartialEq<RelativeUrl> for &'a str {
    #[inline]
    fn eq(&self, url: &RelativeUrl) -> bool {
        url == self
    }
}

impl PartialEq<RelativeUrl> for str {
    #[inline]
    fn eq(&self, url: &RelativeUrl) -> bool {
        url == self
    }
}

impl fmt::Debug for RelativeUrl {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

impl fmt::Display for RelativeUrl {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_ref(), f)
    }
}

type_error_wrapper!(InvalidUrl);

fn query_pairs(query: Option<&str>) -> impl Iterator<Item = (&str, &str)> {
    form_urlencoded::parse(query.unwrap_or("").as_bytes()).map(|(k, v)| {
        let k = if let Cow::Borrowed(k) = k {
            k
        } else {
            unreachable!()
        };

        let v = if let Cow::Borrowed(v) = v {
            v
        } else {
            unreachable!()
        };

        (k, v)
    })
}
