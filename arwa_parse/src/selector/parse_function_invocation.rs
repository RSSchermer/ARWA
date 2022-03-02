use super::{ParseError, Remainder};

pub fn parse_function_invocation<F, T>(
    remainder: Remainder,
    argument_parser: F,
) -> Result<(T, Remainder), ParseError>
where
    F: FnOnce(Remainder) -> Result<(T, Remainder), ParseError>,
{
    if let Some(c) = remainder.chars().next() {
        if c == '(' {
            let remainder = remainder.skip(1).skip_whitespace();

            let (argument, remainder) = argument_parser(remainder)?;

            let remainder = remainder.skip_whitespace();

            if let Some(c) = remainder.chars().next() {
                if c == ')' {
                    Ok((argument, remainder.skip(1)))
                } else {
                    Err(ParseError {
                        message: format!("expected `)`, found `{}`", c),
                        offset: remainder.offset(),
                    })
                }
            } else {
                Err(ParseError {
                    message: "unexpected end; expected `)`".to_string(),
                    offset: remainder.offset(),
                })
            }
        } else {
            Err(ParseError {
                message: format!("expected `(`, found `{}`", c),
                offset: remainder.offset(),
            })
        }
    } else {
        Err(ParseError {
            message: "unexpected end; expected `(`".to_string(),
            offset: remainder.offset(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::parse_identifier::parse_identifier;

    #[test]
    fn valid_invocation() {
        let (selector, remainder) =
            parse_function_invocation("(something) rest".into(), parse_identifier).unwrap();

        assert_eq!(selector, 1..10);
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn valid_invocation_with_whitespace() {
        let (selector, remainder) =
            parse_function_invocation("( something ) rest".into(), parse_identifier).unwrap();

        assert_eq!(selector, 2..11);
        assert_eq!(remainder, " rest");
    }

    #[test]
    fn with_out_opening_paren() {
        assert!(parse_function_invocation("something)".into(), parse_identifier).is_err());
    }

    #[test]
    fn with_out_closing_paren() {
        assert!(parse_function_invocation("(something rest".into(), parse_identifier).is_err());
    }

    #[test]
    fn unparsed_argument_remainder() {
        assert!(
            parse_function_invocation("(something something_else)".into(), parse_identifier)
                .is_err()
        );
    }
}
