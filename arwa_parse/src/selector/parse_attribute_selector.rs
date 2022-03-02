use super::{
    parse_identifier, parse_identifier_or_string, AttributeMatcher, AttributeSelector,
    AttributeWithValue, CaseSensitivity, ParseError, Remainder,
};

pub fn parse_attribute_selector(
    remainder: Remainder,
) -> Result<(AttributeSelector, Remainder), ParseError> {
    let remainder = remainder.skip_whitespace();

    let (name, remainder) = parse_identifier(remainder)?;

    let remainder = remainder.skip_whitespace();

    if remainder.starts_with(']') {
        let attribute = AttributeSelector::Exists(name);
        let remainder = remainder.skip(1);

        return Ok((attribute, remainder));
    }

    let (skip, matcher) = if remainder.starts_with('=') {
        (1, AttributeMatcher::Exact)
    } else if remainder.starts_with("~=") {
        (2, AttributeMatcher::Includes)
    } else if remainder.starts_with("|=") {
        (2, AttributeMatcher::HyphenatedBeginsWidth)
    } else if remainder.starts_with("^=") {
        (2, AttributeMatcher::BeginsWith)
    } else if remainder.starts_with("$=") {
        (2, AttributeMatcher::EndsWith)
    } else if remainder.starts_with("*=") {
        (2, AttributeMatcher::Substring)
    } else {
        if let Some(c) = remainder.chars().next() {
            return Err(ParseError {
                message: format!(
                    "expected attribute selector terminator (`]`) or value matcher (`=`, `~=`, \
                    `|=`, `^=`,  `$=`, `*=`), found `{}`",
                    c
                ),
                offset: remainder.offset(),
            });
        } else {
            return Err(ParseError {
                message: "unexpected end; unclosed attribute selector".to_string(),
                offset: remainder.offset(),
            });
        }
    };

    let remainder = remainder.skip(skip);

    let (value, remainder) = parse_identifier_or_string(remainder)?;

    let mut case = CaseSensitivity::Default;
    let mut remainder = remainder.skip_whitespace();

    if remainder.starts_with(&['i', 'I', 's', 'S']) {
        case = if remainder.starts_with(&['i', 'I']) {
            CaseSensitivity::Insensitive
        } else {
            CaseSensitivity::Sensitive
        };

        remainder = remainder.skip(1).skip_whitespace();
    }

    if let Some(c) = remainder.chars().next() {
        if c == ']' {
            let attribute = AttributeWithValue {
                name,
                matcher,
                value,
                case_sensitivity: case,
            };

            Ok((AttributeSelector::WithValue(attribute), remainder.skip(1)))
        } else {
            Err(ParseError {
                message: format!(
                    "expected attribute selector terminator (`]`), found `{}`",
                    c
                ),
                offset: remainder.offset(),
            })
        }
    } else {
        return Err(ParseError {
            message: "unexpected end; unclosed attribute selector".to_string(),
            offset: remainder.offset(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_exists() {
        let (selector, remainder) = parse_attribute_selector("attr] rest".into()).unwrap();

        assert_eq!(selector, AttributeSelector::Exists(0..4));
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_exact() {
        let (selector, remainder) =
            parse_attribute_selector("attr=something] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..14,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_includes() {
        let (selector, remainder) =
            parse_attribute_selector("attr~=something] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Includes,
                value: 6..15,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_hyphenated_begins_with() {
        let (selector, remainder) =
            parse_attribute_selector("attr|=something] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::HyphenatedBeginsWidth,
                value: 6..15,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_begins_with() {
        let (selector, remainder) =
            parse_attribute_selector("attr^=something] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::BeginsWith,
                value: 6..15,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_end_with() {
        let (selector, remainder) =
            parse_attribute_selector("attr$=something] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::EndsWith,
                value: 6..15,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_substring() {
        let (selector, remainder) =
            parse_attribute_selector("attr*=something] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Substring,
                value: 6..15,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_with_single_quoted_string_value() {
        let (selector, remainder) =
            parse_attribute_selector("attr='something'] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..16,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_with_double_quoted_string_value() {
        let (selector, remainder) =
            parse_attribute_selector("attr=\"something\"] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..16,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_with_whitespace() {
        let (selector, remainder) =
            parse_attribute_selector(" attr = something ] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 1..5,
                matcher: AttributeMatcher::Exact,
                value: 8..17,
                case_sensitivity: CaseSensitivity::Default
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_identifier_with_whitespace_i() {
        let (selector, remainder) =
            parse_attribute_selector("attr=something i] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..14,
                case_sensitivity: CaseSensitivity::Insensitive
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_identifier_with_whitespace_I() {
        let (selector, remainder) =
            parse_attribute_selector("attr=something I] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..14,
                case_sensitivity: CaseSensitivity::Insensitive
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_identifier_with_whitespace_s() {
        let (selector, remainder) =
            parse_attribute_selector("attr=something s] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..14,
                case_sensitivity: CaseSensitivity::Sensitive
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_identifier_with_whitespace_S() {
        let (selector, remainder) =
            parse_attribute_selector("attr=something S] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..14,
                case_sensitivity: CaseSensitivity::Sensitive
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_string_with_i() {
        let (selector, remainder) =
            parse_attribute_selector("attr='something'i] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..16,
                case_sensitivity: CaseSensitivity::Insensitive
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_string_with_whitespace_i() {
        let (selector, remainder) =
            parse_attribute_selector("attr='something' i] rest".into()).unwrap();

        assert_eq!(
            selector,
            AttributeSelector::WithValue(AttributeWithValue {
                name: 0..4,
                matcher: AttributeMatcher::Exact,
                value: 5..16,
                case_sensitivity: CaseSensitivity::Insensitive
            })
        );
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn empty() {
        assert!(parse_attribute_selector("".into()).is_err())
    }

    #[test]
    fn unparsed_value_remainder() {
        assert!(parse_attribute_selector("attr=something something_else] rest".into()).is_err())
    }

    #[test]
    fn unclosed_attribute() {
        assert!(parse_attribute_selector("attr rest".into()).is_err())
    }

    #[test]
    fn unclosed_attribute_with_value() {
        assert!(parse_attribute_selector("attr=something rest".into()).is_err())
    }
}
