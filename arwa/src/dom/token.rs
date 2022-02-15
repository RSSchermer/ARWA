use std::borrow::Cow;
use std::ops::Deref;

#[derive(Clone)]
pub struct InvalidToken<'a>(Cow<'a, str>);

pub struct Token<'a>(Cow<'a, str>);

impl<'a> Token<'a> {
    pub const fn from_str(name: &'a str) -> Result<Token<'a>, InvalidToken<'a>> {
        let mut chars = name.chars();

        if let Some(c) = chars.next() {
            if is_whitespace(&c) {
                return Err(InvalidToken(name.into()));
            }

            for c in chars.next() {
                if is_whitespace(&c) {
                    return Err(InvalidToken(name.into()));
                }
            }

            Ok(Token(name.into()))
        } else {
            Err(InvalidToken(name.into()))
        }
    }
}

impl Deref for Token<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AsRef<str> for Token<'_> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'a> From<Token<'a>> for String {
    fn from(name: Token<'a>) -> Self {
        name.0.into()
    }
}

macro_rules! token {
    ($name:literal) => {{
        const TOKEN: &'static Token<'static> = &Token::from_str($name).unwrap();

        TOKEN
    }};
}

pub use token;

fn is_whitespace(c: &char) -> bool {
    c == &'\u{0009}' || c == &'\u{000A}' || c == &'\u{000C}' || c == &'\u{000D}' || c == &'\u{0020}'
}
