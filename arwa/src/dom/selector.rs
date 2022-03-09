use std::fmt;

use arwa_parse::selector::Selector as DynamicallyParsedSelector;

use crate::console::{Argument, ToArgument};

pub use arwa_parse::selector::InvalidSelector;

#[doc(hidden)]
pub struct StaticallyParsedSelector {
    #[doc(hidden)]
    pub selector: &'static str,
}

impl AsRef<str> for StaticallyParsedSelector {
    fn as_ref(&self) -> &str {
        self.selector
    }
}

enum SelectorInternal {
    Static(StaticallyParsedSelector),
    Dynamic(DynamicallyParsedSelector),
}

pub struct Selector {
    internal: SelectorInternal,
}

impl Selector {
    pub fn parse(selector: &str) -> Result<Self, InvalidSelector> {
        DynamicallyParsedSelector::parse(selector).map(|selector| Selector {
            internal: SelectorInternal::Dynamic(selector),
        })
    }

    #[doc(hidden)]
    pub fn from_statically_parsed(selector: StaticallyParsedSelector) -> Self {
        Selector {
            internal: SelectorInternal::Static(selector),
        }
    }
}

impl AsRef<str> for Selector {
    fn as_ref(&self) -> &str {
        match &self.internal {
            SelectorInternal::Static(selector) => selector.as_ref(),
            SelectorInternal::Dynamic(selector) => selector.as_ref(),
        }
    }
}

impl ToArgument for Selector {
    fn to_argument(&self) -> Argument {
        let as_str: &str = self.as_ref();

        ToArgument::to_argument(as_str)
    }
}

impl PartialEq for Selector {
    fn eq(&self, other: &Self) -> bool {
        let self_as_str: &str = self.as_ref();
        let other_as_str: &str = other.as_ref();

        self_as_str == other_as_str
    }
}

impl PartialEq<str> for Selector {
    fn eq(&self, other: &str) -> bool {
        let self_as_str: &str = self.as_ref();

        self_as_str == other
    }
}

impl PartialEq<Selector> for str {
    fn eq(&self, other: &Selector) -> bool {
        let other_as_str: &str = other.as_ref();

        self == other_as_str
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Display::fmt(as_str, f)
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_str: &str = self.as_ref();

        fmt::Debug::fmt(as_str, f)
    }
}
