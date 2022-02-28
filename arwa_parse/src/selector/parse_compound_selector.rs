use super::{
    is_identifier_start, parse_attribute_selector, parse_identifier, parse_pseudo_class_selector,
    skip_whitespace, CompoundSelector, ParseError, TypeSelector,
};

pub fn parse_compound_selector(
    input_remainder: &str,
    offset: usize,
) -> Result<(CompoundSelector, &str), ParseError> {
    let mut type_selector = None;
    let mut id_selector = None;
    let mut class_selectors = Vec::new();
    let mut attribute_selectors = Vec::new();
    let mut pseudo_class_selectors = Vec::new();

    let mut remainder = skip_whitespace(input_remainder);
    let mut is_empty = true;

    loop {
        let mut chars = remainder.chars();

        match chars.next() {
            Some('*') => {
                if !is_empty {
                    return Err(ParseError {
                        message: "unexpected type selector; a type selector must be the first \
                        selector in a compound selector"
                            .to_string(),
                        position: offset + input_remainder.len() - remainder.len(),
                    });
                } else if type_selector.is_some() {
                    return Err(ParseError {
                        message: "expected type selector, compound selector already contains a \
                        type selector"
                            .to_string(),
                        position: offset + input_remainder.len() - remainder.len(),
                    });
                } else {
                    type_selector = Some(TypeSelector::Universal);
                    remainder = chars.as_str();
                    is_empty = false;
                }
            }
            Some('#') => {
                if id_selector.is_some() {
                    return Err(ParseError {
                        message:
                            "unexpected ID selector, compound selector already contains an ID \
                        selector"
                                .to_string(),
                        position: offset + input_remainder.len() - remainder.len(),
                    });
                } else {
                    let (s, r) = parse_identifier(
                        chars.as_str(),
                        offset + input_remainder.len() - remainder.len() + 1,
                    )?;

                    id_selector = Some(s);
                    remainder = r;
                    is_empty = false;
                }
            }
            Some('.') => {
                let (s, r) = parse_identifier(
                    chars.as_str(),
                    offset + input_remainder.len() - remainder.len() + 1,
                )?;

                class_selectors.push(s);
                remainder = r;
                is_empty = false;
            }
            Some('[') => {
                let (s, r) = parse_attribute_selector(
                    chars.as_str(),
                    offset + input_remainder.len() - remainder.len() + 1,
                )?;

                attribute_selectors.push(s);
                remainder = r;
                is_empty = false;
            }
            Some(':') => {
                let (s, r) = parse_pseudo_class_selector(
                    chars.as_str(),
                    offset + input_remainder.len() - remainder.len() + 1,
                )?;

                pseudo_class_selectors.push(s);
                remainder = r;
                is_empty = false;
            }
            Some(c) if is_identifier_start(c) => {
                if !is_empty {
                    return Err(ParseError {
                        message: "unexpected type selector; a type selector must be the first \
                        selector in a compound selector"
                            .to_string(),
                        position: offset + input_remainder.len() - remainder.len(),
                    });
                } else if type_selector.is_some() {
                    return Err(ParseError {
                        message: "expected type selector, compound selector already contains a \
                        type selector"
                            .to_string(),
                        position: offset + input_remainder.len() - remainder.len(),
                    });
                } else {
                    let (s, r) = parse_identifier(
                        remainder,
                        offset + input_remainder.len() - remainder.len(),
                    )?;

                    type_selector = Some(TypeSelector::Identifier(s));
                    remainder = r;
                    is_empty = false;
                }
            }
            _ => break,
        }
    }

    if is_empty {
        if let Some(c) = remainder.chars().next() {
            Err(ParseError {
                message: format!("expected compound selector, found `{}`", c),
                position: offset + input_remainder.len() - remainder.len(),
            })
        } else {
            Err(ParseError {
                message: "unexpected end; expected compound selector".to_string(),
                position: offset + input_remainder.len() - remainder.len(),
            })
        }
    } else {
        let compound_selector = CompoundSelector {
            type_selector,
            id_selector,
            class_selectors,
            attribute_selectors,
            pseudo_class_selectors,
        };

        Ok((compound_selector, remainder))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::{
        AttributeMatcher, AttributeSelector, AttributeWithValue, CaseSensitivity, ComplexSelector,
        PseudoClassSelector, SelectorList,
    };

    #[test]
    fn valid_universal_type_only() {
        assert_eq!(
            parse_compound_selector("* rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: Some(TypeSelector::Universal),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_identifier_type_only() {
        assert_eq!(
            parse_compound_selector("type rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: Some(TypeSelector::Identifier(0..4)),
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_id_only() {
        assert_eq!(
            parse_compound_selector("#id rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: None,
                    id_selector: Some(1..3),
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_class_only() {
        assert_eq!(
            parse_compound_selector(".class rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: None,
                    id_selector: None,
                    class_selectors: vec![1..6],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_attribute_only() {
        assert_eq!(
            parse_compound_selector("[attr] rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: None,
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![AttributeSelector::Exists(1..5)],
                    pseudo_class_selectors: vec![]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_pseudo_class_only() {
        assert_eq!(
            parse_compound_selector(":checked rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: None,
                    id_selector: None,
                    class_selectors: vec![],
                    attribute_selectors: vec![],
                    pseudo_class_selectors: vec![PseudoClassSelector::Checked]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_one_of_each() {
        assert_eq!(
            parse_compound_selector("type#id.class[attr]:checked rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: Some(TypeSelector::Identifier(0..4)),
                    id_selector: Some(5..7),
                    class_selectors: vec![8..13],
                    attribute_selectors: vec![AttributeSelector::Exists(14..18)],
                    pseudo_class_selectors: vec![PseudoClassSelector::Checked]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_one_of_each_second_class() {
        assert_eq!(
            parse_compound_selector("type#id.class[attr]:checked.other_class rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: Some(TypeSelector::Identifier(0..4)),
                    id_selector: Some(5..7),
                    class_selectors: vec![8..13, 28..39],
                    attribute_selectors: vec![AttributeSelector::Exists(14..18)],
                    pseudo_class_selectors: vec![PseudoClassSelector::Checked]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_one_of_each_second_attribute() {
        assert_eq!(
            parse_compound_selector("type#id.class[attr]:checked[other_attr=something] rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: Some(TypeSelector::Identifier(0..4)),
                    id_selector: Some(5..7),
                    class_selectors: vec![8..13],
                    attribute_selectors: vec![
                        AttributeSelector::Exists(14..18),
                        AttributeSelector::WithValue(AttributeWithValue {
                            name: 28..38,
                            matcher: AttributeMatcher::Exact,
                            value: 39..48,
                            case_sensitivity: CaseSensitivity::Default
                        })
                    ],
                    pseudo_class_selectors: vec![PseudoClassSelector::Checked]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn valid_one_of_each_second_pseudo_class() {
        assert_eq!(
            parse_compound_selector("type#id.class[attr]:checked:is(*) rest", 0),
            Ok((
                CompoundSelector {
                    type_selector: Some(TypeSelector::Identifier(0..4)),
                    id_selector: Some(5..7),
                    class_selectors: vec![8..13],
                    attribute_selectors: vec![AttributeSelector::Exists(14..18)],
                    pseudo_class_selectors: vec![
                        PseudoClassSelector::Checked,
                        PseudoClassSelector::Is(SelectorList {
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
                        })
                    ]
                },
                " rest"
            ))
        )
    }

    #[test]
    fn empty() {
        assert!(parse_compound_selector("", 0).is_err())
    }

    #[test]
    fn non_compound_selector_char() {
        assert!(parse_compound_selector("> type#id.class[attr]:checked", 0).is_err())
    }

    #[test]
    fn one_of_each_second_type_identifier() {
        assert!(parse_compound_selector("type#id.class:checked[attr]second_type rest", 0).is_err())
    }

    #[test]
    fn one_of_each_second_universal_type() {
        assert!(parse_compound_selector("type#id.class:checked[attr]* rest", 0).is_err())
    }

    #[test]
    fn one_of_each_second_id() {
        assert!(parse_compound_selector("type#id.class[attr]:checked#second_id rest", 0).is_err())
    }
}
