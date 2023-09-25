use std::borrow::Cow;
use std::fmt;
use std::ops::Range;
use std::str::FromStr;

pub use arwa_macro::url;
use lazycell::LazyCell;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

use crate::type_error_wrapper;

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
    pub path_start: usize,
    #[doc(hidden)]
    pub query_start: Option<usize>,
    #[doc(hidden)]
    pub fragment_start: Option<usize>,
    #[doc(hidden)]
    pub origin_kind: OriginKind,
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

    fn path(&self) -> &str {
        match (self.query_start, self.fragment_start) {
            (None, None) => &self.raw[self.path_start..],
            (Some(next_component_start), _) | (None, Some(next_component_start)) => {
                &self.raw[self.path_start..next_component_start]
            }
        }
    }

    fn query(&self) -> Option<&str> {
        match (self.query_start, self.fragment_start) {
            (None, _) => None,
            (Some(query_start), None) => Some(&self.raw[query_start..]),
            (Some(query_start), Some(fragment_start)) => {
                Some(&self.raw[query_start..fragment_start])
            }
        }
    }

    fn fragment(&self) -> Option<&str> {
        self.fragment_start.map(|start| &self.raw[start..])
    }

    fn origin(&self) -> Origin {
        match self.origin_kind {
            OriginKind::Opaque => Origin::Opaque,
            OriginKind::Tuple => {
                let scheme = self.scheme();
                let host = self.host().unwrap_throw();
                let port = self.port_or_known_default().unwrap_throw();

                Origin::Tuple(scheme, host, port)
            }
        }
    }
}

impl AsRef<str> for StaticallyParsedUrl {
    fn as_ref(&self) -> &str {
        self.raw
    }
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub enum OriginKind {
    Opaque,
    Tuple,
}

#[derive(Clone)]
struct ParsedDynamicCache {
    scheme: LazyCell<String>,
    username: LazyCell<String>,
    password: LazyCell<String>,
    host: LazyCell<String>,
    port: LazyCell<Option<u16>>,
    path: LazyCell<String>,
    query: LazyCell<String>,
    fragment: LazyCell<String>,
    origin: LazyCell<OriginKind>,
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

#[derive(Clone)]
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

    fn path(&self) -> &str {
        self.cache.path.borrow_with(|| self.parsed.pathname())
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
                OriginKind::Opaque
            } else {
                OriginKind::Tuple
            }
        });

        match origin {
            OriginKind::Opaque => Origin::Opaque,
            OriginKind::Tuple => {
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

#[derive(Clone)]
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

#[derive(Clone)]
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

    pub fn parse_with_base(relative: &str, base: &Url) -> Result<Self, InvalidUrl> {
        web_sys::Url::new_with_base(relative, &base.to_string())
            .map(|parsed| Url {
                internal: DynamicallyParsedUrl {
                    raw: parsed.href(),
                    parsed,
                    cache: ParsedDynamicCache::uninitialized(),
                }
                .into(),
            })
            .map_err(|err| InvalidUrl::new(err.unchecked_into()))
    }

    /// Only meant to be called by the accompanying proc-macro, not part of the public API.
    #[doc(hidden)]
    pub const fn from_statically_parsed(parsed: StaticallyParsedUrl) -> Self {
        Url {
            internal: UrlInternal::Static(parsed),
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

    pub fn path(&self) -> &str {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.path(),
            UrlInternal::Static(url) => url.path(),
        }
    }

    pub fn path_segments(&self) -> impl Iterator<Item = &str> {
        let path = self.path();

        if path.starts_with('/') {
            path[1..].split('/')
        } else {
            path.split('/')
        }
    }

    pub fn query(&self) -> Option<&str> {
        match &self.internal {
            UrlInternal::Dynamic(url) => url.query(),
            UrlInternal::Static(url) => url.query(),
        }
    }

    pub fn query_pairs(&self) -> impl Iterator<Item = (Cow<str>, Cow<str>)> {
        form_urlencoded::parse(self.query().map(|q| &q[1..]).unwrap_or("").as_bytes())
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

impl PartialEq<&'_ str> for Url {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<Url> for &'_ str {
    #[inline]
    fn eq(&self, other: &Url) -> bool {
        other == self
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

type_error_wrapper!(InvalidUrl);
