const WHITESPACE_PATTERN: &'static [char] =
    &['\u{0020}', '\u{0009}', '\u{000A}', '\u{000D}', '\u{000C}'];

pub fn skip_whitespace(remainder: &str) -> &str {
    remainder.trim_start_matches(WHITESPACE_PATTERN)
}

pub fn is_whitespace(c: char) -> bool {
    WHITESPACE_PATTERN.contains(&c)
}

pub fn is_identifier_start(c: char) -> bool {
    c > '\u{0080}' || c.is_ascii_alphabetic() || c == '_'
}

pub fn is_identifier_continue(c: char) -> bool {
    is_identifier_start(c) || c.is_ascii_digit() || c == '-'
}

pub fn case_insensitive_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_chars = a.chars();
    let b_chars = b.chars();

    for (a, b) in a_chars.zip(b_chars) {
        if a.to_ascii_lowercase() != b.to_ascii_lowercase() {
            return false;
        }
    }

    true
}
