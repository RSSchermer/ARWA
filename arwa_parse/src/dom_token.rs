use std::error::Error;
use std::fmt;

#[derive(Clone)]
pub struct InvalidToken {
    token: String,
    invalid_pos: usize,
}

impl fmt::Display for InvalidToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.is_empty() {
            write!(f, "The empty string is not a valid DOM token.")
        } else {
            write!(
                f,
                "`{}` is not a valid DOM token; disallowed whitespace character at position `{}`.",
                &self.token, self.invalid_pos
            )
        }
    }
}

impl fmt::Debug for InvalidToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for InvalidToken {}

#[derive(Clone)]
pub struct Token {
    name: String,
}

impl Token {
    pub fn parse(token: &str) -> Result<Self, InvalidToken> {
        let mut chars = token.chars().enumerate();

        if let Some((_, c)) = chars.next() {
            if c.is_ascii_whitespace() {
                return Err(InvalidToken {
                    token: token.to_string(),
                    invalid_pos: 0,
                });
            }

            for (i, c) in chars {
                if c.is_ascii_whitespace() {
                    return Err(InvalidToken {
                        token: token.to_string(),
                        invalid_pos: i,
                    });
                }
            }

            Ok(Token {
                name: token.to_string(),
            })
        } else {
            Err(InvalidToken {
                token: token.to_string(),
                invalid_pos: 0,
            })
        }
    }

    pub fn trusted(name: String) -> Self {
        Token { name }
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        &self.name
    }
}
