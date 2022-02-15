// We essentially just wrap the mime_4::MediaType type so we can slightly adjust the interface for
// consistency with other parsed string types (url, lang, etc.), in particular the
// `MediaType::parse` signature, which uses a sealed `Parse` trait for no obvious reason (to me at
// least, maybe a legacy reason?).

pub use mime_4::Value;

#[doc(hidden)]
pub use mime_4::media_type as mime_4_media_type;

pub struct MediaType {
    inner: mime_4::MediaType,
}

impl MediaType {
    pub fn parse(media_type: &str) -> Result<MediaType, InvalidMediaType> {
        mime_4::MediaType::parse(media_type)
            .map(|inner| MediaType { inner })
            .map_err(|inner| InvalidMediaType { inner })
    }

    /// This function is not part of the public interface, only meant to be called by the
    /// accompanying `media_type` macro.
    #[doc(hidden)]
    pub fn from_mime_4(inner: mime_4::MediaType) -> Self {
        MediaType { inner }
    }

    pub fn base_type(&self) -> &str {
        self.inner.type_()
    }

    pub fn sub_type(&self) -> &str {
        self.inner.subtype()
    }

    pub fn suffix(&self) -> Option<&str> {
        self.inner.suffix()
    }

    pub fn has_params(&self) -> bool {
        self.inner.has_params()
    }

    pub fn param(&self, name: &str) -> Option<Value> {
        self.inner.param(name)
    }

    pub fn params(&self) -> impl Iterator<Item = (&str, Value)> {
        self.inner.params()
    }

    pub fn without_params(&self) -> Self {
        MediaType {
            inner: self.inner.without_params(),
        }
    }
}

impl AsRef<str> for MediaType {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}

impl PartialEq for MediaType {
    fn eq(&self, other: &MediaType) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl PartialEq<str> for MediaType {
    fn eq(&self, s: &str) -> bool {
        self.inner.eq(s)
    }
}

impl<'a> PartialEq<&'a str> for MediaType {
    #[inline]
    fn eq(&self, s: &&'a str) -> bool {
        self == *s
    }
}

impl<'a> PartialEq<MediaType> for &'a str {
    #[inline]
    fn eq(&self, mt: &MediaType) -> bool {
        mt == self
    }
}

impl PartialEq<MediaType> for str {
    #[inline]
    fn eq(&self, mt: &MediaType) -> bool {
        mt == self
    }
}

impl fmt::Debug for MediaType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

impl fmt::Display for MediaType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

pub struct InvalidMediaType {
    inner: mime_4::InvalidMime,
}

macro_rules! media_type {
    ($media_type:literal) => {
        MediaType::from_mime_4($crate::media_type::mime_4_media_type!($media_type))
    };
}

pub use media_type;
use std::fmt;
