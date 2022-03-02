use super::{parse_complex_selector, ParseError, Remainder, SelectorList};

pub fn parse_selector_list(remainder: Remainder) -> Result<(SelectorList, Remainder), ParseError> {
    let mut selector_list = Vec::new();

    let remainder = remainder.skip_whitespace();

    let (complex_selector, remainder) = parse_complex_selector(remainder)?;

    selector_list.push(complex_selector);

    let mut remainder = remainder.skip_whitespace();

    loop {
        if remainder.starts_with(',') {
            remainder = remainder.skip(1);

            let (complex_selector, r) = parse_complex_selector(remainder)?;

            selector_list.push(complex_selector);

            remainder = r.skip_whitespace();
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
        let (selector, remainder) = parse_selector_list("* ".into()).unwrap();

        assert_eq!(
            selector,
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
            }
        );
        assert_eq!(remainder, "");
    }

    #[test]
    fn valid_two_complex_selector() {
        let (selector, remainder) = parse_selector_list("*, * ".into()).unwrap();

        assert_eq!(
            selector,
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
            }
        );
        assert_eq!(remainder, "");
    }

    #[test]
    fn empty() {
        assert!(parse_selector_list("".into()).is_err())
    }

    #[test]
    fn empty_first_list_element() {
        assert!(parse_selector_list(", *".into()).is_err())
    }

    #[test]
    fn empty_second_list_element() {
        assert!(parse_selector_list("*, , *".into()).is_err())
    }
}
