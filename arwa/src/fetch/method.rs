use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct RequestMethod<'a> {
    method: Cow<'a, str>,
}

impl RequestMethod<'static> {
    pub(crate) fn from_string_unchecked(string: String) -> Self {
        RequestMethod {
            method: string.into(),
        }
    }

    pub const GET: RequestMethod<'static> = RequestMethod {
        method: Cow::Borrowed("GET"),
    };

    pub const HEAD: RequestMethod<'static> = RequestMethod {
        method: Cow::Borrowed("HEAD"),
    };

    pub const OPTIONS: RequestMethod<'static> = RequestMethod {
        method: Cow::Borrowed("OPTIONS"),
    };

    pub const POST: RequestMethod<'static> = RequestMethod {
        method: Cow::Borrowed("POST"),
    };

    pub const PUT: RequestMethod<'static> = RequestMethod {
        method: Cow::Borrowed("PUT"),
    };

    pub const PATCH: RequestMethod<'static> = RequestMethod {
        method: Cow::Borrowed("PATCH"),
    };

    pub const CONNECT: RequestMethod<'static> = RequestMethod {
        method: Cow::Borrowed("CONNECT"),
    };

    pub const TRACE: RequestMethod<'static> = RequestMethod {
        method: Cow::Borrowed("TRACE"),
    };
}

impl<'a> TryFrom<&'a str> for RequestMethod<'a> {
    type Error = InvalidMethodToken;

    fn try_from(token: &'a str) -> Result<Self, Self::Error> {
        fn valid_char(c: &char) -> bool {
            ('0'..='9').contains(c)
                || ('a'..='z').contains(c)
                || ('A'..='Z').contains(c)
                || "!#$%&'*+-.^_`|~".contains(c)
        }

        if token.is_empty() {
            return Err(InvalidMethodToken(token.to_string()));
        }

        for c in token.chars() {
            if !valid_char(&c) {
                return Err(InvalidMethodToken(token.to_string()));
            }
        }

        Ok(RequestMethod {
            method: token.into(),
        })
    }
}

impl AsRef<str> for RequestMethod<'_> {
    fn as_ref(&self) -> &str {
        self.method.as_ref()
    }
}

impl Default for RequestMethod<'static> {
    fn default() -> Self {
        RequestMethod::GET
    }
}

impl fmt::Debug for RequestMethod<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.method.fmt(f)
    }
}

pub struct InvalidMethodToken(String);

#[cfg(test)]
mod tests {
    use crate::fetch::RequestMethod;
    use std::convert::TryFrom;

    #[test]
    fn method_from_valid_str() {
        assert!(RequestMethod::try_from("019abzABZ!#$%&'*+-.^_`|~").is_ok());
    }

    #[test]
    fn method_from_invalid_str() {
        assert!(RequestMethod::try_from("019abzABZ!#$%&'*+-.^_`|~?").is_err());
    }
}
