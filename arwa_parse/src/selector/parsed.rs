use std::ops::{Deref, Range};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectorList {
    pub(super) selector_list: Vec<ComplexSelector>,
}

impl Deref for SelectorList {
    type Target = [ComplexSelector];

    fn deref(&self) -> &Self::Target {
        &self.selector_list
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RelativeSelectorList {
    pub(super) relative_selector_list: Vec<RelativeComplexSelector>,
}

impl Deref for RelativeSelectorList {
    type Target = [RelativeComplexSelector];

    fn deref(&self) -> &Self::Target {
        &self.relative_selector_list
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ComplexSelector {
    pub(super) head: CompoundSelector,
    pub(super) tail: Vec<CombinedSelector>,
}

impl ComplexSelector {
    pub fn head(&self) -> &CompoundSelector {
        &self.head
    }

    pub fn tail(&self) -> &[CombinedSelector] {
        &self.tail
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RelativeComplexSelector {
    pub(super) parts: Vec<CombinedSelector>,
}

impl RelativeComplexSelector {
    pub fn parts(&self) -> &[CombinedSelector] {
        &self.parts
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CombinedSelector {
    pub(super) combinator: Combinator,
    pub(super) selector: CompoundSelector,
}

impl CombinedSelector {
    pub fn combinator(&self) -> Combinator {
        self.combinator
    }

    pub fn selector(&self) -> &CompoundSelector {
        &self.selector
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Combinator {
    Descendant,
    Child,
    NextSibling,
    SubsequentSibling,
    Column,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompoundSelector {
    pub(super) type_selector: Option<TypeSelector>,
    pub(super) id_selector: Option<Range<usize>>,
    pub(super) class_selectors: Vec<Range<usize>>,
    pub(super) attribute_selectors: Vec<AttributeSelector>,
    pub(super) pseudo_class_selectors: Vec<PseudoClassSelector>,
}

impl CompoundSelector {
    pub fn type_selector(&self) -> Option<TypeSelector> {
        self.type_selector.clone()
    }

    pub fn id_selector(&self) -> Option<Range<usize>> {
        self.id_selector.clone()
    }

    pub fn class_selectors(&self) -> &[Range<usize>] {
        &self.class_selectors
    }

    pub fn attribute_selectors(&self) -> &[AttributeSelector] {
        &self.attribute_selectors
    }

    pub fn pseudo_class_selectors(&self) -> &[PseudoClassSelector] {
        &self.pseudo_class_selectors
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TypeSelector {
    Universal,
    Identifier(Range<usize>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CaseSensitivity {
    Sensitive,
    Insensitive,
    Default,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum AttributeSelector {
    Exists(Range<usize>),
    WithValue(AttributeWithValue),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AttributeWithValue {
    pub(super) name: Range<usize>,
    pub(super) matcher: AttributeMatcher,
    pub(super) value: Range<usize>,
    pub(super) case_sensitivity: CaseSensitivity,
}

impl AttributeWithValue {
    pub fn name(&self) -> Range<usize> {
        self.name.clone()
    }

    pub fn matcher(&self) -> AttributeMatcher {
        self.matcher
    }

    pub fn value(&self) -> Range<usize> {
        self.value.clone()
    }

    pub fn case_sensitivity(&self) -> CaseSensitivity {
        self.case_sensitivity
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttributeMatcher {
    Exact,
    Includes,
    BeginsWith,
    EndsWith,
    Substring,
    HyphenatedBeginsWidth,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PseudoClassSelector {
    Is(SelectorList),
    Not(SelectorList),
    Where(SelectorList),
    Has(RelativeSelectorList),
    Dir(Range<usize>),
    Lang(Range<usize>),
    AnyLink,
    Link,
    Visited,
    LocalLink,
    Target,
    TargetWithin,
    Scope,
    Hover,
    Active,
    Focus,
    FocusVisible,
    FocusWithin,
    Current(Option<SelectorList>),
    Past,
    Future,
    Playing,
    Paused,
    Enabled,
    Disabled,
    ReadOnly,
    ReadWrite,
    PlaceholderShown,
    Default,
    Checked,
    Indeterminate,
    Blank,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    Required,
    Optional,
    UserInvalid,
    Root,
    Empty,
    NthChild(ANPlusBOf),
    NthLastChild(ANPlusBOf),
    FirstChild,
    LastChild,
    OnlyChild,
    NthOfType(ANPlusB),
    NthLastOfType(ANPlusB),
    FirstOfType,
    LastOfType,
    OnlyOfType,
    NthCol(ANPlusB),
    NthLastCol(ANPlusB),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ANPlusB {
    Even,
    Odd,
    ANPlusB(i32, i32),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ANPlusBOf {
    pub(super) a_n_plus_b: ANPlusB,
    pub(super) of: Option<SelectorList>,
}

impl ANPlusBOf {
    pub fn a_n_plus_b(&self) -> ANPlusB {
        self.a_n_plus_b
    }

    pub fn of(&self) -> Option<&SelectorList> {
        self.of.as_ref()
    }
}
