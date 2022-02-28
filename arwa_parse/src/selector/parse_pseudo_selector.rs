use super::{
    case_insensitive_eq, parse_a_n_plus_b, parse_a_n_plus_b_of, parse_function_invocation,
    parse_identifier, parse_identifier_or_string, parse_relative_selector_list,
    parse_selector_list, ParseError, PseudoClassSelector,
};

pub fn parse_pseudo_class_selector(
    input_remainder: &str,
    offset: usize,
) -> Result<(PseudoClassSelector, &str), ParseError> {
    let (identifier, remainder) = parse_identifier(input_remainder, offset)?;
    let identifier = &input_remainder[..identifier.len()];

    if case_insensitive_eq(identifier, "is") {
        let (selector, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_selector_list,
        )?;

        Ok((PseudoClassSelector::Is(selector), remainder))
    } else if case_insensitive_eq(identifier, "not") {
        let (selector, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_selector_list,
        )?;

        Ok((PseudoClassSelector::Not(selector), remainder))
    } else if case_insensitive_eq(identifier, "where") {
        let (selector, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_selector_list,
        )?;

        Ok((PseudoClassSelector::Where(selector), remainder))
    } else if case_insensitive_eq(identifier, "has") {
        let (relative_selector, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_relative_selector_list,
        )?;

        Ok((PseudoClassSelector::Has(relative_selector), remainder))
    } else if case_insensitive_eq(identifier, "dir") {
        let (relative_selector, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_identifier,
        )?;

        Ok((PseudoClassSelector::Dir(relative_selector), remainder))
    } else if case_insensitive_eq(identifier, "lang") {
        let (relative_selector, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_identifier_or_string,
        )?;

        Ok((PseudoClassSelector::Lang(relative_selector), remainder))
    } else if case_insensitive_eq(identifier, "any-link") {
        Ok((PseudoClassSelector::AnyLink, remainder))
    } else if case_insensitive_eq(identifier, "link") {
        Ok((PseudoClassSelector::Link, remainder))
    } else if case_insensitive_eq(identifier, "visited") {
        Ok((PseudoClassSelector::Visited, remainder))
    } else if case_insensitive_eq(identifier, "local-link") {
        Ok((PseudoClassSelector::LocalLink, remainder))
    } else if case_insensitive_eq(identifier, "target") {
        Ok((PseudoClassSelector::Target, remainder))
    } else if case_insensitive_eq(identifier, "target-within") {
        Ok((PseudoClassSelector::TargetWithin, remainder))
    } else if case_insensitive_eq(identifier, "scope") {
        Ok((PseudoClassSelector::Scope, remainder))
    } else if case_insensitive_eq(identifier, "hover") {
        Ok((PseudoClassSelector::Hover, remainder))
    } else if case_insensitive_eq(identifier, "active") {
        Ok((PseudoClassSelector::Active, remainder))
    } else if case_insensitive_eq(identifier, "focus") {
        Ok((PseudoClassSelector::Focus, remainder))
    } else if case_insensitive_eq(identifier, "focus-visible") {
        Ok((PseudoClassSelector::FocusVisible, remainder))
    } else if case_insensitive_eq(identifier, "focus-within") {
        Ok((PseudoClassSelector::FocusWithin, remainder))
    } else if case_insensitive_eq(identifier, "current") {
        let (selector, remainder) = if remainder.starts_with('(') {
            let (selector, remainder) = parse_function_invocation(
                remainder,
                offset + input_remainder.len() - remainder.len(),
                parse_selector_list,
            )?;

            (Some(selector), remainder)
        } else {
            (None, remainder)
        };

        Ok((PseudoClassSelector::Current(selector), remainder))
    } else if case_insensitive_eq(identifier, "past") {
        Ok((PseudoClassSelector::Past, remainder))
    } else if case_insensitive_eq(identifier, "future") {
        Ok((PseudoClassSelector::Future, remainder))
    } else if case_insensitive_eq(identifier, "playing") {
        Ok((PseudoClassSelector::Playing, remainder))
    } else if case_insensitive_eq(identifier, "paused") {
        Ok((PseudoClassSelector::Paused, remainder))
    } else if case_insensitive_eq(identifier, "enabled") {
        Ok((PseudoClassSelector::Enabled, remainder))
    } else if case_insensitive_eq(identifier, "disabled") {
        Ok((PseudoClassSelector::Disabled, remainder))
    } else if case_insensitive_eq(identifier, "read-only") {
        Ok((PseudoClassSelector::ReadOnly, remainder))
    } else if case_insensitive_eq(identifier, "read-write") {
        Ok((PseudoClassSelector::ReadWrite, remainder))
    } else if case_insensitive_eq(identifier, "placeholder-shown") {
        Ok((PseudoClassSelector::PlaceholderShown, remainder))
    } else if case_insensitive_eq(identifier, "default") {
        Ok((PseudoClassSelector::Default, remainder))
    } else if case_insensitive_eq(identifier, "checked") {
        Ok((PseudoClassSelector::Checked, remainder))
    } else if case_insensitive_eq(identifier, "indeterminate") {
        Ok((PseudoClassSelector::Indeterminate, remainder))
    } else if case_insensitive_eq(identifier, "blank") {
        Ok((PseudoClassSelector::Blank, remainder))
    } else if case_insensitive_eq(identifier, "valid") {
        Ok((PseudoClassSelector::Valid, remainder))
    } else if case_insensitive_eq(identifier, "invalid") {
        Ok((PseudoClassSelector::Invalid, remainder))
    } else if case_insensitive_eq(identifier, "in-range") {
        Ok((PseudoClassSelector::InRange, remainder))
    } else if case_insensitive_eq(identifier, "out-of-range") {
        Ok((PseudoClassSelector::OutOfRange, remainder))
    } else if case_insensitive_eq(identifier, "required") {
        Ok((PseudoClassSelector::Required, remainder))
    } else if case_insensitive_eq(identifier, "optional") {
        Ok((PseudoClassSelector::Optional, remainder))
    } else if case_insensitive_eq(identifier, "user-invalid") {
        Ok((PseudoClassSelector::UserInvalid, remainder))
    } else if case_insensitive_eq(identifier, "root") {
        Ok((PseudoClassSelector::Root, remainder))
    } else if case_insensitive_eq(identifier, "empty") {
        Ok((PseudoClassSelector::Empty, remainder))
    } else if case_insensitive_eq(identifier, "nth-child") {
        let (a_n_plus_b_of, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_a_n_plus_b_of,
        )?;

        Ok((PseudoClassSelector::NthChild(a_n_plus_b_of), remainder))
    } else if case_insensitive_eq(identifier, "nth-last-child") {
        let (a_n_plus_b_of, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_a_n_plus_b_of,
        )?;

        Ok((PseudoClassSelector::NthLastChild(a_n_plus_b_of), remainder))
    } else if case_insensitive_eq(identifier, "first-child") {
        Ok((PseudoClassSelector::FirstChild, remainder))
    } else if case_insensitive_eq(identifier, "last-child") {
        Ok((PseudoClassSelector::LastChild, remainder))
    } else if case_insensitive_eq(identifier, "only-child") {
        Ok((PseudoClassSelector::LastChild, remainder))
    } else if case_insensitive_eq(identifier, "nth-of-type") {
        let (a_n_plus_b, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_a_n_plus_b,
        )?;

        Ok((PseudoClassSelector::NthOfType(a_n_plus_b), remainder))
    } else if case_insensitive_eq(identifier, "nth-last-of-type") {
        let (a_n_plus_b, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_a_n_plus_b,
        )?;

        Ok((PseudoClassSelector::NthLastOfType(a_n_plus_b), remainder))
    } else if case_insensitive_eq(identifier, "first-of-type") {
        Ok((PseudoClassSelector::FirstOfType, remainder))
    } else if case_insensitive_eq(identifier, "last-of-type") {
        Ok((PseudoClassSelector::LastOfType, remainder))
    } else if case_insensitive_eq(identifier, "only-of-type") {
        Ok((PseudoClassSelector::OnlyOfType, remainder))
    } else if case_insensitive_eq(identifier, "nth-col") {
        let (a_n_plus_b, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_a_n_plus_b,
        )?;

        Ok((PseudoClassSelector::NthCol(a_n_plus_b), remainder))
    } else if case_insensitive_eq(identifier, "nth-last-col") {
        let (a_n_plus_b, remainder) = parse_function_invocation(
            remainder,
            offset + input_remainder.len() - remainder.len(),
            parse_a_n_plus_b,
        )?;

        Ok((PseudoClassSelector::NthLastCol(a_n_plus_b), remainder))
    } else {
        Err(ParseError {
            message: format!("unknown pseudo-class identifier `{}`", identifier),
            position: offset + input_remainder.len() - remainder.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::selector::{
        ANPlusB, ANPlusBOf, Combinator, CombinedSelector, ComplexSelector, CompoundSelector,
        RelativeComplexSelector, RelativeSelectorList, SelectorList, TypeSelector,
    };

    #[test]
    fn valid_non_function_pseudo_class() {
        assert_eq!(
            parse_pseudo_class_selector("checked rest", 0),
            Ok((PseudoClassSelector::Checked, " rest"))
        );
    }

    #[test]
    fn valid_upper_case_non_function_pseudo_class() {
        assert_eq!(
            parse_pseudo_class_selector("CHECKED rest", 0),
            Ok((PseudoClassSelector::Checked, " rest"))
        );
    }

    #[test]
    fn unknown_non_function_pseudo_class() {
        assert!(parse_pseudo_class_selector("unknown rest", 0).is_err());
    }

    #[test]
    fn valid_is_selector() {
        assert_eq!(
            parse_pseudo_class_selector("is(*) rest", 0),
            Ok((
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
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn is_no_invocation() {
        assert!(parse_pseudo_class_selector("is rest", 0).is_err());
    }

    #[test]
    fn is_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("is( rest", 0).is_err());
    }

    #[test]
    fn is_no_selector() {
        assert!(parse_pseudo_class_selector("is() rest", 0).is_err());
    }

    #[test]
    fn valid_not_selector() {
        assert_eq!(
            parse_pseudo_class_selector("not(*) rest", 0),
            Ok((
                PseudoClassSelector::Not(SelectorList {
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
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn not_no_invocation() {
        assert!(parse_pseudo_class_selector("not rest", 0).is_err());
    }

    #[test]
    fn not_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("not( rest", 0).is_err());
    }

    #[test]
    fn not_no_selector() {
        assert!(parse_pseudo_class_selector("not() rest", 0).is_err());
    }

    #[test]
    fn valid_where_selector() {
        assert_eq!(
            parse_pseudo_class_selector("where(*) rest", 0),
            Ok((
                PseudoClassSelector::Where(SelectorList {
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
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn where_no_invocation() {
        assert!(parse_pseudo_class_selector("where rest", 0).is_err());
    }

    #[test]
    fn where_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("where( rest", 0).is_err());
    }

    #[test]
    fn where_no_selector() {
        assert!(parse_pseudo_class_selector("where() rest", 0).is_err());
    }

    #[test]
    fn valid_has_descendant_selector() {
        assert_eq!(
            parse_pseudo_class_selector("has(*) rest", 0),
            Ok((
                PseudoClassSelector::Has(RelativeSelectorList {
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
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn valid_has_child_selector() {
        assert_eq!(
            parse_pseudo_class_selector("has(> *) rest", 0),
            Ok((
                PseudoClassSelector::Has(RelativeSelectorList {
                    relative_selector_list: vec![RelativeComplexSelector {
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
                    }]
                }),
                " rest"
            ))
        );
    }

    #[test]
    fn has_no_invocation() {
        assert!(parse_pseudo_class_selector("has rest", 0).is_err());
    }

    #[test]
    fn has_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("has( rest", 0).is_err());
    }

    #[test]
    fn has_no_selector() {
        assert!(parse_pseudo_class_selector("has() rest", 0).is_err());
    }

    #[test]
    fn valid_dir_selector() {
        assert_eq!(
            parse_pseudo_class_selector("dir(ltr) rest", 0),
            Ok((PseudoClassSelector::Dir(4..7), " rest"))
        );
    }

    #[test]
    fn valid_dir_selector_with_whitespace() {
        assert_eq!(
            parse_pseudo_class_selector("dir( ltr ) rest", 0),
            Ok((PseudoClassSelector::Dir(5..8), " rest"))
        );
    }

    #[test]
    fn dir_string() {
        assert!(parse_pseudo_class_selector("dir('ltr') rest", 0).is_err());
    }

    #[test]
    fn dir_no_invocation() {
        assert!(parse_pseudo_class_selector("dir rest", 0).is_err());
    }

    #[test]
    fn dir_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("dir( rest", 0).is_err());
    }

    #[test]
    fn dir_empty_invocation() {
        assert!(parse_pseudo_class_selector("dir( ) rest", 0).is_err());
    }

    #[test]
    fn valid_lang_selector_with_identifier() {
        assert_eq!(
            parse_pseudo_class_selector("lang(en) rest", 0),
            Ok((PseudoClassSelector::Lang(5..7), " rest"))
        );
    }

    #[test]
    fn valid_lang_selector_with_single_quoted_string() {
        assert_eq!(
            parse_pseudo_class_selector("lang('en') rest", 0),
            Ok((PseudoClassSelector::Lang(5..9), " rest"))
        );
    }

    #[test]
    fn valid_lang_selector_with_double_quoted_string() {
        assert_eq!(
            parse_pseudo_class_selector("lang(\"en\") rest", 0),
            Ok((PseudoClassSelector::Lang(5..9), " rest"))
        );
    }

    #[test]
    fn lang_no_invocation() {
        assert!(parse_pseudo_class_selector("lang rest", 0).is_err());
    }

    #[test]
    fn lang_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("lang( rest", 0).is_err());
    }

    #[test]
    fn lang_empty_invocation() {
        assert!(parse_pseudo_class_selector("lang( ) rest", 0).is_err());
    }

    #[test]
    fn valid_current_no_invocation() {
        assert_eq!(
            parse_pseudo_class_selector("current rest", 0),
            Ok((PseudoClassSelector::Current(None), " rest"))
        );
    }

    #[test]
    fn valid_current_with_selector() {
        assert_eq!(
            parse_pseudo_class_selector("current(*) rest", 0),
            Ok((
                PseudoClassSelector::Current(Some(SelectorList {
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
                })),
                " rest"
            ))
        );
    }

    #[test]
    fn current_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("current( rest", 0).is_err());
    }

    #[test]
    fn current_no_selector() {
        assert!(parse_pseudo_class_selector("current() rest", 0).is_err());
    }

    #[test]
    fn valid_nth_child() {
        assert_eq!(
            parse_pseudo_class_selector("nth-child(2n+1) rest", 0),
            Ok((
                PseudoClassSelector::NthChild(ANPlusBOf {
                    a_n_plus_b: ANPlusB::ANPlusB(2, 1),
                    of: None
                }),
                " rest"
            ))
        )
    }

    #[test]
    fn valid_nth_child_with_of() {
        assert_eq!(
            parse_pseudo_class_selector("nth-child(2n+1 of *) rest", 0),
            Ok((
                PseudoClassSelector::NthChild(ANPlusBOf {
                    a_n_plus_b: ANPlusB::ANPlusB(2, 1),
                    of: Some(SelectorList {
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
                }),
                " rest"
            ))
        )
    }

    #[test]
    fn nth_child_no_invocation() {
        assert!(parse_pseudo_class_selector("nth-child rest", 0).is_err());
    }

    #[test]
    fn nth_child_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("nth-child( rest", 0).is_err());
    }

    #[test]
    fn nth_child_empty_invocation() {
        assert!(parse_pseudo_class_selector("nth-child( ) rest", 0).is_err());
    }

    #[test]
    fn valid_nth_last_child() {
        assert_eq!(
            parse_pseudo_class_selector("nth-last-child(2n+1) rest", 0),
            Ok((
                PseudoClassSelector::NthLastChild(ANPlusBOf {
                    a_n_plus_b: ANPlusB::ANPlusB(2, 1),
                    of: None
                }),
                " rest"
            ))
        )
    }

    #[test]
    fn valid_nth_last_child_with_of() {
        assert_eq!(
            parse_pseudo_class_selector("nth-last-child(2n+1 of *) rest", 0),
            Ok((
                PseudoClassSelector::NthLastChild(ANPlusBOf {
                    a_n_plus_b: ANPlusB::ANPlusB(2, 1),
                    of: Some(SelectorList {
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
                }),
                " rest"
            ))
        )
    }

    #[test]
    fn nth_last_child_no_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-child rest", 0).is_err());
    }

    #[test]
    fn nth_last_child_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-child( rest", 0).is_err());
    }

    #[test]
    fn nth_last_child_empty_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-child( ) rest", 0).is_err());
    }

    #[test]
    fn valid_nth_of_type() {
        assert_eq!(
            parse_pseudo_class_selector("nth-of-type(2n+1) rest", 0),
            Ok((
                PseudoClassSelector::NthOfType(ANPlusB::ANPlusB(2, 1)),
                " rest"
            ))
        )
    }

    #[test]
    fn nth_of_type_with_of() {
        assert!(parse_pseudo_class_selector("nth-of-type(2n+1 of *) rest", 0).is_err())
    }

    #[test]
    fn nth_of_type_no_invocation() {
        assert!(parse_pseudo_class_selector("nth-of-type rest", 0).is_err());
    }

    #[test]
    fn nth_of_type_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("nth-of-type( rest", 0).is_err());
    }

    #[test]
    fn nth_of_type_empty_invocation() {
        assert!(parse_pseudo_class_selector("nth-of-type( ) rest", 0).is_err());
    }

    #[test]
    fn valid_nth_last_of_type() {
        assert_eq!(
            parse_pseudo_class_selector("nth-last-of-type(2n+1) rest", 0),
            Ok((
                PseudoClassSelector::NthLastOfType(ANPlusB::ANPlusB(2, 1)),
                " rest"
            ))
        )
    }

    #[test]
    fn nth_last_of_type_with_of() {
        assert!(parse_pseudo_class_selector("nth-last-of-type(2n+1 of *) rest", 0).is_err())
    }

    #[test]
    fn nth_last_of_type_no_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-of-type rest", 0).is_err());
    }

    #[test]
    fn nth_last_of_type_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-of-type( rest", 0).is_err());
    }

    #[test]
    fn nth_last_of_type_empty_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-of-type( ) rest", 0).is_err());
    }

    #[test]
    fn valid_nth_col() {
        assert_eq!(
            parse_pseudo_class_selector("nth-col(2n+1) rest", 0),
            Ok((PseudoClassSelector::NthCol(ANPlusB::ANPlusB(2, 1)), " rest"))
        )
    }

    #[test]
    fn nth_col_with_of() {
        assert!(parse_pseudo_class_selector("nth-col(2n+1 of *) rest", 0).is_err())
    }

    #[test]
    fn nth_col_no_invocation() {
        assert!(parse_pseudo_class_selector("nth-col rest", 0).is_err());
    }

    #[test]
    fn nth_col_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("nth-col( rest", 0).is_err());
    }

    #[test]
    fn nth_col_empty_invocation() {
        assert!(parse_pseudo_class_selector("nth-col( ) rest", 0).is_err());
    }

    #[test]
    fn valid_nth_last_col() {
        assert_eq!(
            parse_pseudo_class_selector("nth-last-col(2n+1) rest", 0),
            Ok((
                PseudoClassSelector::NthLastCol(ANPlusB::ANPlusB(2, 1)),
                " rest"
            ))
        )
    }

    #[test]
    fn nth_last_col_with_of() {
        assert!(parse_pseudo_class_selector("nth-last-col(2n+1 of *) rest", 0).is_err())
    }

    #[test]
    fn nth_last_col_no_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-col rest", 0).is_err());
    }

    #[test]
    fn nth_last_col_unclosed_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-col( rest", 0).is_err());
    }

    #[test]
    fn nth_last_col_empty_invocation() {
        assert!(parse_pseudo_class_selector("nth-last-col( ) rest", 0).is_err());
    }

    #[test]
    fn empty() {
        assert!(parse_pseudo_class_selector("", 0).is_err())
    }
}
