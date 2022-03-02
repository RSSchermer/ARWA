use super::{parse_relative_complex_selector, ParseError, RelativeSelectorList, Remainder};

pub fn parse_relative_selector_list(
    remainder: Remainder,
) -> Result<(RelativeSelectorList, Remainder), ParseError> {
    let mut relative_selector_list = Vec::new();

    let remainder = remainder.skip_whitespace();

    let (complex_selector, remainder) = parse_relative_complex_selector(remainder)?;

    relative_selector_list.push(complex_selector);

    let mut remainder = remainder.skip_whitespace();

    loop {
        if remainder.chars().next() == Some(',') {
            let r = remainder.skip(1);

            let (complex_selector, r) = parse_relative_complex_selector(r)?;

            relative_selector_list.push(complex_selector);

            remainder = r.skip_whitespace();
        } else {
            break;
        }
    }

    let relative_selector = RelativeSelectorList {
        relative_selector_list,
    };

    Ok((relative_selector, remainder))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::{
        Combinator, CombinedSelector, CompoundSelector, RelativeComplexSelector, TypeSelector,
    };

    #[test]
    fn valid_one_complex_selector() {
        let (selector, remainder) = parse_relative_selector_list("* ".into()).unwrap();

        assert_eq!(
            selector,
            RelativeSelectorList {
                relative_selector_list: vec![RelativeComplexSelector {
                    parts: vec![CombinedSelector {
                        combinator: Combinator::Descendant,
                        selector: CompoundSelector {
                            type_selector: Some(TypeSelector::Universal),
                            id_selector: None,
                            class_selectors: vec![],
                            attribute_selectors: vec![],
                            pseudo_class_selectors: vec![]
                        }
                    }]
                }]
            }
        );
        assert_eq!(remainder, "");
    }

    #[test]
    fn valid_two_complex_selector() {
        let (selector, remainder) = parse_relative_selector_list("*, > * ".into()).unwrap();

        assert_eq!(
            selector,
            RelativeSelectorList {
                relative_selector_list: vec![
                    RelativeComplexSelector {
                        parts: vec![CombinedSelector {
                            combinator: Combinator::Descendant,
                            selector: CompoundSelector {
                                type_selector: Some(TypeSelector::Universal),
                                id_selector: None,
                                class_selectors: vec![],
                                attribute_selectors: vec![],
                                pseudo_class_selectors: vec![]
                            }
                        }]
                    },
                    RelativeComplexSelector {
                        parts: vec![CombinedSelector {
                            combinator: Combinator::Child,
                            selector: CompoundSelector {
                                type_selector: Some(TypeSelector::Universal),
                                id_selector: None,
                                class_selectors: vec![],
                                attribute_selectors: vec![],
                                pseudo_class_selectors: vec![]
                            }
                        }]
                    }
                ]
            }
        );
        assert_eq!(remainder, "");
    }

    #[test]
    fn empty() {
        assert!(parse_relative_selector_list("".into()).is_err())
    }

    #[test]
    fn empty_first_list_element() {
        assert!(parse_relative_selector_list(", *".into()).is_err())
    }

    #[test]
    fn empty_second_list_element() {
        assert!(parse_relative_selector_list("*, , *".into()).is_err())
    }
}
