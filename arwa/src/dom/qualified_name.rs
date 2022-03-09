use std::fmt;

use crate::console::{Argument, ToArgument};

pub use arwa_parse::xml_name::InvalidQualifiedName;

#[doc(hidden)]
pub struct StaticallyParsedQualifiedName {
    #[doc(hidden)]
    pub name: &'static str,
    #[doc(hidden)]
    pub colon_pos: Option<usize>,
}

impl StaticallyParsedQualifiedName {
    pub fn prefix(&self) -> Option<&str> {
        self.colon_pos.map(|colon_pos| &self.name[0..colon_pos])
    }

    pub fn local_name(&self) -> &str {
        let start = self.colon_pos.unwrap_or(0);

        &self.name[start..]
    }
}

impl AsRef<str> for StaticallyParsedQualifiedName {
    fn as_ref(&self) -> &str {
        self.name
    }
}

enum QualifiedNameInternal {
    Static(StaticallyParsedQualifiedName),
    Dynamic(arwa_parse::xml_name::QualifiedName),
}

pub struct QualifiedName {
    internal: QualifiedNameInternal,
}

impl QualifiedName {
    pub fn parse(name: &str) -> Result<Self, InvalidQualifiedName> {
        arwa_parse::xml_name::QualifiedName::parse(name).map(|name| QualifiedName {
            internal: QualifiedNameInternal::Dynamic(name),
        })
    }

    #[doc(hidden)]
    pub fn from_statically_parsed(name: StaticallyParsedQualifiedName) -> Self {
        QualifiedName {
            internal: QualifiedNameInternal::Static(name),
        }
    }

    pub fn prefix(&self) -> Option<&str> {
        match &self.internal {
            QualifiedNameInternal::Static(name) => name.prefix(),
            QualifiedNameInternal::Dynamic(name) => name.prefix(),
        }
    }

    pub fn local_name(&self) -> &str {
        match &self.internal {
            QualifiedNameInternal::Static(name) => name.local_name(),
            QualifiedNameInternal::Dynamic(name) => name.local_name(),
        }
    }
}

impl AsRef<str> for QualifiedName {
    fn as_ref(&self) -> &str {
        match &self.internal {
            QualifiedNameInternal::Static(name) => name.as_ref(),
            QualifiedNameInternal::Dynamic(name) => name.as_ref(),
        }
    }
}

impl ToArgument for QualifiedName {
    fn to_argument(&self) -> Argument {
        let as_str: &str = self.as_ref();

        ToArgument::to_argument(as_str)
    }
}

impl PartialEq for QualifiedName {
    fn eq(&self, other: &Self) -> bool {
        let self_as_str: &str = self.as_ref();
        let other_as_str: &str = other.as_ref();

        self_as_str == other_as_str
    }
}

impl PartialEq<str> for QualifiedName {
    fn eq(&self, other: &str) -> bool {
        let self_as_str: &str = self.as_ref();

        self_as_str == other
    }
}

impl PartialEq<QualifiedName> for str {
    fn eq(&self, other: &QualifiedName) -> bool {
        let other_as_str: &str = other.as_ref();

        self == other_as_str
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Display::fmt(as_str, f)
    }
}

impl fmt::Debug for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Debug::fmt(as_str, f)
    }
}
