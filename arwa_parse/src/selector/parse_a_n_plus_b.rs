use std::str::FromStr;

use super::{
    case_insensitive_eq, is_whitespace, parse_selector_list, ANPlusB, ANPlusBOf, ParseError,
    Remainder,
};

pub fn parse_a_n_plus_b(remainder: Remainder) -> Result<(ANPlusB, Remainder), ParseError> {
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

    let mut remainder = remainder.skip_whitespace();

    if remainder.len() >= 3 && case_insensitive_eq(&remainder[..3], "odd") {
        let remainder = remainder.skip(3);

        if let Some(c) = remainder.chars().next() {
            if c == ')' || is_whitespace(c) {
                return Ok((ANPlusB::Odd, remainder));
            }
        } else {
            return Err(ParseError {
                message: "unexpected end; unclosed function".to_string(),
                offset: remainder.offset(),
            });
        }
    }

    if remainder.len() >= 4 && case_insensitive_eq(&remainder[..4], "even") {
        let remainder = remainder.skip(4);

        if let Some(c) = remainder.chars().next() {
            if c == ')' || is_whitespace(c) {
                return Ok((ANPlusB::Even, remainder));
            }
        } else {
            return Err(ParseError {
                message: "unexpected end; unclosed function".to_string(),
                offset: remainder.offset(),
            });
        }
    }

    let number_0_negative = remainder.starts_with('-');

    // Advance past the sign if present
    if remainder.starts_with(&['+', '-']) {
        remainder = remainder.skip(1);
    }

    let number_0_digits_end = find_digits_end(&remainder);

    // Evaluates to None if there are no digits at the start of the remainder
    let number_0 = i32::from_str(&remainder[0..number_0_digits_end]).ok();

    let remainder = remainder.skip(number_0_digits_end);

    if remainder.starts_with(&['n', 'N']) {
        let mut a = number_0.unwrap_or(1);

        if number_0_negative {
            a = -a;
        }

        let remainder = remainder.skip(1).skip_whitespace();

        if remainder.starts_with(&['+', '-']) {
            let b_is_negative = remainder.starts_with('-');

            let remainder = remainder.skip(1).skip_whitespace();

            let b_digits_end = find_digits_end(&remainder);

            if let Ok(mut b) = i32::from_str(&remainder[0..b_digits_end]) {
                if b_is_negative {
                    b = -b;
                }

                return Ok((ANPlusB::ANPlusB(a, b), remainder.skip(b_digits_end)));
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
        offset: remainder.offset(),
    })
}

pub fn parse_a_n_plus_b_of(remainder: Remainder) -> Result<(ANPlusBOf, Remainder), ParseError> {
    let (a_n_plus_b, remainder) = parse_a_n_plus_b(remainder)?;

    let has_whitespace = remainder.chars().next().map(is_whitespace).unwrap_or(false);
    let remainder = remainder.skip_whitespace();

    if remainder.starts_with(')') {
        let an_plus_b_of = ANPlusBOf {
            a_n_plus_b,
            of: None,
        };

        return Ok((an_plus_b_of, remainder));
    }

    if has_whitespace && remainder.len() >= 2 {
        let starts_with_of = case_insensitive_eq(&remainder[..2], "of");
        let has_whitespace = remainder[2..]
            .chars()
            .next()
            .map(is_whitespace)
            .unwrap_or(false);

        if starts_with_of && has_whitespace {
            let remainder = remainder.skip(2);

            let (selector, remainder) = parse_selector_list(remainder)?;

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
            offset: remainder.offset(),
        })
    } else {
        Err(ParseError {
            message: "expected function terminator `)` or ` of [selector]` pattern".to_string(),
            offset: remainder.offset(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::{ComplexSelector, CompoundSelector, SelectorList, TypeSelector};

    #[test]
    fn valid_odd() {
        let (selector, remainder) = parse_a_n_plus_b("odd) rest".into()).unwrap();

        assert_eq!(selector, ANPlusB::Odd);
        assert_eq!(remainder, ") rest");
    }

    #[test]
    fn valid_upper_case_odd() {
        let (selector, remainder) = parse_a_n_plus_b("ODD) rest".into()).unwrap();

        assert_eq!(selector, ANPlusB::Odd);
        assert_eq!(remainder, ") rest");
    }

    #[test]
    fn valid_odd_with_whitespace() {
        let (selector, remainder) = parse_a_n_plus_b("odd ) rest".into()).unwrap();

        assert_eq!(selector, ANPlusB::Odd);
        assert_eq!(remainder, " ) rest");
    }

    #[test]
    fn valid_odd_of_selector() {
        let (selector, remainder) = parse_a_n_plus_b_of("odd of *) rest".into()).unwrap();

        assert_eq!(
            selector,
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
            }
        );
        assert_eq!(remainder, ") rest");
    }

    #[test]
    fn valid_odd_upper_case_of_selector() {
        let (selector, remainder) = parse_a_n_plus_b_of("odd OF *) rest".into()).unwrap();

        assert_eq!(
            selector,
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
            }
        );
        assert_eq!(remainder, ") rest");
    }

    #[test]
    fn odd_of_no_selector() {
        assert!(parse_a_n_plus_b_of("odd of )".into()).is_err());
    }

    #[test]
    fn valid_even() {
        let (selector, remainder) = parse_a_n_plus_b("even) rest".into()).unwrap();

        assert_eq!(selector, ANPlusB::Even);
        assert_eq!(remainder, ") rest");
    }

    #[test]
    fn valid_upper_case_even() {
        let (selector, remainder) = parse_a_n_plus_b("EVEN) rest".into()).unwrap();

        assert_eq!(selector, ANPlusB::Even);
        assert_eq!(remainder, ") rest");
    }

    #[test]
    fn non_odd_non_even_identifier() {
        assert!(parse_a_n_plus_b("odd_)".into()).is_err());
    }

    #[test]
    fn only_b() {
        let (selector, remainder) = parse_a_n_plus_b("3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(0, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn only_b_multiple_digits() {
        let (selector, remainder) = parse_a_n_plus_b("321)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(0, 321));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn only_b_signed_positive() {
        let (selector, remainder) = parse_a_n_plus_b("+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(0, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn only_b_signed_negative() {
        let (selector, remainder) = parse_a_n_plus_b("-3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(0, -3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn only_b_signed_with_whitespace() {
        assert!(parse_a_n_plus_b("+ 3)".into()).is_err());
    }

    #[test]
    fn n_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("n+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(1, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn upper_case_n_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("N+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(1, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn n_whitespace_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("n +3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(1, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn n_whitespace_plus_whitespace_b() {
        let (selector, remainder) = parse_a_n_plus_b("n + 3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(1, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn n_plus_whitespace_b() {
        let (selector, remainder) = parse_a_n_plus_b("n+ 3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(1, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn n_minus_b() {
        let (selector, remainder) = parse_a_n_plus_b("n-3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(1, -3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn n_plus_minus_b() {
        assert!(parse_a_n_plus_b("n+-3)".into()).is_err());
    }

    #[test]
    fn plus_n_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("+n+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(1, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn minus_n_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("-n+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(-1, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn minus_whitespace_n_plus_b() {
        assert!(parse_a_n_plus_b("- n+3)".into()).is_err());
    }

    #[test]
    fn a_n_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("2n+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(2, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn multiple_digits_a_n_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("321n+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(321, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn a_whitespace_n_plus_b() {
        // Will fail later on check for end of function invocation (perhaps not ideal in terms of
        // error message...)
        let (selector, remainder) = parse_a_n_plus_b("2 n+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(0, 2));
        assert_eq!(remainder, " n+3)");
    }

    #[test]
    fn plus_a_n_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("+2n+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(2, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn minus_a_n_plus_b() {
        let (selector, remainder) = parse_a_n_plus_b("-2n+3)".into()).unwrap();

        assert_eq!(selector, ANPlusB::ANPlusB(-2, 3));
        assert_eq!(remainder, ")");
    }

    #[test]
    fn plus_whitespace_a_n_plus_b() {
        assert!(parse_a_n_plus_b("+ 2n+3)".into()).is_err());
    }
}
