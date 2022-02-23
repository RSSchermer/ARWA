use std::error::Error;
use std::fmt;

use wasm_bindgen::UnwrapThrowExt;

#[doc(hidden)]
pub struct StaticallyParsedLanguageTag {
    #[doc(hidden)]
    pub raw: &'static str,
    #[doc(hidden)]
    pub language_end: usize,
    #[doc(hidden)]
    pub extlang_end: usize,
    #[doc(hidden)]
    pub script_end: usize,
    #[doc(hidden)]
    pub region_end: usize,
    #[doc(hidden)]
    pub variant_end: usize,
    #[doc(hidden)]
    pub extension_end: usize,
}

impl StaticallyParsedLanguageTag {
    fn primary_language(&self) -> &str {
        &self.raw[..self.language_end]
    }

    fn extended_language(&self) -> Option<&str> {
        if self.language_end == self.extlang_end {
            None
        } else {
            Some(&self.raw[self.language_end + 1..self.extlang_end])
        }
    }

    fn full_language(&self) -> &str {
        &self.raw[..self.extlang_end]
    }

    fn script(&self) -> Option<&str> {
        if self.extlang_end == self.script_end {
            None
        } else {
            Some(&self.raw[self.extlang_end + 1..self.script_end])
        }
    }

    fn region(&self) -> Option<&str> {
        if self.script_end == self.region_end {
            None
        } else {
            Some(&self.raw[self.script_end + 1..self.region_end])
        }
    }

    fn variant(&self) -> Option<&str> {
        if self.region_end == self.variant_end {
            None
        } else {
            Some(&self.raw[self.region_end + 1..self.variant_end])
        }
    }

    fn extension(&self) -> Option<&str> {
        if self.variant_end == self.extension_end {
            None
        } else {
            Some(&self.raw[self.variant_end + 1..self.extension_end])
        }
    }

    fn private_use(&self) -> Option<&str> {
        if self.raw.starts_with("x-") {
            Some(&self.raw)
        } else if self.extension_end == self.raw.len() {
            None
        } else {
            Some(&self.raw[self.extension_end + 1..])
        }
    }
}

impl AsRef<str> for StaticallyParsedLanguageTag {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}

enum LanguageTagInternal {
    Dynamic(oxilangtag::LanguageTag<String>),
    Static(StaticallyParsedLanguageTag),
}

pub struct LanguageTag {
    internal: LanguageTagInternal,
}

impl LanguageTag {
    pub fn parse(language_tag: &str) -> Result<Self, InvalidLanguageTag> {
        oxilangtag::LanguageTag::parse(language_tag.to_string())
            .map(|ok| LanguageTag {
                internal: LanguageTagInternal::Dynamic(ok),
            })
            .map_err(|inner| InvalidLanguageTag { inner })
    }

    #[doc(hidden)]
    pub fn from_statically_parsed(parsed: StaticallyParsedLanguageTag) -> Self {
        LanguageTag {
            internal: LanguageTagInternal::Static(parsed),
        }
    }

    pub fn primary_language(&self) -> &str {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.primary_language(),
            LanguageTagInternal::Static(tag) => tag.primary_language(),
        }
    }

    pub fn extended_language(&self) -> Option<&str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.extended_language(),
            LanguageTagInternal::Static(tag) => tag.extended_language(),
        }
    }

    pub fn extended_language_subtags(&self) -> impl Iterator<Item = &str> {
        self.extended_language().unwrap_or("").split_terminator('-')
    }

    pub fn full_language(&self) -> &str {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.full_language(),
            LanguageTagInternal::Static(tag) => tag.full_language(),
        }
    }

    pub fn script(&self) -> Option<&str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.script(),
            LanguageTagInternal::Static(tag) => tag.script(),
        }
    }

    pub fn region(&self) -> Option<&str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.region(),
            LanguageTagInternal::Static(tag) => tag.region(),
        }
    }

    pub fn variant(&self) -> Option<&str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.variant(),
            LanguageTagInternal::Static(tag) => tag.variant(),
        }
    }

    pub fn variant_subtags(&self) -> impl Iterator<Item = &str> {
        self.variant().unwrap_or("").split_terminator('-')
    }

    pub fn extension(&self) -> Option<&str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.extension(),
            LanguageTagInternal::Static(tag) => tag.extension(),
        }
    }

    pub fn extension_subtags(&self) -> impl Iterator<Item = (char, &str)> {
        match self.extension() {
            Some(extension) => ExtensionsIterator::new(extension),
            None => ExtensionsIterator::new(""),
        }
    }

    pub fn private_use(&self) -> Option<&str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.private_use(),
            LanguageTagInternal::Static(tag) => tag.private_use(),
        }
    }

    pub fn private_use_subtags(&self) -> impl Iterator<Item = &str> {
        self.private_use()
            .map(|part| &part[2..])
            .unwrap_or("")
            .split_terminator('-')
    }
}

impl AsRef<str> for LanguageTag {
    fn as_ref(&self) -> &str {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.as_ref(),
            LanguageTagInternal::Static(tag) => tag.as_ref(),
        }
    }
}

impl PartialEq for LanguageTag {
    fn eq(&self, other: &LanguageTag) -> bool {
        let as_str: &str = self.as_ref();

        as_str == other.as_ref()
    }
}

impl PartialEq<str> for LanguageTag {
    fn eq(&self, s: &str) -> bool {
        self.as_ref() == s
    }
}

impl<'a> PartialEq<&'a str> for LanguageTag {
    #[inline]
    fn eq(&self, s: &&'a str) -> bool {
        self == *s
    }
}

impl<'a> PartialEq<LanguageTag> for &'a str {
    #[inline]
    fn eq(&self, url: &LanguageTag) -> bool {
        url == self
    }
}

impl PartialEq<LanguageTag> for str {
    #[inline]
    fn eq(&self, url: &LanguageTag) -> bool {
        url == self
    }
}

impl fmt::Debug for LanguageTag {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

impl fmt::Display for LanguageTag {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_ref(), f)
    }
}

pub struct InvalidLanguageTag {
    inner: oxilangtag::LanguageTagParseError,
}

impl fmt::Debug for InvalidLanguageTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl fmt::Display for InvalidLanguageTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl Error for InvalidLanguageTag {}

// Taken directly from https://docs.rs/oxilangtag/0.1.2/src/oxilangtag/lib.rs.html#234-239
struct ExtensionsIterator<'a> {
    input: &'a str,
}

impl<'a> ExtensionsIterator<'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Iterator for ExtensionsIterator<'a> {
    type Item = (char, &'a str);

    fn next(&mut self) -> Option<(char, &'a str)> {
        let mut parts_iterator = self.input.split_terminator('-');
        let singleton = parts_iterator.next()?.chars().next().unwrap_throw();
        let mut content_size: usize = 2;

        for part in parts_iterator {
            if part.len() == 1 {
                let content = &self.input[2..content_size - 1];

                self.input = &self.input[content_size..];

                return Some((singleton, content));
            } else {
                content_size += part.len() + 1;
            }
        }

        let result = self.input.get(2..).map(|content| (singleton, content));

        self.input = "";

        result
    }
}
