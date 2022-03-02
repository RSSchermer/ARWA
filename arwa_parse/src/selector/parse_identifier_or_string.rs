use std::ops::Range;

use super::{
    parse_double_quoted_string, parse_identifier, parse_single_quoted_string, ParseError, Remainder,
};

pub fn parse_identifier_or_string(
    remainder: Remainder,
) -> Result<(Range<usize>, Remainder), ParseError> {
    let remainder = remainder.skip_whitespace();

    if remainder.starts_with('\'') {
        parse_single_quoted_string(remainder)
    } else if remainder.starts_with('"') {
        parse_double_quoted_string(remainder)
    } else {
        parse_identifier(remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_identifier() {
        let (selector, remainder) = parse_identifier_or_string("a_1- rest".into()).unwrap();

        assert_eq!(selector, 0..4);
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_single_quoted_string() {
        let (selector, remainder) =
            parse_identifier_or_string("'some string' rest".into()).unwrap();

        assert_eq!(selector, 0..13);
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_double_quoted_string() {
        let (selector, remainder) =
            parse_identifier_or_string("\"some string\" rest".into()).unwrap();

        assert_eq!(selector, 0..13);
        assert_eq!(remainder, " rest");
    }
}
