use super::{parse_complex_selector, skip_whitespace, ParseError, SelectorList};

pub fn parse_selector_list(
    input_remainder: &str,
    offset: usize,
) -> Result<(SelectorList, &str), ParseError> {
    let mut selector_list = Vec::new();

    let remainder = skip_whitespace(input_remainder);

    let (complex_selector, remainder) =
        parse_complex_selector(remainder, offset + input_remainder.len() - remainder.len())?;

    selector_list.push(complex_selector);

    let mut remainder = skip_whitespace(remainder);

    loop {
        let mut chars = remainder.chars();

        if chars.next() == Some(',') {
            let r = chars.as_str();

            let (complex_selector, r) =
                parse_complex_selector(r, offset + input_remainder.len() - r.len())?;

            selector_list.push(complex_selector);

            remainder = skip_whitespace(r);
        } else {
            break;
        }
    }

    let selector = SelectorList { selector_list };

    Ok((selector, remainder))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::{ComplexSelector, CompoundSelector, TypeSelector};

    #[test]
    fn valid_one_complex_selector() {
        assert_eq!(
            parse_selector_list("* ", 0),
            Ok((
                SelectorList {
                    selector_list: vec![ComplexSelector {
                        head: CompoundSelector {
                            type_selector: Some(TypeSelector::Universal),
                            id_selector: None,
                            class_selectors: vec![],
                            attribute_selectors: vec![],
                            pseudo_class_selectors: vec![]
                        },
                        tail: vec![]
                    }]
                },
                ""
            ))
        )
    }

    #[test]
    fn valid_two_complex_selector() {
        assert_eq!(
            parse_selector_list("*, * ", 0),
            Ok((
                SelectorList {
                    selector_list: vec![
                        ComplexSelector {
                            head: CompoundSelector {
                                type_selector: Some(TypeSelector::Universal),
                                id_selector: None,
                                class_selectors: vec![],
                                attribute_selectors: vec![],
                                pseudo_class_selectors: vec![]
                            },
                            tail: vec![]
                        },
                        ComplexSelector {
                            head: CompoundSelector {
                                type_selector: Some(TypeSelector::Universal),
                                id_selector: None,
                                class_selectors: vec![],
                                attribute_selectors: vec![],
                                pseudo_class_selectors: vec![]
                            },
                            tail: vec![]
                        }
                    ]
                },
                ""
            ))
        )
    }

    #[test]
    fn empty() {
        assert!(parse_selector_list("", 0).is_err())
    }

    #[test]
    fn empty_first_list_element() {
        assert!(parse_selector_list(", *", 0).is_err())
    }

    #[test]
    fn empty_second_list_element() {
        assert!(parse_selector_list("*, , *", 0).is_err())
    }
}
