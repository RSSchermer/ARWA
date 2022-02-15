use std::borrow::Cow;
use std::ops::Deref;

pub use arwa_parse::xml_name::InvalidName;

#[doc(hidden)]
pub struct StaticallyParsedName {
    #[doc(hidden)]
    pub name: &'static str,
}

impl AsRef<str> for StaticallyParsedName {
    fn as_ref(&self) -> &str {
        self.name
    }
}

enum NameInternal {
    Static(StaticallyParsedName),
    Dynamic(arwa_parse::xml_name::Name),
}

pub struct Name {
    internal: NameInternal,
}

impl Name {
    pub fn parse(name: &str) -> Result<Self, InvalidName> {
        arwa_parse::xml_name::Name::parse(name).map(|name| Name {
            internal: NameInternal::Dynamic(name),
        })
    }

    #[doc(hidden)]
    pub fn from_statically_parsed_name(name: StaticallyParsedName) -> Self {
        Name {
            internal: NameInternal::Static(name),
        }
    }

    pub(crate) fn trusted(name: String) -> Self {
        Name {
            internal: NameInternal::Dynamic(arwa_parse::xml_name::Name::trusted(name)),
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        match &self.internal {
            NameInternal::Static(name) => name.as_ref(),
            NameInternal::Dynamic(name) => name.as_ref(),
        }
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        let self_as_str: &str = self.as_ref();
        let other_as_str: &str = other.as_ref();

        self_as_str == other_as_str
    }
}

impl PartialEq<str> for Name {
    fn eq(&self, other: &str) -> bool {
        let self_as_str: &str = self.as_ref();

        self_as_str == other
    }
}

impl PartialEq<Name> for str {
    fn eq(&self, other: &Name) -> bool {
        let other_as_str: &str = other.as_ref();

        self == other_as_str
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Display::fmt(as_str, f)
    }
}

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Debug::fmt(as_str, f)
    }
}

#[derive(Clone)]
pub struct InvalidQualifiedName<'a>(Cow<'a, str>);

pub struct QualifiedName<'a> {
    name: Cow<'a, str>,
    colon_pos: Option<usize>,
}

impl<'a> QualifiedName<'a> {
    pub fn new(prefix: &NonColonName, local_name: &NonColonName) -> Self {
        let mut name = String::with_capacity(prefix.len() + local_name.len() + 1);

        name.push_str(prefix);
        name.push(':');
        name.push_str(local_name);

        QualifiedName {
            name: name.into(),
            colon_pos: Some(prefix.len()),
        }
    }

    pub const fn from_local_name(local_name: NonColonName) -> Self {
        QualifiedName {
            name: local_name.0,
            colon_pos: None,
        }
    }

    pub const fn from_str(name: &'a str) -> Result<QualifiedName<'a>, InvalidQualifiedName<'a>> {
        let mut chars = name.chars();
        let mut colon_pos = None;

        if let Some(c) = chars.next() {
            if !valid_first_char(&c) {
                return Err(InvalidQualifiedName(name.into()));
            }

            if c == ':' {
                colon_pos = Some(0);
            }

            for (pos, c) in chars.enumerate().next() {
                if !valid_tail_char(&c) {
                    return Err(InvalidQualifiedName(name.into()));
                }

                if c == ':' {
                    if colon_pos.is_some() {
                        return Err(InvalidQualifiedName(name.into()));
                    } else {
                        colon_pos = Some(pos + 1);
                    }
                }
            }

            Ok(QualifiedName {
                name: name.into(),
                colon_pos,
            })
        } else {
            Err(InvalidQualifiedName(name.into()))
        }
    }

    pub fn prefix(&self) -> Option<NonColonName> {
        if let Some(colon_pos) = self.colon_pos {
            Some(NonColonName(self.name[0..colon_pos].into()))
        } else {
            None
        }
    }

    pub fn local_name(&self) -> NonColonName {
        let start = self.colon_pos.unwrap_or(0);

        NonColonName(self.name[start..].into())
    }
}

impl Deref for QualifiedName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AsRef<str> for QualifiedName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl ToString for QualifiedName {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl<'a> From<QualifiedName<'a>> for String {
    fn from(name: QualifiedName<'a>) -> Self {
        name.0.into()
    }
}

impl<'a> From<NonColonName<'a>> for QualifiedName<'a> {
    fn from(local_name: NonColonName<'a>) -> Self {
        QualifiedName {
            name: local_name.0,
            colon_pos: None,
        }
    }
}

macro_rules! qualified_name {
    ($name:literal) => {{
        const NAME: &'static QualifiedName<'static> = &QualifiedName::from_str($name).unwrap();

        NAME
    }};
}

use bitflags::_core::borrow::Borrow;
pub use qualified_name;
use std::fmt;

const fn valid_first_char(c: &char) -> bool {
    c == &'_'
        || c == &':'
        || ('a'..='z').contains(c)
        || ('A'..='Z').contains(c)
        || ('\u{00C0}'..='\u{00D6}').contains(c)
        || ('\u{00D8}'..='\u{00F6}').contains(c)
        || ('\u{00F8}'..='\u{02FF}').contains(c)
        || ('\u{0370}'..='\u{037D}').contains(c)
        || ('\u{037F}'..='\u{1FFF}').contains(c)
        || ('\u{200C}'..='\u{200D}').contains(c)
        || ('\u{2070}'..='\u{218F}').contains(c)
        || ('\u{2C00}'..='\u{2FEF}').contains(c)
        || ('\u{3001}'..='\u{D7FF}').contains(c)
        || ('\u{F900}'..='\u{FDCF}').contains(c)
        || ('\u{FDF0}'..='\u{FFFD}').contains(c)
        || ('\u{10000}'..='\u{EFFFF}').contains(c)
}

const fn valid_tail_char(c: &char) -> bool {
    valid_first_char(c)
        || c == &'-'
        || c == &'.'
        || c == '\u{00B7}'
        || ('0'..='9').contains(c)
        || ('\u{0300}'..='\u{036F}').contains(c)
        || ('\u{203F}'..='\u{2040}').contains(c)
}
