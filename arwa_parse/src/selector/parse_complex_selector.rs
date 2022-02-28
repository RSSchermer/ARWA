use super::{
    is_whitespace, parse_compound_selector, skip_whitespace, Combinator, CombinedSelector,
    ComplexSelector, ParseError,
};

pub fn parse_complex_selector(
    input_remainder: &str,
    offset: usize,
) -> Result<(ComplexSelector, &str), ParseError> {
    let (head, remainder) = parse_compound_selector(input_remainder, offset)?;

    let mut tail = Vec::new();
    let mut remainder = remainder;

    loop {
        let has_whitespace = if let Some(c) = remainder.chars().next() {
            is_whitespace(c)
        } else {
            break;
        };

        remainder = skip_whitespace(remainder);

        if remainder.is_empty() || remainder.starts_with(',') {
            break;
        }

        let (combinator, r) = if remainder.starts_with('>') {
            (Combinator::Child, &remainder[1..])
        } else if remainder.starts_with('+') {
            (Combinator::NextSibling, &remainder[1..])
        } else if remainder.starts_with('~') {
            (Combinator::SubsequentSibling, &remainder[1..])
        } else if remainder.starts_with("||") {
            (Combinator::Column, &remainder[2..])
        } else if has_whitespace {
            (Combinator::Descendant, remainder)
        } else {
            break;
        };

        let (selector, r) = parse_compound_selector(r, offset + input_remainder.len() - r.len())?;

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
        assert_eq!(
            parse_complex_selector("*, rest", 0),
            Ok((
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
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_single_compound_selector_no_rest() {
        assert_eq!(
            parse_complex_selector("* ", 0),
            Ok((
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
                ""
            ))
        )
    }

    #[test]
    fn valid_compound_descendant_of_compound() {
        assert_eq!(
            parse_complex_selector("* *, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_compound_child_of_compound() {
        assert_eq!(
            parse_complex_selector("*>*, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    fn valid_compound_child_of_compound_with_whitespace() {
        assert_eq!(
            parse_complex_selector("* > *, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_compound_next_sibling_of_compound() {
        assert_eq!(
            parse_complex_selector("*+*, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_compound_subsequent_sibling_of_compound() {
        assert_eq!(
            parse_complex_selector("*~*, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_compound_column_of_compound() {
        assert_eq!(
            parse_complex_selector("*||*, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_compound_descendant_of_compound_child_of_compound() {
        assert_eq!(
            parse_complex_selector("a b > c, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn empty() {
        assert!(parse_complex_selector("", 0).is_err())
    }

    #[test]
    fn invalid_first_char() {
        assert!(parse_complex_selector("> *", 0).is_err())
    }
}
