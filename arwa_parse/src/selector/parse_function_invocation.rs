use super::{skip_whitespace, ParseError};

pub fn parse_function_invocation<F, T>(
    input_remainder: &str,
    offset: usize,
    argument_parser: F,
) -> Result<(T, &str), ParseError>
where
    F: FnOnce(&str, usize) -> Result<(T, &str), ParseError>,
{
    let mut chars = input_remainder.chars();

    if let Some(c) = chars.next() {
        if c == '(' {
            let remainder = skip_whitespace(chars.as_str());

            let (argument, remainder) =
                argument_parser(remainder, offset + input_remainder.len() - remainder.len())?;

            let remainder = skip_whitespace(remainder);

            let mut chars = remainder.chars();

            if let Some(c) = chars.next() {
                let remainder = chars.as_str();

                if c == ')' {
                    Ok((argument, remainder))
                } else {
                    Err(ParseError {
                        message: format!("expected `)`, found `{}`", c),
                        position: offset + input_remainder.len() - remainder.len(),
                    })
                }
            } else {
                Err(ParseError {
                    message: "unexpected end; expected `)`".to_string(),
                    position: offset + input_remainder.len(),
                })
            }
        } else {
            Err(ParseError {
                message: format!("expected `(`, found `{}`", c),
                position: offset,
            })
        }
    } else {
        Err(ParseError {
            message: "unexpected end; expected `(`".to_string(),
            position: offset + input_remainder.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::parse_identifier::parse_identifier;

    #[test]
    fn valid_invocation() {
        assert_eq!(
            parse_function_invocation("(something) rest", 0, parse_identifier),
            Ok((1..10, " rest"))
        );
    }

    #[test]
    fn valid_invocation_with_whitespace() {
        assert_eq!(
            parse_function_invocation("( something ) rest", 0, parse_identifier),
            Ok((2..11, " rest"))
        );
    }

    #[test]
    fn with_out_opening_paren() {
        assert!(parse_function_invocation("something)", 0, parse_identifier).is_err());
    }

    #[test]
    fn with_out_closing_paren() {
        assert!(parse_function_invocation("(something rest", 0, parse_identifier).is_err());
    }

    #[test]
    fn unparsed_argument_remainder() {
        assert!(
            parse_function_invocation("(something something_else)", 0, parse_identifier).is_err()
        );
    }
}
