use std::fmt;

use crate::console::{Argument, ToArgument};

pub use arwa_parse::dom_token::InvalidToken;

#[doc(hidden)]
#[derive(Clone)]
pub struct StaticallyParsedToken {
    #[doc(hidden)]
    pub token: &'static str,
}

impl AsRef<str> for StaticallyParsedToken {
    fn as_ref(&self) -> &str {
        self.token
    }
}

#[derive(Clone)]
enum TokenInternal {
    Static(StaticallyParsedToken),
    Dynamic(arwa_parse::dom_token::Token),
}

#[derive(Clone)]
pub struct Token {
    internal: TokenInternal,
}

impl Token {
    pub fn parse(token: &str) -> Result<Self, InvalidToken> {
        arwa_parse::dom_token::Token::parse(token).map(|token| Token {
            internal: TokenInternal::Dynamic(token),
        })
    }

    #[doc(hidden)]
    pub const fn from_statically_parsed(token: StaticallyParsedToken) -> Self {
        Token {
            internal: TokenInternal::Static(token),
        }
    }

    pub(crate) fn trusted(token: String) -> Self {
        Token {
            internal: TokenInternal::Dynamic(arwa_parse::dom_token::Token::trusted(token)),
        }
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        match &self.internal {
            TokenInternal::Static(token) => token.as_ref(),
            TokenInternal::Dynamic(token) => token.as_ref(),
        }
    }
}

impl ToArgument for Token {
    fn to_argument(&self) -> Argument {
        let as_str: &str = self.as_ref();

        ToArgument::to_argument(as_str)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        let self_as_str: &str = self.as_ref();
        let other_as_str: &str = other.as_ref();

        self_as_str == other_as_str
    }
}

impl PartialEq<str> for Token {
    fn eq(&self, other: &str) -> bool {
        let self_as_str: &str = self.as_ref();

        self_as_str == other
    }
}

impl PartialEq<&'_ str> for Token {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<Token> for &'_ str {
    #[inline]
    fn eq(&self, other: &Token) -> bool {
        other == self
    }
}

impl PartialEq<Token> for str {
    fn eq(&self, other: &Token) -> bool {
        let other_as_str: &str = other.as_ref();

        self == other_as_str
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Display::fmt(as_str, f)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Debug::fmt(as_str, f)
    }
}
