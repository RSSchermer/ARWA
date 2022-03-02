use std::fmt;
use std::ops::{Add, Deref, Range};

use super::{parse_selector_list, skip_whitespace, SelectorList};

#[derive(Clone, PartialEq, Debug)]
pub struct ParseError {
    pub message: String,
    pub offset: Offset,
}

pub fn parse(selector_string: &str) -> Result<SelectorList, ParseError> {
    let (selector, remainder) = parse_selector_list(selector_string.into())?;

    // Note: parse_selector will have consumed all trailing whitespace already.

    if let Some(c) = remainder.chars().next() {
        Err(ParseError {
            message: format!("unexpected character `{}`", c),
            offset: remainder.offset(),
        })
    } else {
        Ok(selector)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Offset(usize);

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Remainder<'a> {
    starting_len: usize,
    remainder: &'a str,
}

impl<'a> Remainder<'a> {
    pub fn offset(&self) -> Offset {
        Offset(self.starting_len - self.remainder.len())
    }

    pub fn skip(self, count: usize) -> Remainder<'a> {
        self.updated(&self.remainder[count..])
    }

    pub fn skip_whitespace(self) -> Remainder<'a> {
        self.updated(skip_whitespace(self.remainder))
    }

    fn updated(self, new_remainder: &str) -> Remainder {
        Remainder {
            starting_len: self.starting_len,
            remainder: new_remainder,
        }
    }
}

impl Deref for Remainder<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.remainder
    }
}

impl<'a> PartialEq<&'a str> for Remainder<'_> {
    fn eq(&self, other: &&'a str) -> bool {
        &self.remainder == other
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
