use super::{parse_relative_complex_selector, skip_whitespace, ParseError, RelativeSelectorList};

pub fn parse_relative_selector_list(
    input_remainder: &str,
    offset: usize,
) -> Result<(RelativeSelectorList, &str), ParseError> {
    let mut relative_selector_list = Vec::new();

    let remainder = skip_whitespace(input_remainder);

    let (complex_selector, remainder) = parse_relative_complex_selector(
        remainder,
        offset + input_remainder.len() - remainder.len(),
    )?;

    relative_selector_list.push(complex_selector);

    let mut remainder = skip_whitespace(remainder);

    loop {
        let mut chars = remainder.chars();

        if chars.next() == Some(',') {
            let r = chars.as_str();

            let (complex_selector, r) =
                parse_relative_complex_selector(r, offset + input_remainder.len() - r.len())?;

            relative_selector_list.push(complex_selector);

            remainder = skip_whitespace(r);
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
        assert_eq!(
            parse_relative_selector_list("* ", 0),
            Ok((
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
                },
                ""
            ))
        )
    }

    #[test]
    fn valid_two_complex_selector() {
        assert_eq!(
            parse_relative_selector_list("*, > * ", 0),
            Ok((
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
                },
                ""
            ))
        )
    }

    #[test]
    fn empty() {
        assert!(parse_relative_selector_list("", 0).is_err())
    }

    #[test]
    fn empty_first_list_element() {
        assert!(parse_relative_selector_list(", *", 0).is_err())
    }

    #[test]
    fn empty_second_list_element() {
        assert!(parse_relative_selector_list("*, , *", 0).is_err())
    }
}
