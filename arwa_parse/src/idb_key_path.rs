use std::error::Error;
use std::fmt;
use std::ops::{Add, Deref, Range};

pub struct InvalidKeyPath {
    key_path_string: String,
    error: ParseError,
}

impl fmt::Display for InvalidKeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` is not a valid key path: {} (at offset {})",
            &self.key_path_string, &self.error.message, self.error.offset
        )
    }
}

impl fmt::Debug for InvalidKeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for InvalidKeyPath {}

#[derive(Clone, PartialEq, Debug)]
struct ParseError {
    message: String,
    offset: Offset,
}

#[derive(Clone)]
pub struct IdbKeyPath {
    raw: String,
    identifiers: Vec<Range<usize>>,
}

impl IdbKeyPath {
    pub fn parse(key_path: &str) -> Result<IdbKeyPath, InvalidKeyPath> {
        parse_key_path(key_path).map_err(|error| InvalidKeyPath {
            key_path_string: key_path.to_string(),
            error,
        })
    }
}

impl AsRef<str> for IdbKeyPath {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}

fn parse_key_path(raw: &str) -> Result<IdbKeyPath, ParseError> {
    let (initial, mut remainder) = parse_identifier(raw.into())?;

    let mut identifiers = Vec::new();

    identifiers.push(initial);

    while remainder.len() > 0 {
        if remainder.starts_with('.') {
            remainder = remainder.skip(1);

            let (identifier, r) = parse_identifier(remainder)?;

            identifiers.push(identifier);
            remainder = r;
        } else {
            return Err(ParseError {
                message: "unexpected end; expected an identifier".to_string(),
                offset: remainder.offset(),
            });
        }
    }

    Ok(IdbKeyPath {
        raw: raw.to_string(),
        identifiers,
    })
}

fn parse_identifier(remainder: Remainder) -> Result<(Range<usize>, Remainder), ParseError> {
    let mut char_indices = remainder.char_indices();

    if let Some((_, c)) = char_indices.next() {
        if !is_identifier_start(c) {
            return Err(ParseError {
                message: format!("expected an identifier but found `{}`", c),
                offset: remainder.offset(),
            });
        }

        let mut identifier_len = 1;

        for (pos, c) in char_indices {
            if is_identifier_continue(c) {
                identifier_len = pos + 1;
            } else {
                break;
            }
        }

        Ok((
            remainder.offset().range(identifier_len),
            remainder.skip(identifier_len),
        ))
    } else {
        Err(ParseError {
            message: "unexpected end; expected an identifier".to_string(),
            offset: remainder.offset(),
        })
    }
}

fn is_identifier_start(c: char) -> bool {
    c > '\u{0080}' || c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    is_identifier_start(c) || c.is_ascii_digit() || c == '-'
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Offset(usize);

impl Offset {
    pub fn range(self, size: usize) -> Range<usize> {
        let end = self.0 + size;

        self.0..end
    }
}

impl Add<usize> for Offset {
    type Output = Offset;

    fn add(self, rhs: usize) -> Self::Output {
        Offset(self.0 + rhs)
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

struct Remainder<'a> {
    starting_len: usize,
    remainder: &'a str,
}

impl<'a> Remainder<'a> {
    fn offset(&self) -> Offset {
        Offset(self.starting_len - self.remainder.len())
    }

    fn skip(self, count: usize) -> Remainder<'a> {
        Remainder {
            starting_len: self.starting_len,
            remainder: &self.remainder[count..],
        }
    }
}

impl Deref for Remainder<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.remainder
    }
}

impl<'a> From<&'a str> for Remainder<'a> {
    fn from(remainder: &'a str) -> Self {
        Remainder {
            starting_len: remainder.len(),
            remainder,
        }
    }
}
