use std::fmt;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct InvalidName {
    name: String,
    invalid_char: Option<char>,
    invalid_pos: usize
}

impl fmt::Debug for InvalidName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(invalid_char) = self.invalid_char {
            write!(f, "`{}` is not a valid XML name token; invalid character `{}` at position `{}`.", &self.name, invalid_char, self.invalid_pos)
        } else {
            write!(f, "The empty string is not a valid XML name token.")
        }
    }
}

#[derive(Clone)]
pub struct Name {
    name: String
}

impl Name {
    pub fn parse(name: &str) -> Result<Self, InvalidName> {
        let mut chars = name.chars().enumerate();

        if let Some((_, c)) = chars.next() {
            if !valid_first_char(&c) {
                return Err(InvalidName {
                    name: name.to_string(),
                    invalid_char: Some(c),
                    invalid_pos: 0
                });
            }

            for (i, c) in chars {
                if !valid_tail_char(&c) {
                    return Err(InvalidName {
                        name: name.to_string(),
                        invalid_char: Some(c),
                        invalid_pos: i
                    });
                }
            }

            Ok(Name {
                name: name.to_string()
            })
        } else {
            Err(InvalidName{
                name: name.to_string(),
                invalid_char: None,
                invalid_pos: 0
            })
        }
    }

    pub fn trusted(name: String) -> Self {
        Name {
            name
        }
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct InvalidNonColonName {
    name: String,
    invalid_char: Option<char>,
    invalid_pos: usize
}

impl fmt::Debug for InvalidNonColonName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(invalid_char) = self.invalid_char {
            write!(f, "`{}` is not a valid XML non-colon name token; invalid character `{}` at position `{}`.", &self.name, invalid_char, self.invalid_pos)
        } else {
            write!(f, "The empty string is not a valid XML non-colon name token.")
        }
    }
}

#[derive(Clone)]
pub struct NonColonName {
    name: String
}

impl NonColonName {
    pub fn parse(name: &str) -> Result<Self, InvalidNonColonName> {
        let mut chars = name.chars().enumerate();

        if let Some((_, c)) = chars.next() {
            if c == ':' || !valid_first_char(&c) {
                return Err(InvalidNonColonName {
                    name: name.to_string(),
                    invalid_char: Some(c),
                    invalid_pos: 0
                });
            }

            for (i, c) in chars {
                if c == ':' || !valid_tail_char(&c) {
                    return Err(InvalidNonColonName {
                        name: name.to_string(),
                        invalid_char: Some(c),
                        invalid_pos: i
                    });
                }
            }

            Ok(NonColonName {
                name: name.to_string()
            })
        } else {
            Err(InvalidNonColonName {
                name: name.to_string(),
                invalid_char: None,
                invalid_pos: 0
            })
        }
    }

    pub fn trusted(name: String) -> Self {
        NonColonName {
            name
        }
    }
}

impl AsRef<str> for NonColonName {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
pub struct InvalidQualifiedName {
    name: String,
    invalid_char: Option<char>,
    invalid_pos: usize
}

impl fmt::Debug for InvalidQualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(invalid_char) = self.invalid_char {
            write!(f, "`{}` is not a valid XML qualified name token; invalid character `{}` at position `{}`.", &self.name, invalid_char, self.invalid_pos)
        } else {
            write!(f, "The empty string is not a valid XML qualified name token.")
        }
    }
}

#[derive(Clone)]
pub struct QualifiedName {
    name: String,
    colon_pos: Option<usize>
}

impl QualifiedName {
    pub fn parse(name: &str) -> Result<Self, InvalidQualifiedName> {
        let mut chars = name.chars().enumerate();
        let mut colon_pos = None;

        if let Some((_, c)) = chars.next() {
            if !valid_first_char(&c) {
                return Err(InvalidQualifiedName {
                    name: name.to_string(),
                    invalid_char: Some(c),
                    invalid_pos: 0
                });
            }

            if c == ':' {
                colon_pos = Some(0);
            }

            for (i, c) in chars.next() {
                if !valid_tail_char(&c) {
                    return Err(InvalidQualifiedName {
                        name: name.to_string(),
                        invalid_char: Some(c),
                        invalid_pos: i
                    });
                }

                if c == ':' {
                    if colon_pos.is_some() {
                        return Err(InvalidQualifiedName {
                            name: name.to_string(),
                            invalid_char: Some(c),
                            invalid_pos: i
                        });
                    } else {
                        colon_pos = Some(i);
                    }
                }
            }

            Ok(QualifiedName {
                name: name.into(),
                colon_pos,
            })
        } else {
            Err(InvalidQualifiedName {
                name: name.to_string(),
                invalid_char: Some(c),
                invalid_pos: i
            })
        }
    }

    pub fn colon_position(&self) -> Option<usize> {
        self.colon_pos
    }

    pub fn prefix(&self) -> Option<&str> {
        self.colon_pos.map(|colon_pos| {
            &self.name[0..colon_pos]
        })
    }

    pub fn local_name(&self) -> &str {
        let start = self.colon_pos.unwrap_or(0);

        &self.name[start..]
    }
}

impl AsRef<str> for QualifiedName {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

const fn valid_first_char(c: &char) -> bool {
    c == &'_'
        || c == &':'
        || ('a'..='z').contains(c)
        || ('A'..='Z').contains(c)
        || ('\u{00C0}'..='\u{00D6}').contains(c)
        || ('\u{00D8}'..='\u{00F6}').contains(c)
        || ('\u{00F8}'..='\u{02FF}').contains(c)
        || ('\u{0370}'..='\u{037D}').contains(c)
        || ('\u{037F}'..='\u{1FFF}').contains(c)
        || ('\u{200C}'..='\u{200D}').contains(c)
        || ('\u{2070}'..='\u{218F}').contains(c)
        || ('\u{2C00}'..='\u{2FEF}').contains(c)
        || ('\u{3001}'..='\u{D7FF}').contains(c)
        || ('\u{F900}'..='\u{FDCF}').contains(c)
        || ('\u{FDF0}'..='\u{FFFD}').contains(c)
        || ('\u{10000}'..='\u{EFFFF}').contains(c)
}

const fn valid_tail_char(c: &char) -> bool {
    valid_first_char(c)
        || c == &'-'
        || c == &'.'
        || c == '\u{00B7}'
        || ('0'..='9').contains(c)
        || ('\u{0300}'..='\u{036F}').contains(c)
        || ('\u{203F}'..='\u{2040}').contains(c)
}
