use std::ops::Range;

use super::{is_identifier_continue, is_identifier_start, ParseError, Remainder};

pub fn parse_identifier(remainder: Remainder) -> Result<(Range<usize>, Remainder), ParseError> {
    let mut char_indices = remainder.char_indices();

    if let Some((_, c)) = char_indices.next() {
        if !is_identifier_start(c) {
            return Err(ParseError {
                message: format!("expected an identifier but found `{}`", c),
                offset: remainder.offset(),
            });
        }

        let mut identifier_len = 1;

        for (pos, c) in char_indices {
            if is_identifier_continue(c) {
                identifier_len = pos + 1;
            } else {
                break;
            }
        }

        Ok((
            remainder.offset().range(identifier_len),
            remainder.skip(identifier_len),
        ))
    } else {
        Err(ParseError {
            message: "unexpected end; expected an identifier".to_string(),
            offset: remainder.offset(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_followed_by_whitespace() {
        let (selector, remainder) = parse_identifier("a_1- rest".into()).unwrap();

        assert_eq!(selector, 0..4);
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_followed_by_non_whitespace_non_identifier_char() {
        let (selector, remainder) = parse_identifier("a_1-#rest".into()).unwrap();

        assert_eq!(selector, 0..4);
        assert_eq!(remainder, "#rest");
    }

    #[test]
    fn empty_string() {
        assert!(parse_identifier("".into()).is_err());
    }

    #[test]
    fn invalid_first_char() {
        assert!(parse_identifier("0".into()).is_err());
    }
}
