use std::ops::Range;

use super::parse::ParseError;

fn parse_quoted_string(
    quote_char: char,
    input_remainder: &str,
    offset: usize,
) -> Result<(Range<usize>, &str), ParseError> {
    let mut char_indices = input_remainder.char_indices();

    if let Some((pos, c)) = char_indices.next() {
        if c != quote_char {
            return Err(ParseError {
                message: format!("expected a string but found `{}`", c),
                position: offset,
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
                        position: offset + pos,
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
                position: offset,
            });
        }

        let remainder = &input_remainder[(end_pos + 1)..];
        let offset_end = offset + end_pos + 1;

        Ok((offset..offset_end, remainder))
    } else {
        Err(ParseError {
            message: "unexpected end; expected a string".to_string(),
            position: offset,
        })
    }
}

pub fn parse_single_quoted_string(
    input_remainder: &str,
    offset: usize,
) -> Result<(Range<usize>, &str), ParseError> {
    parse_quoted_string('\'', input_remainder, offset)
}

pub fn parse_double_quoted_string(
    input_remainder: &str,
    offset: usize,
) -> Result<(Range<usize>, &str), ParseError> {
    parse_quoted_string('"', input_remainder, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_single_quoted() {
        assert_eq!(
            parse_single_quoted_string("'some string' rest", 0),
            Ok((0..13, " rest"))
        );
    }

    #[test]
    fn valid_double_quoted() {
        assert_eq!(
            parse_double_quoted_string("\"some string\" rest", 0),
            Ok((0..13, " rest"))
        );
    }

    #[test]
    fn empty_single_quoted() {
        assert!(parse_single_quoted_string("", 0).is_err());
    }

    #[test]
    fn empty_double_quoted() {
        assert!(parse_double_quoted_string("", 0).is_err());
    }

    #[test]
    fn parse_single_quoted_non_string() {
        assert!(parse_single_quoted_string("some string", 0).is_err());
    }

    #[test]
    fn parse_double_quoted_non_string() {
        assert!(parse_double_quoted_string("some string", 0).is_err());
    }

    #[test]
    fn unclosed_single_quoted() {
        assert!(parse_single_quoted_string("'some string", 0).is_err());
    }

    #[test]
    fn unclosed_double_quoted() {
        assert!(parse_double_quoted_string("\"some string", 0).is_err());
    }

    #[test]
    fn single_quoted_unescaped_newline() {
        assert!(parse_single_quoted_string("'some\nstring'", 0).is_err());
    }

    #[test]
    fn double_quoted_unescaped_newline() {
        assert!(parse_double_quoted_string("\"some\nstring\"", 0).is_err());
    }

    #[test]
    fn single_quoted_escaped_newline() {
        assert_eq!(
            parse_single_quoted_string("'some\\\nstring' rest", 0),
            Ok((0..14, " rest"))
        );
    }

    #[test]
    fn double_quoted_escaped_newline() {
        assert_eq!(
            parse_double_quoted_string("\"some\\\nstring\" rest", 0),
            Ok((0..14, " rest"))
        );
    }
}
