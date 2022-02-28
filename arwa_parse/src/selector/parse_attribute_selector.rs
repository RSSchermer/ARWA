use super::{
    parse_identifier, parse_identifier_or_string, skip_whitespace, AttributeMatcher,
    AttributeSelector, AttributeWithValue, CaseSensitivity, ParseError,
};

pub fn parse_attribute_selector(
    input_remainder: &str,
    offset: usize,
) -> Result<(AttributeSelector, &str), ParseError> {
    let remainder = skip_whitespace(input_remainder);

    let (name, remainder) =
        parse_identifier(remainder, offset + input_remainder.len() - remainder.len())?;

    let remainder = skip_whitespace(remainder);

    if remainder.starts_with(']') {
        let attribute = AttributeSelector::Exists(name);
        let remainder = &remainder[1..];

        return Ok((attribute, remainder));
    }

    let (value, matcher, remainder) = if remainder.starts_with('=') {
        let (value, remainder) = parse_identifier_or_string(
            &remainder[1..],
            offset + input_remainder.len() - remainder.len() + 1,
        )?;

        (value, AttributeMatcher::Exact, remainder)
    } else if remainder.starts_with("~=") {
        let (value, remainder) = parse_identifier_or_string(
            &remainder[2..],
            offset + input_remainder.len() - remainder.len() + 2,
        )?;

        (value, AttributeMatcher::Includes, remainder)
    } else if remainder.starts_with("|=") {
        let (value, remainder) = parse_identifier_or_string(
            &remainder[2..],
            offset + input_remainder.len() - remainder.len() + 2,
        )?;

        (value, AttributeMatcher::HyphenatedBeginsWidth, remainder)
    } else if remainder.starts_with("^=") {
        let (value, remainder) = parse_identifier_or_string(
            &remainder[2..],
            offset + input_remainder.len() - remainder.len() + 2,
        )?;

        (value, AttributeMatcher::BeginsWith, remainder)
    } else if remainder.starts_with("$=") {
        let (value, remainder) = parse_identifier_or_string(
            &remainder[2..],
            offset + input_remainder.len() - remainder.len() + 2,
        )?;

        (value, AttributeMatcher::EndsWith, remainder)
    } else if remainder.starts_with("*=") {
        let (value, remainder) = parse_identifier_or_string(
            &remainder[2..],
            offset + input_remainder.len() - remainder.len() + 2,
        )?;

        (value, AttributeMatcher::Substring, remainder)
    } else {
        if let Some(c) = remainder.chars().next() {
            return Err(ParseError {
                message: format!(
                    "expected attribute selector terminator (`]`) or value matcher (`=`, `~=`, \
                    `|=`, `^=`,  `$=`, `*=`), found `{}`",
                    c
                ),
                position: offset + input_remainder.len() - remainder.len(),
            });
        } else {
            return Err(ParseError {
                message: "unexpected end; unclosed attribute selector".to_string(),
                position: offset + input_remainder.len() - remainder.len(),
            });
        }
    };

    let mut case = CaseSensitivity::Default;
    let mut remainder = skip_whitespace(remainder);

    if remainder.starts_with(&['i', 'I']) {
        case = CaseSensitivity::Insensitive;
        remainder = skip_whitespace(&remainder[1..])
    }

    if remainder.starts_with(&['s', 'S']) {
        case = CaseSensitivity::Sensitive;
        remainder = skip_whitespace(&remainder[1..])
    }

    if let Some(c) = remainder.chars().next() {
        if c == ']' {
            let attribute = AttributeWithValue {
                name,
                matcher,
                value,
                case_sensitivity: case,
            };

            Ok((AttributeSelector::WithValue(attribute), &remainder[1..]))
        } else {
            Err(ParseError {
                message: format!(
                    "expected attribute selector terminator (`]`), found `{}`",
                    c
                ),
                position: offset + input_remainder.len() - remainder.len(),
            })
        }
    } else {
        return Err(ParseError {
            message: "unexpected end; unclosed attribute selector".to_string(),
            position: offset + input_remainder.len() - remainder.len(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_exists() {
        assert_eq!(
            parse_attribute_selector("attr] rest", 0),
            Ok((AttributeSelector::Exists(0..4), " rest"))
        );
    }

    #[test]
    fn valid_exact() {
        assert_eq!(
            parse_attribute_selector("attr=something] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..14,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_includes() {
        assert_eq!(
            parse_attribute_selector("attr~=something] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Includes,
                    value: 6..15,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_hyphenated_begins_with() {
        assert_eq!(
            parse_attribute_selector("attr|=something] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::HyphenatedBeginsWidth,
                    value: 6..15,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_begins_with() {
        assert_eq!(
            parse_attribute_selector("attr^=something] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::BeginsWith,
                    value: 6..15,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_end_with() {
        assert_eq!(
            parse_attribute_selector("attr$=something] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::EndsWith,
                    value: 6..15,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_substring() {
        assert_eq!(
            parse_attribute_selector("attr*=something] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Substring,
                    value: 6..15,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_with_single_quoted_string_value() {
        assert_eq!(
            parse_attribute_selector("attr='something'] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..16,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_with_double_quoted_string_value() {
        assert_eq!(
            parse_attribute_selector("attr=\"something\"] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..16,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_with_whitespace() {
        assert_eq!(
            parse_attribute_selector(" attr = something ] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 1..5,
                    matcher: AttributeMatcher::Exact,
                    value: 8..17,
                    case_sensitivity: CaseSensitivity::Default
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_identifier_with_whitespace_i() {
        assert_eq!(
            parse_attribute_selector("attr=something i] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..14,
                    case_sensitivity: CaseSensitivity::Insensitive
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_identifier_with_whitespace_I() {
        assert_eq!(
            parse_attribute_selector("attr=something I] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..14,
                    case_sensitivity: CaseSensitivity::Insensitive
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_identifier_with_whitespace_s() {
        assert_eq!(
            parse_attribute_selector("attr=something s] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..14,
                    case_sensitivity: CaseSensitivity::Sensitive
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_identifier_with_whitespace_S() {
        assert_eq!(
            parse_attribute_selector("attr=something S] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..14,
                    case_sensitivity: CaseSensitivity::Sensitive
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_string_with_i() {
        assert_eq!(
            parse_attribute_selector("attr='something'i] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..16,
                    case_sensitivity: CaseSensitivity::Insensitive
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_string_with_whitespace_i() {
        assert_eq!(
            parse_attribute_selector("attr='something' i] rest", 0),
            Ok((
                AttributeSelector::WithValue(AttributeWithValue {
                    name: 0..4,
                    matcher: AttributeMatcher::Exact,
                    value: 5..16,
                    case_sensitivity: CaseSensitivity::Insensitive
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn empty() {
        assert!(parse_attribute_selector("", 0).is_err())
    }

    #[test]
    fn unparsed_value_remainder() {
        assert!(parse_attribute_selector("attr=something something_else] rest", 0).is_err())
    }

    #[test]
    fn unclosed_attribute() {
        assert!(parse_attribute_selector("attr rest", 0).is_err())
    }

    #[test]
    fn unclosed_attribute_with_value() {
        assert!(parse_attribute_selector("attr=something rest", 0).is_err())
    }
}
