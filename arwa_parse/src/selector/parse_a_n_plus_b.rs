use std::str::FromStr;

use super::{
    case_insensitive_eq, is_whitespace, parse_selector_list, skip_whitespace, ANPlusB, ANPlusBOf,
    ParseError,
};

pub fn parse_a_n_plus_b(
    input_remainder: &str,
    offset: usize,
) -> Result<(ANPlusB, &str), ParseError> {
    // Helper for finding the end of a sequence of digits from the start of the remainder.
    fn find_digits_end(remainder: &str) -> usize {
        let mut digits_end = 0;

        for (pos, c) in remainder.char_indices() {
            if c.is_ascii_digit() {
                digits_end = pos + 1;
            } else {
                break;
            }
        }

        digits_end
    }

    let mut remainder = skip_whitespace(input_remainder);

    if remainder.len() >= 3 && case_insensitive_eq(&remainder[..3], "odd") {
        let remainder = &remainder[3..];

        if let Some(c) = remainder.chars().next() {
            if c == ')' || is_whitespace(c) {
                return Ok((ANPlusB::Odd, remainder));
            }
        } else {
            return Err(ParseError {
                message: "unexpected end; unclosed function".to_string(),
                position: offset + input_remainder.len(),
            });
        }
    }

    if remainder.len() >= 4 && case_insensitive_eq(&remainder[..4], "even") {
        let remainder = &remainder[4..];

        if let Some(c) = remainder.chars().next() {
            if c == ')' || is_whitespace(c) {
                return Ok((ANPlusB::Even, remainder));
            }
        } else {
            return Err(ParseError {
                message: "unexpected end; unclosed function".to_string(),
                position: offset + input_remainder.len(),
            });
        }
    }

    let number_0_negative = remainder.starts_with('-');

    // Advance past the sign if present
    if remainder.starts_with(&['+', '-']) {
        remainder = &remainder[1..];
    }

    let number_0_digits_end = find_digits_end(remainder);

    let number_0 = i32::from_str(&remainder[0..number_0_digits_end]).ok();

    let remainder = &remainder[number_0_digits_end..];

    if remainder.starts_with(&['n', 'N']) {
        let mut a = number_0.unwrap_or(1);

        if number_0_negative {
            a = -a;
        }

        let remainder = &remainder[1..];

        let remainder = skip_whitespace(remainder);

        if remainder.starts_with(&['+', '-']) {
            let b_is_negative = remainder.starts_with('-');

            let remainder = &remainder[1..];
            let remainder = skip_whitespace(remainder);

            let b_digits_end = find_digits_end(remainder);

            if let Ok(mut b) = i32::from_str(&remainder[0..b_digits_end]) {
                if b_is_negative {
                    b = -b;
                }

                return Ok((ANPlusB::ANPlusB(a, b), &remainder[b_digits_end..]));
            }
        } else {
            return Ok((ANPlusB::ANPlusB(a, 0), remainder));
        }
    } else {
        if let Some(mut b) = number_0 {
            if number_0_negative {
                b = -b;
            }

            return Ok((ANPlusB::ANPlusB(0, b), remainder));
        }
    }

    Err(ParseError {
        message: "expected `an + b` pattern or the `odd` or `even` short-hand".to_string(),
        position: offset,
    })
}

pub fn parse_a_n_plus_b_of(
    input_remainder: &str,
    offset: usize,
) -> Result<(ANPlusBOf, &str), ParseError> {
    let (a_n_plus_b, remainder) = parse_a_n_plus_b(input_remainder, offset)?;

    let has_whitespace = remainder.chars().next().map(is_whitespace).unwrap_or(false);
    let remainder = skip_whitespace(remainder);

    if remainder.starts_with(')') {
        let an_plus_b_of = ANPlusBOf {
            a_n_plus_b,
            of: None,
        };

        return Ok((an_plus_b_of, remainder));
    }

    if has_whitespace && remainder.len() >= 2 {
        let starts_with_of = case_insensitive_eq(&remainder[..2], "of");
        let remainder = &remainder[2..];
        let has_whitespace = remainder.chars().next().map(is_whitespace).unwrap_or(false);

        if starts_with_of && has_whitespace {
            let (selector, remainder) =
                parse_selector_list(remainder, offset + input_remainder.len() - remainder.len())?;

            let an_plus_b_of = ANPlusBOf {
                a_n_plus_b,
                of: Some(selector),
            };

            return Ok((an_plus_b_of, remainder));
        }
    }

    if remainder.is_empty() {
        Err(ParseError {
            message: "unexpected end; expected function terminator `)` or ` of [selector]` pattern"
                .to_string(),
            position: offset + input_remainder.len(),
        })
    } else {
        Err(ParseError {
            message: "expected function terminator `)` or ` of [selector]` pattern".to_string(),
            position: offset + input_remainder.len() - remainder.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::{ComplexSelector, CompoundSelector, SelectorList, TypeSelector};

    #[test]
    fn valid_odd() {
        assert_eq!(
            parse_a_n_plus_b("odd) rest", 0),
            Ok((ANPlusB::Odd, ") rest"))
        );
    }

    #[test]
    fn valid_upper_case_odd() {
        assert_eq!(
            parse_a_n_plus_b("ODD) rest", 0),
            Ok((ANPlusB::Odd, ") rest"))
        );
    }

    #[test]
    fn valid_odd_with_whitespace() {
        assert_eq!(
            parse_a_n_plus_b("odd ) rest", 0),
            Ok((ANPlusB::Odd, " ) rest"))
        );
    }

    #[test]
    fn valid_odd_of_selector() {
        assert_eq!(
            parse_a_n_plus_b_of("odd of *) rest", 0),
            Ok((
                ANPlusBOf {
                    a_n_plus_b: ANPlusB::Odd,
                    of: Some(SelectorList {
                        selector_list: vec![ComplexSelector {
                            head: CompoundSelector {
                                type_selector: Some(TypeSelector::Universal),
                                id_selector: None,
                                class_selectors: vec![],
                                attribute_selectors: vec![],
                                pseudo_class_selectors: vec![]
                            },
                            tail: vec![]
                        }]
                    })
                },
                ") rest"
            ))
        );
    }

    #[test]
    fn valid_odd_upper_case_of_selector() {
        assert_eq!(
            parse_a_n_plus_b_of("odd OF *) rest", 0),
            Ok((
                ANPlusBOf {
                    a_n_plus_b: ANPlusB::Odd,
                    of: Some(SelectorList {
                        selector_list: vec![ComplexSelector {
                            head: CompoundSelector {
                                type_selector: Some(TypeSelector::Universal),
                                id_selector: None,
                                class_selectors: vec![],
                                attribute_selectors: vec![],
                                pseudo_class_selectors: vec![]
                            },
                            tail: vec![]
                        }]
                    })
                },
                ") rest"
            ))
        );
    }

    #[test]
    fn odd_of_no_selector() {
        assert!(parse_a_n_plus_b_of("odd of )", 0).is_err());
    }

    #[test]
    fn valid_even() {
        assert_eq!(
            parse_a_n_plus_b("even) rest", 0),
            Ok((ANPlusB::Even, ") rest"))
        );
    }

    #[test]
    fn valid_upper_case_even() {
        assert_eq!(
            parse_a_n_plus_b("EVEN) rest", 0),
            Ok((ANPlusB::Even, ") rest"))
        );
    }

    #[test]
    fn non_odd_non_even_identifier() {
        assert!(parse_a_n_plus_b("odd_)", 0).is_err());
    }

    #[test]
    fn only_b() {
        assert_eq!(parse_a_n_plus_b("3)", 0), Ok((ANPlusB::ANPlusB(0, 3), ")")));
    }

    #[test]
    fn only_b_multiple_digits() {
        assert_eq!(
            parse_a_n_plus_b("321)", 0),
            Ok((ANPlusB::ANPlusB(0, 321), ")"))
        );
    }

    #[test]
    fn only_b_signed_positive() {
        assert_eq!(
            parse_a_n_plus_b("+3)", 0),
            Ok((ANPlusB::ANPlusB(0, 3), ")"))
        );
    }

    #[test]
    fn only_b_signed_negative() {
        assert_eq!(
            parse_a_n_plus_b("-3)", 0),
            Ok((ANPlusB::ANPlusB(0, -3), ")"))
        );
    }

    #[test]
    fn only_b_signed_with_whitespace() {
        assert!(parse_a_n_plus_b("+ 3)", 0).is_err());
    }

    #[test]
    fn n_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("n+3)", 0),
            Ok((ANPlusB::ANPlusB(1, 3), ")"))
        );
    }

    #[test]
    fn upper_case_n_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("N+3)", 0),
            Ok((ANPlusB::ANPlusB(1, 3), ")"))
        );
    }

    #[test]
    fn n_whitespace_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("n +3)", 0),
            Ok((ANPlusB::ANPlusB(1, 3), ")"))
        );
    }

    #[test]
    fn n_whitespace_plus_whitespace_b() {
        assert_eq!(
            parse_a_n_plus_b("n + 3)", 0),
            Ok((ANPlusB::ANPlusB(1, 3), ")"))
        );
    }

    #[test]
    fn n_plus_whitespace_b() {
        assert_eq!(
            parse_a_n_plus_b("n+ 3)", 0),
            Ok((ANPlusB::ANPlusB(1, 3), ")"))
        );
    }

    #[test]
    fn n_minus_b() {
        assert_eq!(
            parse_a_n_plus_b("n-3)", 0),
            Ok((ANPlusB::ANPlusB(1, -3), ")"))
        );
    }

    #[test]
    fn n_plus_minus_b() {
        assert!(parse_a_n_plus_b("n+-3)", 0).is_err());
    }

    #[test]
    fn plus_n_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("+n+3)", 0),
            Ok((ANPlusB::ANPlusB(1, 3), ")"))
        );
    }

    #[test]
    fn minus_n_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("-n+3)", 0),
            Ok((ANPlusB::ANPlusB(-1, 3), ")"))
        );
    }

    #[test]
    fn minus_whitespace_n_plus_b() {
        assert!(parse_a_n_plus_b("- n+3)", 0).is_err());
    }

    #[test]
    fn a_n_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("2n+3)", 0),
            Ok((ANPlusB::ANPlusB(2, 3), ")"))
        );
    }

    #[test]
    fn multiple_digits_a_n_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("321n+3)", 0),
            Ok((ANPlusB::ANPlusB(321, 3), ")"))
        );
    }

    #[test]
    fn a_whitespace_n_plus_b() {
        // Will fail later on check for end of function invocation (perhaps not ideal in terms of
        // error message...)
        assert_eq!(
            parse_a_n_plus_b("2 n+3)", 0),
            Ok((ANPlusB::ANPlusB(0, 2), " n+3)"))
        );
    }

    #[test]
    fn plus_a_n_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("+2n+3)", 0),
            Ok((ANPlusB::ANPlusB(2, 3), ")"))
        );
    }

    #[test]
    fn minus_a_n_plus_b() {
        assert_eq!(
            parse_a_n_plus_b("-2n+3)", 0),
            Ok((ANPlusB::ANPlusB(-2, 3), ")"))
        );
    }

    #[test]
    fn plus_whitespace_a_n_plus_b() {
        assert!(parse_a_n_plus_b("+ 2n+3)", 0).is_err());
    }
}
