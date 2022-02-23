use std::fmt;

#[derive(Clone)]
pub struct InvalidRequestMethod {
    token: String,
    invalid_pos: usize
}

impl fmt::Debug for InvalidRequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.token.is_empty() {
            write!(f, "The empty string is not a valid request method.")
        } else {
            write!(f, "`{}` is not a valid request method; disallowed code point at position `{}`.", &self.token, self.invalid_pos)
        }
    }
}

#[derive(Clone)]
pub struct RequestMethod {
    request_method: String
}

impl RequestMethod {
    pub fn parse(token: &str) -> Result<Self, InvalidRequestMethod> {
        fn valid_char(c: &char) -> bool {
            ('0'..='9').contains(c)
                || ('a'..='z').contains(c)
                || ('A'..='Z').contains(c)
                || "!#$%&'*+-.^_`|~".contains(*c)
        }

        let mut chars = token.chars().enumerate();

        if let Some((_, c)) = chars.next() {
            if !valid_char(&c) {
                return Err(InvalidRequestMethod {
                    token: token.to_string(),
                    invalid_pos: 0
                });
            }

            for (i, c) in chars {
                if !valid_char(&c) {
                    return Err(InvalidRequestMethod {
                        token: token.to_string(),
                        invalid_pos: i
                    });
                }
            }

            Ok(RequestMethod {
                request_method: token.to_string()
            })
        } else {
            Err(InvalidRequestMethod {
                token: token.to_string(),
                invalid_pos: 0
            })
        }
    }

    pub fn trusted(request_method: String) -> Self {
        RequestMethod {
            request_method
        }
    }
}

impl AsRef<str> for RequestMethod {
    fn as_ref(&self) -> &str {
        &self.request_method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn method_from_valid_str() {
        assert!(RequestMethod::parse("019abzABZ!#$%&'*+-.^_`|~").is_ok());
    }

    #[test]
    fn method_from_invalid_str() {
        assert!(RequestMethod::parse("019abzABZ!#$%&'*+-.^_`|~?").is_err());
    }
}
