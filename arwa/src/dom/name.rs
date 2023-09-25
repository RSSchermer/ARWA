use std::fmt;

pub use arwa_parse::xml_name::InvalidName;

use crate::console::{Argument, ToArgument};

#[doc(hidden)]
#[derive(Clone)]
pub struct StaticallyParsedName {
    #[doc(hidden)]
    pub name: &'static str,
}

impl AsRef<str> for StaticallyParsedName {
    fn as_ref(&self) -> &str {
        self.name
    }
}

#[derive(Clone)]
enum NameInternal {
    Static(StaticallyParsedName),
    Dynamic(arwa_parse::xml_name::Name),
}

#[derive(Clone)]
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
    pub const fn from_statically_parsed(name: StaticallyParsedName) -> Self {
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

impl ToArgument for Name {
    fn to_argument(&self) -> Argument {
        let as_str: &str = self.as_ref();

        ToArgument::to_argument(as_str)
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

impl PartialEq<&'_ str> for Name {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<Name> for &'_ str {
    #[inline]
    fn eq(&self, other: &Name) -> bool {
        other == self
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
