use std::fmt;

pub use arwa_macro::idb_key_path as key_path;
use arwa_parse::idb_key_path::IdbKeyPath as DynamicallyParsedKeyPath;
pub use arwa_parse::idb_key_path::InvalidKeyPath;

use crate::console::{Argument, ToArgument};

#[doc(hidden)]
#[derive(Clone)]
pub struct StaticallyParsedKeyPath {
    #[doc(hidden)]
    pub key_path: &'static str,
}

impl AsRef<str> for StaticallyParsedKeyPath {
    fn as_ref(&self) -> &str {
        self.key_path
    }
}

#[derive(Clone)]
enum KeyPathInternal {
    Static(StaticallyParsedKeyPath),
    Dynamic(DynamicallyParsedKeyPath),
}

#[derive(Clone)]
pub struct KeyPath {
    internal: KeyPathInternal,
}

impl KeyPath {
    pub fn parse(key_path: &str) -> Result<Self, InvalidKeyPath> {
        DynamicallyParsedKeyPath::parse(key_path).map(|key_path| KeyPath {
            internal: KeyPathInternal::Dynamic(key_path),
        })
    }

    #[doc(hidden)]
    pub const fn from_statically_parsed(key_path: StaticallyParsedKeyPath) -> Self {
        KeyPath {
            internal: KeyPathInternal::Static(key_path),
        }
    }
}

impl AsRef<str> for KeyPath {
    fn as_ref(&self) -> &str {
        match &self.internal {
            KeyPathInternal::Static(key_path) => key_path.as_ref(),
            KeyPathInternal::Dynamic(key_path) => key_path.as_ref(),
        }
    }
}

impl ToArgument for KeyPath {
    fn to_argument(&self) -> Argument {
        let as_str: &str = self.as_ref();

        ToArgument::to_argument(as_str)
    }
}

impl PartialEq for KeyPath {
    fn eq(&self, other: &Self) -> bool {
        let self_as_str: &str = self.as_ref();
        let other_as_str: &str = other.as_ref();

        self_as_str == other_as_str
    }
}

impl PartialEq<str> for KeyPath {
    fn eq(&self, other: &str) -> bool {
        let self_as_str: &str = self.as_ref();

        self_as_str == other
    }
}

impl PartialEq<&'_ str> for KeyPath {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        self == *other
    }
}

impl PartialEq<KeyPath> for &'_ str {
    #[inline]
    fn eq(&self, key_path: &KeyPath) -> bool {
        key_path == self
    }
}

impl PartialEq<KeyPath> for str {
    fn eq(&self, other: &KeyPath) -> bool {
        let other_as_str: &str = other.as_ref();

        self == other_as_str
    }
}

impl fmt::Display for KeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Display::fmt(as_str, f)
    }
}

impl fmt::Debug for KeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Debug::fmt(as_str, f)
    }
}
