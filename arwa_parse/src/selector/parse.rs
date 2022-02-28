use super::{parse_selector_list, SelectorList};

#[derive(Clone, PartialEq, Debug)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

pub fn parse(selector_string: &str) -> Result<SelectorList, ParseError> {
    let (selector, remainder) = parse_selector_list(selector_string, 0)?;

    // Note: parse_selector will have consumed all trailing whitespace already.

    if let Some(c) = remainder.chars().next() {
        Err(ParseError {
            message: format!("unexpected character `{}`", c),
            position: selector_string.len() - remainder.len(),
        })
    } else {
        Ok(selector)
    }
}
