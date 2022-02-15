use std::fmt;

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

    fn extended_language_subtags(&self) -> impl Iterator<Item = &str> {
        self.extended_language().unwrap_or("").split_terminator('-')
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

    fn variant_subtags(&self) -> impl Iterator<Item = &str> {
        self.variant().unwrap_or("").split_terminator('-')
    }

    fn extension(&self) -> Option<&str> {
        if self.variant_end == self.extension_end {
            None
        } else {
            Some(&self.raw[self.variant_end + 1..self.extension_end])
        }
    }

    fn extension_subtags(&self) -> impl Iterator<Item = (char, &str)> {
        match self.extension() {
            Some(parts) => ExtensionsIterator::new(parts),
            None => ExtensionsIterator::new(""),
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

    fn private_use_subtags(&self) -> impl Iterator<Item = &str> {
        self.private_use()
            .map(|part| &part[2..])
            .unwrap_or("")
            .split_terminator('-')
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
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.extended_language_subtags(),
            LanguageTagInternal::Static(tag) => tag.extended_language_subtags(),
        }
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
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.variant_subtags(),
            LanguageTagInternal::Static(tag) => tag.variant_subtags(),
        }
    }

    pub fn extension(&self) -> Option<&str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.extension(),
            LanguageTagInternal::Static(tag) => tag.extension(),
        }
    }

    pub fn extension_subtags(&self) -> impl Iterator<Item = (char, &str)> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.extension_subtags(),
            LanguageTagInternal::Static(tag) => tag.extension_subtags(),
        }
    }

    pub fn private_use(&self) -> Option<&str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.private_use(),
            LanguageTagInternal::Static(tag) => tag.private_use(),
        }
    }

    pub fn private_use_subtags(&self) -> impl Iterator<Item = &str> {
        match &self.internal {
            LanguageTagInternal::Dynamic(tag) => tag.private_use_subtags(),
            LanguageTagInternal::Static(tag) => tag.private_use_subtags(),
        }
    }
}

impl PartialEq for LanguageTag {
    fn eq(&self, other: &LanguageTag) -> bool {
        self.as_ref() == other.as_ref()
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
