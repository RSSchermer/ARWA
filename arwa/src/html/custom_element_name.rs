use std::fmt;

use arwa_parse::custom_element_name::InvalidCustomElementName;

use crate::console::{Argument, ToArgument};

#[doc(hidden)]
#[derive(Clone)]
pub struct StaticallyParsedCustomElementName {
    #[doc(hidden)]
    pub name: &'static str,
}

impl AsRef<str> for StaticallyParsedCustomElementName {
    fn as_ref(&self) -> &str {
        self.name
    }
}

#[derive(Clone)]
enum NameInternal {
    Static(StaticallyParsedCustomElementName),
    Dynamic(arwa_parse::custom_element_name::CustomElementName),
}

#[derive(Clone)]
pub struct CustomElementName {
    internal: NameInternal,
}

impl CustomElementName {
    pub fn parse(name: &str) -> Result<Self, InvalidCustomElementName> {
        arwa_parse::custom_element_name::CustomElementName::parse(name).map(|name| {
            CustomElementName {
                internal: NameInternal::Dynamic(name),
            }
        })
    }

    #[doc(hidden)]
    pub const fn from_statically_parsed(name: StaticallyParsedCustomElementName) -> Self {
        CustomElementName {
            internal: NameInternal::Static(name),
        }
    }
}

impl AsRef<str> for CustomElementName {
    fn as_ref(&self) -> &str {
        match &self.internal {
            NameInternal::Static(name) => name.as_ref(),
            NameInternal::Dynamic(name) => name.as_ref(),
        }
    }
}

impl ToArgument for CustomElementName {
    fn to_argument(&self) -> Argument {
        let as_str: &str = self.as_ref();

        ToArgument::to_argument(as_str)
    }
}

impl PartialEq for CustomElementName {
    fn eq(&self, other: &Self) -> bool {
        let self_as_str: &str = self.as_ref();
        let other_as_str: &str = other.as_ref();

        self_as_str == other_as_str
    }
}

impl PartialEq<str> for CustomElementName {
    fn eq(&self, other: &str) -> bool {
        let self_as_str: &str = self.as_ref();

        self_as_str == other
    }
}

impl PartialEq<&'_ str> for CustomElementName {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<CustomElementName> for &'_ str {
    #[inline]
    fn eq(&self, other: &CustomElementName) -> bool {
        other == self
    }
}

impl PartialEq<CustomElementName> for str {
    fn eq(&self, other: &CustomElementName) -> bool {
        let other_as_str: &str = other.as_ref();

        self == other_as_str
    }
}

impl fmt::Display for CustomElementName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Display::fmt(as_str, f)
    }
}

impl fmt::Debug for CustomElementName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Debug::fmt(as_str, f)
    }
}
