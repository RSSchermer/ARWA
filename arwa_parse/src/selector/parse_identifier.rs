use std::ops::Range;

use super::{is_identifier_continue, is_identifier_start, ParseError};

pub fn parse_identifier(
    input_remainder: &str,
    offset: usize,
) -> Result<(Range<usize>, &str), ParseError> {
    let mut char_indices = input_remainder.char_indices();

    if let Some((_, c)) = char_indices.next() {
        if !is_identifier_start(c) {
            return Err(ParseError {
                message: format!("expected an identifier but found `{}`", c),
                position: offset,
            });
        }

        let mut end_pos = 1;

        for (pos, c) in char_indices {
            if is_identifier_continue(c) {
                end_pos = pos + 1;
            } else {
                break;
            }
        }

        let offset_end = offset + end_pos;

        Ok((offset..offset_end, &input_remainder[end_pos..]))
    } else {
        Err(ParseError {
            message: "unexpected end; expected an identifier".to_string(),
            position: offset,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_followed_by_whitespace() {
        assert_eq!(parse_identifier("a_1- rest", 0), Ok((0..4, " rest")));
    }

    #[test]
    fn valid_followed_by_non_whitespace_non_identifier_char() {
        assert_eq!(parse_identifier("a_1-#rest", 0), Ok((0..4, "#rest")));
    }

    #[test]
    fn empty_string() {
        assert!(parse_identifier("", 0).is_err());
    }

    #[test]
    fn invalid_first_char() {
        assert!(parse_identifier("0", 0).is_err());
    }
}
