use super::{
    is_whitespace, parse_compound_selector, skip_whitespace, Combinator, CombinedSelector,
    ParseError, RelativeComplexSelector,
};

pub fn parse_relative_complex_selector(
    input_remainder: &str,
    offset: usize,
) -> Result<(RelativeComplexSelector, &str), ParseError> {
    let mut parts = Vec::new();
    let mut remainder = input_remainder;

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
        } else if has_whitespace || parts.is_empty() {
            (Combinator::Descendant, remainder)
        } else {
            break;
        };

        let (selector, r) = parse_compound_selector(r, offset + input_remainder.len() - r.len())?;

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
                position: offset + input_remainder.len() - remainder.len(),
            })
        } else {
            Err(ParseError {
                message: "unexpected end; expected relative selector".to_string(),
                position: offset + input_remainder.len(),
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
        assert_eq!(
            parse_relative_complex_selector("*, rest", 0),
            Ok((
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
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_single_compound_selector_child() {
        assert_eq!(
            parse_relative_complex_selector(">*, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_single_compound_selector_child_no_rest() {
        assert_eq!(
            parse_relative_complex_selector(">*", 0),
            Ok((
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
                },
                ""
            ))
        )
    }

    #[test]
    fn valid_single_compound_selector_child_with_space() {
        assert_eq!(
            parse_relative_complex_selector(" > *, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_single_compound_selector_next_sibling() {
        assert_eq!(
            parse_relative_complex_selector("+*, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_single_compound_selector_subsequent_sibling() {
        assert_eq!(
            parse_relative_complex_selector("~*, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_single_compound_selector_column() {
        assert_eq!(
            parse_relative_complex_selector("||*, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn valid_compound_descendant_of_compound_child() {
        assert_eq!(
            parse_relative_complex_selector("> * *, rest", 0),
            Ok((
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
                },
                ", rest"
            ))
        )
    }

    #[test]
    fn empty() {
        assert!(parse_relative_complex_selector("", 0).is_err())
    }

    #[test]
    fn invalid_first_char() {
        assert!(parse_relative_complex_selector("% *", 0).is_err())
    }
}
