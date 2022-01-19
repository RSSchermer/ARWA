pub use url::Url as AbsoluteUrl;

#[derive(Clone, PartialEq)]
pub enum ContextualUrl<'a> {
    Absolute(&'a AbsoluteUrl),
    Relative(&'a RelativeUrl),
}

impl AsRef<str> for ContextualUrl {
    fn as_ref(&self) -> &str {
        match self {
            ContextualUrl::Absolute(url) => url.as_ref(),
            ContextualUrl::Relative(url) => url.as_ref(),
        }
    }
}

// TODO: implement
#[derive(Clone, PartialEq)]
pub struct RelativeUrl {}

impl AsRef<str> for RelativeUrl {
    fn as_ref(&self) -> &str {
        todo!()
    }
}

// TODO: figure out what to do about Object URLs.
