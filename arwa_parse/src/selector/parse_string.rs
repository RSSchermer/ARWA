use std::ops::Range;

use super::parse::{ParseError, Remainder};

fn parse_quoted_string(
    quote_char: char,
    remainder: Remainder,
) -> Result<(Range<usize>, Remainder), ParseError> {
    let mut char_indices = remainder.char_indices();

    if let Some((pos, c)) = char_indices.next() {
        if c != quote_char {
            return Err(ParseError {
                message: format!("expected a string but found `{}`", c),
                offset: remainder.offset(),
            });
        }

        let mut escaped = false;
        let mut closed = false;
        let mut end_pos = pos;

        for (pos, c) in char_indices {
            if !escaped {
                if c == quote_char {
                    closed = true;
                    end_pos = pos;

                    break;
                }

                if c == '\n' {
                    return Err(ParseError {
                        message: "unescaped newlines are not allowed in a string sequence"
                            .to_string(),
                        offset: remainder.offset() + pos,
                    });
                }

                if c == '\\' {
                    escaped = true;
                }
            } else {
                escaped = false;
            }
        }

        if !closed {
            return Err(ParseError {
                message: "unclosed string sequence".to_string(),
                offset: remainder.offset(),
            });
        }

        let string_len = end_pos + 1;

        Ok((
            remainder.offset().range(string_len),
            remainder.skip(string_len),
        ))
    } else {
        Err(ParseError {
            message: "unexpected end; expected a string".to_string(),
            offset: remainder.offset(),
        })
    }
}

pub fn parse_single_quoted_string(
    remainder: Remainder,
) -> Result<(Range<usize>, Remainder), ParseError> {
    parse_quoted_string('\'', remainder)
}

pub fn parse_double_quoted_string(
    remainder: Remainder,
) -> Result<(Range<usize>, Remainder), ParseError> {
    parse_quoted_string('"', remainder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_single_quoted() {
        let (selector, remainder) =
            parse_single_quoted_string("'some string' rest".into()).unwrap();

        assert_eq!(selector, 0..13);
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_double_quoted() {
        let (selector, remainder) =
            parse_double_quoted_string("\"some string\" rest".into()).unwrap();

        assert_eq!(selector, 0..13);
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn empty_single_quoted() {
        assert!(parse_single_quoted_string("".into()).is_err());
    }

    #[test]
    fn empty_double_quoted() {
        assert!(parse_double_quoted_string("".into()).is_err());
    }

    #[test]
    fn parse_single_quoted_non_string() {
        assert!(parse_single_quoted_string("some string".into()).is_err());
    }

    #[test]
    fn parse_double_quoted_non_string() {
        assert!(parse_double_quoted_string("some string".into()).is_err());
    }

    #[test]
    fn unclosed_single_quoted() {
        assert!(parse_single_quoted_string("'some string".into()).is_err());
    }

    #[test]
    fn unclosed_double_quoted() {
        assert!(parse_double_quoted_string("\"some string".into()).is_err());
    }

    #[test]
    fn single_quoted_unescaped_newline() {
        assert!(parse_single_quoted_string("'some\nstring'".into()).is_err());
    }

    #[test]
    fn double_quoted_unescaped_newline() {
        assert!(parse_double_quoted_string("\"some\nstring\"".into()).is_err());
    }

    #[test]
    fn single_quoted_escaped_newline() {
        let (selector, remainder) =
            parse_single_quoted_string("'some\\\nstring' rest".into()).unwrap();

        assert_eq!(selector, 0..14);
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn double_quoted_escaped_newline() {
        let (selector, remainder) =
            parse_double_quoted_string("\"some\\\nstring\" rest".into()).unwrap();

        assert_eq!(selector, 0..14);
        assert_eq!(remainder, " rest");
    }
}
