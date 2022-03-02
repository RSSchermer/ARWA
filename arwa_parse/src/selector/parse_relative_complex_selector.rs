use super::{
    is_whitespace, parse_compound_selector, Combinator, CombinedSelector, ParseError,
    RelativeComplexSelector, Remainder,
};

pub fn parse_relative_complex_selector(
    remainder: Remainder,
) -> Result<(RelativeComplexSelector, Remainder), ParseError> {
    let mut parts = Vec::new();
    let mut remainder = remainder;

    loop {
        let has_whitespace = if let Some(c) = remainder.chars().next() {
            is_whitespace(c)
        } else {
            break;
        };

        remainder = remainder.skip_whitespace();

        if remainder.is_empty() || remainder.starts_with(',') {
            break;
        }

        let (combinator, r) = if remainder.starts_with('>') {
            (Combinator::Child, remainder.skip(1))
        } else if remainder.starts_with('+') {
            (Combinator::NextSibling, remainder.skip(1))
        } else if remainder.starts_with('~') {
            (Combinator::SubsequentSibling, remainder.skip(1))
        } else if remainder.starts_with("||") {
            (Combinator::Column, remainder.skip(2))
        } else if has_whitespace || parts.is_empty() {
            (Combinator::Descendant, remainder)
        } else {
            break;
        };

        let (selector, r) = parse_compound_selector(r)?;

        remainder = r;

        parts.push(CombinedSelector {
            selector,
            combinator,
        });
    }

    if parts.is_empty() {
        if let Some(c) = remainder.chars().next() {
            Err(ParseError {
                message: format!("expected relative selector, found `{}`", c),
                offset: remainder.offset(),
            })
        } else {
            Err(ParseError {
                message: "unexpected end; expected relative selector".to_string(),
                offset: remainder.offset(),
            })
        }
    } else {
        let relative_complex_selector = RelativeComplexSelector { parts };

        Ok((relative_complex_selector, remainder))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::{CompoundSelector, TypeSelector};

    #[test]
    fn valid_single_compound_selector_no_combinator() {
        let (selector, remainder) = parse_relative_complex_selector("*, rest".into()).unwrap();

        assert_eq!(
            selector,
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
            }
        );
        assert_eq!(remainder, ", rest");
    }

    #[test]
    fn valid_single_compound_selector_child() {
        let (selector, remainder) = parse_relative_complex_selector(">*, rest".into()).unwrap();

        assert_eq!(
            selector,
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
        );
        assert_eq!(remainder, ", rest");
    }

    #[test]
    fn valid_single_compound_selector_child_no_rest() {
        let (selector, remainder) = parse_relative_complex_selector(">*".into()).unwrap();

        assert_eq!(
            selector,
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
        );
        assert_eq!(remainder, "");
    }

    #[test]
    fn valid_single_compound_selector_child_with_space() {
        let (selector, remainder) = parse_relative_complex_selector(" > *, rest".into()).unwrap();

        assert_eq!(
            selector,
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
        );
        assert_eq!(remainder, ", rest");
    }

    #[test]
    fn valid_single_compound_selector_next_sibling() {
        let (selector, remainder) = parse_relative_complex_selector("+*, rest".into()).unwrap();

        assert_eq!(
            selector,
            RelativeComplexSelector {
                parts: vec![CombinedSelector {
                    combinator: Combinator::NextSibling,
                    selector: CompoundSelector {
                        type_selector: Some(TypeSelector::Universal),
                        id_selector: None,
                        class_selectors: vec![],
                        attribute_selectors: vec![],
                        pseudo_class_selectors: vec![]
                    }
                }]
            }
        );
        assert_eq!(remainder, ", rest");
    }

    #[test]
    fn valid_single_compound_selector_subsequent_sibling() {
        let (selector, remainder) = parse_relative_complex_selector("~*, rest".into()).unwrap();

        assert_eq!(
            selector,
            RelativeComplexSelector {
                parts: vec![CombinedSelector {
                    combinator: Combinator::SubsequentSibling,
                    selector: CompoundSelector {
                        type_selector: Some(TypeSelector::Universal),
                        id_selector: None,
                        class_selectors: vec![],
                        attribute_selectors: vec![],
                        pseudo_class_selectors: vec![]
                    }
                }]
            }
        );
        assert_eq!(remainder, ", rest");
    }

    #[test]
    fn valid_single_compound_selector_column() {
        let (selector, remainder) = parse_relative_complex_selector("||*, rest".into()).unwrap();

        assert_eq!(
            selector,
            RelativeComplexSelector {
                parts: vec![CombinedSelector {
                    combinator: Combinator::Column,
                    selector: CompoundSelector {
                        type_selector: Some(TypeSelector::Universal),
                        id_selector: None,
                        class_selectors: vec![],
                        attribute_selectors: vec![],
                        pseudo_class_selectors: vec![]
                    }
                }]
            }
        );
        assert_eq!(remainder, ", rest");
    }

    #[test]
    fn valid_compound_descendant_of_compound_child() {
        let (selector, remainder) = parse_relative_complex_selector("> * *, rest".into()).unwrap();

        assert_eq!(
            selector,
            RelativeComplexSelector {
                parts: vec![
                    CombinedSelector {
                        combinator: Combinator::Child,
                        selector: CompoundSelector {
                            type_selector: Some(TypeSelector::Universal),
                            id_selector: None,
                            class_selectors: vec![],
                            attribute_selectors: vec![],
                            pseudo_class_selectors: vec![]
                        }
                    },
                    CombinedSelector {
                        combinator: Combinator::Descendant,
                        selector: CompoundSelector {
                            type_selector: Some(TypeSelector::Universal),
                            id_selector: None,
                            class_selectors: vec![],
                            attribute_selectors: vec![],
                            pseudo_class_selectors: vec![]
                        }
                    }
                ]
            }
        );
        assert_eq!(remainder, ", rest");
    }

    #[test]
    fn empty() {
        assert!(parse_relative_complex_selector("".into()).is_err())
    }

    #[test]
    fn invalid_first_char() {
        assert!(parse_relative_complex_selector("% *".into()).is_err())
    }
}
