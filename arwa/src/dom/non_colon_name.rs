use std::fmt;

use arwa_parse::xml_name;

use crate::console::{Argument, ToArgument};

pub use arwa_parse::xml_name::InvalidNonColonName;

#[doc(hidden)]
#[derive(Clone)]
pub struct StaticallyParsedNonColonName {
    #[doc(hidden)]
    pub name: &'static str,
}

impl AsRef<str> for StaticallyParsedNonColonName {
    fn as_ref(&self) -> &str {
        self.name
    }
}

#[derive(Clone)]
enum NonColonNameInternal {
    Static(StaticallyParsedNonColonName),
    Dynamic(xml_name::NonColonName),
}

#[derive(Clone)]
pub struct NonColonName {
    internal: NonColonNameInternal,
}

impl NonColonName {
    pub fn parse(name: &str) -> Result<Self, InvalidNonColonName> {
        xml_name::NonColonName::parse(name).map(|name| NonColonName {
            internal: NonColonNameInternal::Dynamic(name),
        })
    }

    #[doc(hidden)]
    pub const fn from_statically_parsed(name: StaticallyParsedNonColonName) -> Self {
        NonColonName {
            internal: NonColonNameInternal::Static(name),
        }
    }

    pub(crate) fn trusted(name: String) -> Self {
        NonColonName {
            internal: NonColonNameInternal::Dynamic(xml_name::NonColonName::trusted(name)),
        }
    }
}

impl AsRef<str> for NonColonName {
    fn as_ref(&self) -> &str {
        match &self.internal {
            NonColonNameInternal::Static(name) => name.as_ref(),
            NonColonNameInternal::Dynamic(name) => name.as_ref(),
        }
    }
}

impl ToArgument for NonColonName {
    fn to_argument(&self) -> Argument {
        let as_str: &str = self.as_ref();

        ToArgument::to_argument(as_str)
    }
}

impl PartialEq for NonColonName {
    fn eq(&self, other: &Self) -> bool {
        let self_as_str: &str = self.as_ref();
        let other_as_str: &str = other.as_ref();

        self_as_str == other_as_str
    }
}

impl PartialEq<str> for NonColonName {
    fn eq(&self, other: &str) -> bool {
        let self_as_str: &str = self.as_ref();

        self_as_str == other
    }
}

impl PartialEq<&'_ str> for NonColonName {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<NonColonName> for &'_ str {
    #[inline]
    fn eq(&self, other: &NonColonName) -> bool {
        other == self
    }
}

impl PartialEq<NonColonName> for str {
    fn eq(&self, other: &NonColonName) -> bool {
        let other_as_str: &str = other.as_ref();

        self == other_as_str
    }
}

impl fmt::Display for NonColonName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Display::fmt(as_str, f)
    }
}

impl fmt::Debug for NonColonName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Debug::fmt(as_str, f)
    }
}
