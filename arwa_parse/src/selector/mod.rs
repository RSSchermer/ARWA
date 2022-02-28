mod parse;
use self::parse::*;

mod parse_a_n_plus_b;
use self::parse_a_n_plus_b::*;

mod parse_attribute_selector;
use self::parse_attribute_selector::*;

mod parse_complex_selector;
use self::parse_complex_selector::*;

mod parse_compound_selector;
use self::parse_compound_selector::*;

mod parse_function_invocation;
use self::parse_function_invocation::*;

mod parse_identifier;
use self::parse_identifier::*;

mod parse_identifier_or_string;
use self::parse_identifier_or_string::*;

mod parse_pseudo_selector;
use self::parse_pseudo_selector::*;

mod parse_relative_complex_selector;
use self::parse_relative_complex_selector::*;

mod parse_relative_selector_list;
use self::parse_relative_selector_list::*;

mod parse_selector_list;
use self::parse_selector_list::*;

mod parse_string;
use self::parse_string::*;

mod util;
use self::util::*;

mod parsed;
pub use self::parsed::*;

use std::error::Error;
use std::fmt;
use std::ops::Deref;

pub struct InvalidSelector {
    selector_string: String,
    error: ParseError,
}

impl fmt::Display for InvalidSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` is not a valid selector: {} (at position {})",
            &self.selector_string, &self.error.message, self.error.position
        )
    }
}

impl fmt::Debug for InvalidSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for InvalidSelector {}

pub struct Selector {
    raw: String,
    parsed: SelectorList,
}

impl Selector {
    pub fn parse(selector: &str) -> Result<Selector, InvalidSelector> {
        parse(selector)
            .map(|parsed| Selector {
                raw: selector.to_string(),
                parsed,
            })
            .map_err(|error| InvalidSelector {
                selector_string: selector.to_string(),
                error,
            })
    }
}

impl Deref for Selector {
    type Target = [ComplexSelector];

    fn deref(&self) -> &Self::Target {
        &self.parsed.selector_list
    }
}

impl AsRef<str> for Selector {
    fn as_ref(&self) -> &str {
        &self.raw
    }
}
