use super::{
    is_whitespace, parse_compound_selector, Combinator, CombinedSelector, ComplexSelector,
    ParseError, Remainder,
};

pub fn parse_complex_selector(
    remainder: Remainder,
) -> Result<(ComplexSelector, Remainder), ParseError> {
    let (head, remainder) = parse_compound_selector(remainder)?;

    let mut tail = Vec::new();
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
        } else if has_whitespace {
            (Combinator::Descendant, remainder)
        } else {
            break;
        };

        let (selector, r) = parse_compound_selector(r)?;

        remainder = r;

        tail.push(CombinedSelector {
            selector,
            combinator,
        });
    }

    let complex_selector = ComplexSelector { head, tail };

    Ok((complex_selector, remainder))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::{CompoundSelector, TypeSelector};

    #[test]
    fn valid_single_compound_selector() {
        let (selector, remainder) = parse_complex_selector("*, rest".into()).unwrap();

        assert_eq!(
            selector,
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
        );
        assert_eq!(remainder, ", rest");
    }

    #[test]
    fn valid_single_compound_selector_no_rest() {
        let (selector, remainder) = parse_complex_selector("* ".into()).unwrap();

        assert_eq!(
            selector,
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
        );
        assert_eq!(remainder, "");
    }

    #[test]
    fn valid_compound_descendant_of_compound() {
        let (selector, remainder) = parse_complex_selector("* *, rest".into()).unwrap();

        assert_eq!(
            selector,
            ComplexSelector {
                head: CompoundSelector {
                    type_selector: Some(TypeSelector::Universal),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                tail: vec![CombinedSelector {
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
    fn valid_compound_child_of_compound() {
        let (selector, remainder) = parse_complex_selector("*>*, rest".into()).unwrap();

        assert_eq!(
            selector,
            ComplexSelector {
                head: CompoundSelector {
                    type_selector: Some(TypeSelector::Universal),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                tail: vec![CombinedSelector {
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

    fn valid_compound_child_of_compound_with_whitespace() {
        let (selector, remainder) = parse_complex_selector("* > *, rest".into()).unwrap();

        assert_eq!(
            selector,
            ComplexSelector {
                head: CompoundSelector {
                    type_selector: Some(TypeSelector::Universal),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                tail: vec![CombinedSelector {
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
    fn valid_compound_next_sibling_of_compound() {
        let (selector, remainder) = parse_complex_selector("*+*, rest".into()).unwrap();

        assert_eq!(
            selector,
            ComplexSelector {
                head: CompoundSelector {
                    type_selector: Some(TypeSelector::Universal),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                tail: vec![CombinedSelector {
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
    fn valid_compound_subsequent_sibling_of_compound() {
        let (selector, remainder) = parse_complex_selector("*~*, rest".into()).unwrap();

        assert_eq!(
            selector,
            ComplexSelector {
                head: CompoundSelector {
                    type_selector: Some(TypeSelector::Universal),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                tail: vec![CombinedSelector {
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
    fn valid_compound_column_of_compound() {
        let (selector, remainder) = parse_complex_selector("*||*, rest".into()).unwrap();

        assert_eq!(
            selector,
            ComplexSelector {
                head: CompoundSelector {
                    type_selector: Some(TypeSelector::Universal),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                tail: vec![CombinedSelector {
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
    fn valid_compound_descendant_of_compound_child_of_compound() {
        let (selector, remainder) = parse_complex_selector("a b > c, rest".into()).unwrap();

        assert_eq!(
            selector,
            ComplexSelector {
                head: CompoundSelector {
                    type_selector: Some(TypeSelector::Identifier(0..1)),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                tail: vec![
                    CombinedSelector {
                        combinator: Combinator::Descendant,
                        selector: CompoundSelector {
                            type_selector: Some(TypeSelector::Identifier(2..3)),
                            id_selector: None,
                            class_selectors: vec![],
                            attribute_selectors: vec![],
                            pseudo_class_selectors: vec![]
                        }
                    },
                    CombinedSelector {
                        combinator: Combinator::Child,
                        selector: CompoundSelector {
                            type_selector: Some(TypeSelector::Identifier(6..7)),
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
        assert!(parse_complex_selector("".into()).is_err())
    }

    #[test]
    fn invalid_first_char() {
        assert!(parse_complex_selector("> *".into()).is_err())
    }
}
