use std::ops::Range;

use super::{
    parse_double_quoted_string, parse_identifier, parse_single_quoted_string, skip_whitespace,
    ParseError,
};

pub fn parse_identifier_or_string(
    input_remainder: &str,
    offset: usize,
) -> Result<(Range<usize>, &str), ParseError> {
    let remainder = skip_whitespace(input_remainder);
    let start = offset + input_remainder.len() - remainder.len();

    if remainder.starts_with('\'') {
        parse_single_quoted_string(remainder, start)
    } else if remainder.starts_with('"') {
        parse_double_quoted_string(remainder, start)
    } else {
        parse_identifier(remainder, start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_identifier() {
        assert_eq!(
            parse_identifier_or_string("a_1- rest", 0),
            Ok((0..4, " rest"))
        );
    }

    #[test]
    fn valid_single_quoted_string() {
        assert_eq!(
            parse_identifier_or_string("'some string' rest", 0),
            Ok((0..13, " rest"))
        );
    }

    #[test]
    fn valid_double_quoted_string() {
        assert_eq!(
            parse_identifier_or_string("\"some string\" rest", 0),
            Ok((0..13, " rest"))
        );
    }
}
