use std::error::Error;
use std::fmt;

#[derive(Clone)]
pub struct InvalidCustomElementName {
    name: String,
    invalid_char: Option<char>,
    invalid_pos: usize,
    missing_hyphen: bool,
}

impl fmt::Display for InvalidCustomElementName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(invalid_char) = self.invalid_char {
            write!(
                f,
                "`{}` is not a valid custom element name; invalid character `{}` at position `{}`",
                &self.name, invalid_char, self.invalid_pos
            )
        } else if self.name.is_empty() {
            write!(f, "the empty string is not a valid custom element name")
        } else if self.missing_hyphen {
            write!(
                f,
                "`{}` is not a valid custom element name; must contain a hyphen",
                &self.name
            )
        } else {
            write!(
                f,
                "`{}` is not a valid custom element name; reserved name",
                &self.name
            )
        }
    }
}

impl fmt::Debug for InvalidCustomElementName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for InvalidCustomElementName {}

#[derive(Clone)]
pub struct CustomElementName {
    name: String,
}

impl CustomElementName {
    pub fn parse(name: &str) -> Result<Self, InvalidCustomElementName> {
        if RESERVED_NAMES.contains(&name) {
            return Err(InvalidCustomElementName {
                name: name.to_string(),
                invalid_char: None,
                missing_hyphen: false,
                invalid_pos: 0,
            });
        }

        let mut chars = name.chars().enumerate();

        if let Some((_, c)) = chars.next() {
            if !valid_first_char(&c) {
                return Err(InvalidCustomElementName {
                    name: name.to_string(),
                    invalid_char: Some(c),
                    missing_hyphen: false,
                    invalid_pos: 0,
                });
            }

            let mut has_hyphen = false;

            for (i, c) in chars {
                if !valid_tail_char(&c) {
                    return Err(InvalidCustomElementName {
                        name: name.to_string(),
                        invalid_char: Some(c),
                        missing_hyphen: false,
                        invalid_pos: i,
                    });
                }

                if c == '-' {
                    has_hyphen = true;
                }
            }

            if !has_hyphen {
                return Err(InvalidCustomElementName {
                    name: name.to_string(),
                    invalid_char: None,
                    missing_hyphen: true,
                    invalid_pos: 0,
                });
            }

            Ok(CustomElementName {
                name: name.to_string(),
            })
        } else {
            Err(InvalidCustomElementName {
                name: name.to_string(),
                invalid_char: None,
                missing_hyphen: false,
                invalid_pos: 0,
            })
        }
    }

    pub fn trusted(name: String) -> Self {
        CustomElementName { name }
    }
}

impl AsRef<str> for CustomElementName {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

const RESERVED_NAMES: &'static [&'static str] = &[
    "annotation-xml",
    "color-profile",
    "font-face",
    "font-face-src",
    "font-face-uri",
    "font-face-format",
    "font-face-name",
    "missing-glyph",
];

fn valid_first_char(c: &char) -> bool {
    ('a'..='z').contains(c)
}

fn valid_tail_char(c: &char) -> bool {
    c == &'-'
        || c == &'.'
        || c == &'_'
        || c == &'\u{B7}'
        || ('0'..='9').contains(c)
        || ('a'..='z').contains(c)
        || ('\u{C0}'..='\u{D6}').contains(c)
        || ('\u{D8}'..='\u{F6}').contains(c)
        || ('\u{F8}'..='\u{37D}').contains(c)
        || ('\u{37F}'..='\u{1FFF}').contains(c)
        || ('\u{200C}'..='\u{200D}').contains(c)
        || ('\u{203F}'..='\u{2040}').contains(c)
        || ('\u{2070}'..='\u{218F}').contains(c)
        || ('\u{2C00}'..='\u{2FEF}').contains(c)
        || ('\u{3001}'..='\u{D7FF}').contains(c)
        || ('\u{F900}'..='\u{FDCF}').contains(c)
        || ('\u{FDF0}'..='\u{FFFD}').contains(c)
        || ('\u{10000}'..='\u{EFFFF}').contains(c)
}
