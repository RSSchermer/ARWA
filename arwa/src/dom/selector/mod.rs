use crate::DynamicElement;
use std::borrow::{Borrow, Cow};

pub trait Selector {
    type Compiled<'a>: Borrow<CompiledSelector>;

    fn compiled(&self) -> Self::Compiled;
}

impl<'a, T> Selector for &'a T
where
    T: Selector,
{
    type Compiled<'b> = T::Compiled<'a>;

    fn compiled(&self) -> Self::Compiled {
        self.compiled()
    }
}

enum CompiledSelectorInternal {
    Static(&'static str),
    Dynamic(String),
}

pub struct CompiledSelector {
    internal: CompiledSelectorInternal,
}

impl CompiledSelector {
    pub fn compile_from<T>(selector: &T) -> Self
    where
        T: CompilableSelector,
    {
        let compiled = String::with_capacity(selector.required_capacity());
        let mut compiler = SelectorCompiler { compiled };

        selector.compile(&mut compiler);

        CompiledSelector {
            internal: CompiledSelectorInternal::Dynamic(compiler.compiled),
        }
    }

    // pub const fn raw(raw: &'static str) -> Self {
    //     validator.validate(raw).unwrap();
    //
    //     CompiledSelector {
    //         internal: CompiledSelectorInternal::Static(raw)
    //     }
    // }
}

impl Selector for CompiledSelector {
    type Compiled<'a> = &'a CompiledSelector;

    fn compiled(&self) -> Self::Compiled {
        self
    }
}

impl<T> Selector for T
where
    T: CompilableSelector,
{
    type Compiled<'static> = CompiledSelector;

    fn compiled(&self) -> Self::Compiled {
        CompiledSelector::compile_from(self)
    }
}

pub struct SelectorCompiler {
    compiled: String,
}

impl SelectorCompiler {
    fn push_quoted_string(&mut self, string: &str) {
        self.compiled.push('"');

        for c in string.chars() {
            if c == '"' {
                self.compiled.push('\\');
            }
            self.compiled.push(c)
        }

        self.compiled.push('"');
    }

    fn push_unsigned_integer(&mut self, integer: u32) {
        // The greatest power of 10 that is smaller than `integer`.
        let mut divisor = if integer == 0 {
            1
        } else {
            10_f64.powf((integer as f64).log10().floor()) as usize
        };

        while divisor > 0 {
            let digit = (integer / divisor) % 10;

            self.compiled.push('0' + digit);

            divisor /= 10;
        }
    }
}

pub trait CompilableSelector {
    fn required_capacity(&self) -> usize;

    fn compile(&self, compiler: &mut SelectorCompiler);
}

mod compound_selector_seal {
    pub trait Seal {}
}

pub trait CompoundSelector: CompilableSelector + Sized + compound_selector_seal::Seal {
    fn is<T>(self, is: T) -> Is<Self, T>
    where
        T: CompilableSelector,
    {
        Is { base: self, is }
    }

    fn not<T>(self, not: T) -> Not<Self, T>
    where
        T: CompilableSelector,
    {
        Not { base: self, not }
    }

    fn where_<T>(self, where_: T) -> Where<Self, T>
    where
        T: CompilableSelector,
    {
        Where { base: self, where_ }
    }

    fn has<T>(self, has: T) -> Has<Self, T>
    where
        T: RelativeSelector,
    {
        Has { base: self, has }
    }

    fn dir(self, text_directionality: TextDirectionality) -> Dir<Self> {
        Dir {
            selector: self,
            text_directionality,
        }
    }

    fn lang(self, language: &str) -> Lang<Self> {
        Lang {
            selector: self,
            language,
        }
    }

    fn any_link(self) -> AnyLink<Self> {
        AnyLink { selector: self }
    }

    fn link(self) -> Link<Self> {
        Link { selector: self }
    }

    fn visited(self) -> Visited<Self> {
        Visited { selector: self }
    }

    fn local_link(self) -> LocalLink<Self> {
        LocalLink { selector: self }
    }

    fn target(self) -> Target<Self> {
        Target { selector: self }
    }

    fn target_within(self) -> TargetWithin<Self> {
        TargetWithin { selector: self }
    }

    fn scope(self) -> Scope<Self> {
        Scope { selector: self }
    }

    fn hover(self) -> Hover<Self> {
        Hover { selector: self }
    }

    fn active(self) -> Active<Self> {
        Active { selector: self }
    }

    fn focus(self) -> Focus<Self> {
        Focus { selector: self }
    }

    fn focus_visible(self) -> FocusVisible<Self> {
        FocusVisible { selector: self }
    }

    fn focus_within(self) -> FocusWithin<Self> {
        FocusWithin { selector: self }
    }

    fn current(self) -> Current<Self> {
        Current { selector: self }
    }

    fn past(self) -> Past<Self> {
        Past { selector: self }
    }

    fn future(self) -> Future<Self> {
        Future { selector: self }
    }

    fn playing(self) -> Playing<Self> {
        Playing { selector: self }
    }

    fn paused(self) -> Paused<Self> {
        Paused { selector: self }
    }

    fn enabled(self) -> Enabled<Self> {
        Enabled { selector: self }
    }

    fn disabled(self) -> Disabled<Self> {
        Disabled { selector: self }
    }

    fn read_only(self) -> ReadOnly<Self> {
        ReadOnly { selector: self }
    }

    fn read_write(self) -> ReadWrite<Self> {
        ReadWrite { selector: self }
    }

    fn placeholder_shown(self) -> PlaceholderShown<Self> {
        PlaceholderShown { selector: self }
    }

    fn default(self) -> Default<Self> {
        Default { selector: self }
    }

    fn checked(self) -> Checked<Self> {
        Checked { selector: self }
    }

    fn indeterminate(self) -> Indeterminate<Self> {
        Indeterminate { selector: self }
    }

    fn blank(self) -> Blank<Self> {
        Blank { selector: self }
    }

    fn valid(self) -> Valid<Self> {
        Valid { selector: self }
    }

    fn invalid(self) -> Invalid<Self> {
        Invalid { selector: self }
    }

    fn in_range(self) -> InRange<Self> {
        InRange { selector: self }
    }

    fn out_of_range(self) -> OutOfRange<Self> {
        OutOfRange { selector: self }
    }

    fn required(self) -> Required<Self> {
        Required { selector: self }
    }

    fn optional(self) -> Optional<Self> {
        Optional { selector: self }
    }

    fn user_invalid(self) -> UserInvalid<Self> {
        UserInvalid { selector: self }
    }

    fn root(self) -> Root<Self> {
        Root { selector: self }
    }

    fn empty(self) -> Empty<Self> {
        Empty { selector: self }
    }

    fn nth_child(self, a_n_plus_b: ANPlusB) -> NthChild<Self, ()> {
        NthChild {
            selector: self,
            a_n_plus_b,
            of: (),
        }
    }

    fn nth_child_of<S>(self, a_n_plus_b: ANPlusB, of: S) -> NthChild<Self, S>
    where
        S: CompilableSelector,
    {
        NthChild {
            selector: self,
            a_n_plus_b,
            of,
        }
    }

    fn nth_last_child(self, a_n_plus_b: ANPlusB) -> NthLastChild<Self, ()> {
        NthLastChild {
            selector: self,
            a_n_plus_b,
            of: (),
        }
    }

    fn nth_last_child_of<S>(self, a_n_plus_b: ANPlusB, of: S) -> NthLastChild<Self, S>
    where
        S: CompilableSelector,
    {
        NthLastChild {
            selector: self,
            a_n_plus_b,
            of,
        }
    }

    fn first_child(self) -> FirstChild<Self> {
        FirstChild { selector: self }
    }

    fn last_child(self) -> LastChild<Self> {
        LastChild { selector: self }
    }

    fn only_child(self) -> OnlyChild<Self> {
        OnlyChild { selector: self }
    }

    fn nth_of_type(self, a_n_plus_b: ANPlusB) -> NthOfType<Self> {
        NthOfType {
            selector: self,
            a_n_plus_b,
        }
    }

    fn nth_last_of_type(self, a_n_plus_b: ANPlusB) -> NthLastOfType<Self> {
        NthLastOfType {
            selector: self,
            a_n_plus_b,
        }
    }

    fn first_of_type(self) -> FirstOfType<Self> {
        FirstOfType { selector: self }
    }

    fn last_of_type(self) -> LastOfType<Self> {
        LastOfType { selector: self }
    }

    fn only_of_type(self) -> OnlyOfType<Self> {
        OnlyOfType { selector: self }
    }

    fn nth_col(self, a_n_plus_b: ANPlusB) -> NthCol<Self> {
        NthCol {
            selector: self,
            a_n_plus_b,
        }
    }

    fn nth_last_col(self, a_n_plus_b: ANPlusB) -> NthLastCol<Self> {
        NthLastCol {
            selector: self,
            a_n_plus_b,
        }
    }
}

pub struct SimpleCompoundSelector<'a> {
    pub type_selector: TypeSelector<'a>,
    pub id_selector: Option<Identifier<'a>>,
    pub class_selectors: &'a [Identifier<'a>],
    pub attribute_selectors: &'a [AttributeSelector<'a>],
}

impl CompilableSelector for SimpleCompoundSelector {
    fn required_capacity(&self) -> usize {
        let mut capacity = 0;

        capacity += self.type_selector.required_capacity();

        if let Some(id_selector) = &self.id_selector {
            capacity += id_selector.required_capacity();
        }

        for class_selector in self.class_selectors.borrow() {
            capacity += class_selector.required_capacity();
        }

        for attribute_selector in self.attribute_selectors.borrow() {
            capacity += attribute_selector.required_capacity();
        }

        capacity
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        compiler.compiled.push_str(self.type_selector.compiled());

        if let Some(id_selector) = &self.id_selector {
            compiler.compiled.push_str(id_selector.0);
        }
    }
}

impl compound_selector_seal::Seal for SimpleCompoundSelector {}
impl CompoundSelector for SimpleCompoundSelector {}

pub enum TypeSelector<'a> {
    Universal,
    Identifier(Identifier<'a>),
}

impl TypeSelector {
    fn required_capacity(&self) -> usize {
        match self {
            TypeSelector::Universal => 1,
            TypeSelector::Identifier(identifier) => identifier.required_capacity(),
        }
    }

    fn compiled(&self) -> &str {
        match self {
            TypeSelector::Universal => "*",
            TypeSelector::Identifier(identifier) => identifier.borrow(),
        }
    }
}

impl<'a> From<Identifier<'a>> for TypeSelector<'a> {
    fn from(identifier: Identifier<'a>) -> Self {
        TypeSelector::Identifier(identifier)
    }
}

pub struct InvalidIdentifier<'a>(&'a str);

pub struct Identifier<'a>(&'a str);

impl<'a> Identifier<'a> {
    const fn try_from(raw: &'a str) -> Result<Identifier<'a>, InvalidIdentifier<'a>> {
        if is_valid_identifier(raw) {
            Ok(Identifier(raw))
        } else {
            Err(InvalidIdentifier(raw))
        }
    }
}

const fn is_valid_identifier_first_char(c: &char) -> bool {
    c > &'\u{0080}' || c.is_ascii_alphabetic() || c == &'_'
}

const fn is_valid_identifier_char(c: &char) -> bool {
    is_valid_identifier_first_char(c) || c.is_ascii_digit() || c == &'-'
}

const fn is_valid_identifier(raw: &str) -> bool {
    let mut chars = raw.chars();
    let first = chars.next();

    if let Some(first) = first {
        if !is_valid_identifier_first_char(&first) {
            return false;
        }

        for c in chars {
            if !is_valid_identifier_char(&c) {
                return false;
            }
        }

        true
    } else {
        false
    }
}

impl Identifier<'_> {
    fn required_capacity(&self) -> usize {
        self.0.borrow().len()
    }
}

impl AsRef<str> for Identifier<'_> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct AttributeSelector<'a> {
    pub attribute_name: Identifier<'a>,
    pub kind: AttributeSelectorKind<'a>,
}

pub enum AttributeSelectorKind<'a> {
    Exists,
    Exact(&'a str),
    Includes(&'a str),
    BeginsWith(&'a str),
    EndsWith(&'a str),
    Substring(&'a str),
    HyphenatedBeginsWidth(&'a str),
}

pub struct Or<S0, S1> {
    s0: S0,
    s1: S1,
}

impl<S0, S1> CompilableSelector for Or<S0, S1>
where
    S0: CompilableSelector,
    S1: CompilableSelector,
{
    fn required_capacity(&self) -> usize {
        self.s0.required_capacity() + self.s1.required_capacity() + 2
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.s0.compiled(compiler);

        compiler.compiled.push_str(", ");

        self.s1.compiled(compiler);
    }
}

mod complex_selector_seal {
    pub trait Seal {}
}

pub trait ComplexSelector: CompilableSelector + Sized + complex_selector_seal::Seal {
    fn descendant_of<T>(self, base_selector: T) -> DescendantOf<T, Self>
    where
        T: ComplexSelector,
    {
        DescendantOf {
            base_selector,
            descendant_selector: Self,
        }
    }

    fn descendant(self) -> DescendantOf<(), Self> {
        DescendantOf {
            base_selector: (),
            descendant_selector: Self,
        }
    }

    fn child_of<T>(self, base_selector: T) -> ChildOf<T, Self>
    where
        T: ComplexSelector,
    {
        ChildOf {
            base_selector,
            child_selector: Self,
        }
    }

    fn child(self) -> ChildOf<(), Self> {
        ChildOf {
            base_selector: (),
            child_selector: Self,
        }
    }

    fn next_sibling_of<T>(self, base_selector: T) -> NextSiblingOf<T, Self>
    where
        T: ComplexSelector,
    {
        NextSiblingOf {
            base_selector,
            next_sibling_selector: self,
        }
    }

    fn next_sibling(self) -> NextSiblingOf<(), Self> {
        NextSiblingOf {
            base_selector: (),
            next_sibling_selector: Self,
        }
    }

    fn subsequent_sibling_of<T>(self, base_selector: T) -> SubsequentSiblingOf<T, Self>
    where
        T: ComplexSelector,
    {
        SubsequentSiblingOf {
            base_selector,
            subsequent_sibling_selector: self,
        }
    }

    fn subsequent_sibling(self) -> SubsequentSiblingOf<(), Self> {
        SubsequentSiblingOf {
            base_selector: (),
            subsequent_sibling_selector: Self,
        }
    }
}

impl<T> complex_selector_seal::Seal for T where T: CompoundSelector {}
impl<T> ComplexSelector for T where T: CompoundSelector {}

pub struct DescendantOf<S0, S1> {
    base_selector: S0,
    descendant_selector: S1,
}

impl<S0, S1> CompilableSelector for DescendantOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        self.base_selector.required_capacity() + self.descendant_selector.required_capacity() + 1
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.base_selector.compile(compiler);

        compiler.compiled.push_str(" ");

        self.descendant_selector.compile(compiler);
    }
}

impl<S0, S1> complex_selector_seal::Seal for DescendantOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
}
impl<S0, S1> ComplexSelector for DescendantOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
}

pub struct ChildOf<S0, S1> {
    base_selector: S0,
    child_selector: S1,
}

impl<S0, S1> CompilableSelector for ChildOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        self.base_selector.required_capacity() + self.child_selector.required_capacity() + 3
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.base_selector.compile(compiler);

        compiler.compiled.push_str(" > ");

        self.child_selector.compile(compiler);
    }
}

impl<S0, S1> complex_selector_seal::Seal for ChildOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
}
impl<S0, S1> ComplexSelector for ChildOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
}

pub struct NextSiblingOf<S0, S1> {
    base_selector: S0,
    next_sibling_selector: S1,
}

impl<S0, S1> CompilableSelector for NextSiblingOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        self.base_selector.required_capacity() + self.next_sibling_selector.required_capacity() + 3
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.base_selector.compile(compiler);

        compiler.compiled.push_str(" + ");

        self.next_sibling_selector.compile(compiler);
    }
}

impl<S0, S1> complex_selector_seal::Seal for NextSiblingOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
}
impl<S0, S1> ComplexSelector for NextSiblingOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
}

pub struct SubsequentSiblingOf<S0, S1> {
    base_selector: S0,
    subsequent_sibling_selector: S1,
}

impl<S0, S1> CompilableSelector for SubsequentSiblingOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        self.base_selector.required_capacity()
            + self.subsequent_sibling_selector.required_capacity()
            + 3
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.base_selector.compile(compiler);

        compiler.compiled.push_str(" ~ ");

        self.subsequent_sibling_selector.compile(compiler);
    }
}

impl<S0, S1> complex_selector_seal::Seal for SubsequentSiblingOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
}
impl<S0, S1> ComplexSelector for SubsequentSiblingOf<S0, S1>
where
    S0: ComplexSelector,
    S1: ComplexSelector,
{
}

pub struct Is<S, I> {
    base: S,
    is: I,
}

impl<S, I> CompilableSelector for Is<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
    fn required_capacity(&self) -> usize {
        self.base.required_capacity() + self.is.required_capacity() + 5
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.base.compile(compiler);

        compiler.compiled.push_str(":is(");

        self.is.compile(compiler);

        compiler.compiled.push_str(")");
    }
}

impl<S, I> compound_selector_seal::Seal for Is<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
}
impl<S, I> CompoundSelector for Is<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
}

pub struct Not<S, I> {
    base: S,
    not: I,
}

impl<S, I> CompilableSelector for Not<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
    fn required_capacity(&self) -> usize {
        self.base.required_capacity() + self.not.required_capacity() + 6
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.base.compile(compiler);

        compiler.compiled.push_str(":not(");

        self.not.compile(compiler);

        compiler.compiled.push_str(")");
    }
}

impl<S, I> compound_selector_seal::Seal for Not<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
}
impl<S, I> CompoundSelector for Not<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
}

pub struct Where<S, I> {
    base: S,
    where_: I,
}

impl<S, I> CompilableSelector for Where<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
    fn required_capacity(&self) -> usize {
        self.base.required_capacity() + self.where_.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.base.compile(compiler);

        compiler.compiled.push_str(":where(");

        self.where_.compile(compiler);

        compiler.compiled.push_str(")");
    }
}

impl<S, I> compound_selector_seal::Seal for Where<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
}
impl<S, I> CompoundSelector for Where<S, I>
where
    S: CompoundSelector,
    I: CompilableSelector,
{
}

mod relative_selector_seal {
    pub trait Seal {}
}

pub trait RelativeSelector: relative_selector_seal::Seal {
    fn required_capacity(&self) -> usize;

    fn compile(&self, compiler: &mut SelectorCompiler);
}

impl<T> relative_selector_seal::Seal for T where T: ComplexSelector {}
impl<T> RelativeSelector for T
where
    T: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        ComplexSelector::required_capacity(self)
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        ComplexSelector::compile(self, compiler);
    }
}

impl<S> relative_selector_seal::Seal for DescendantOf<(), S> where S: ComplexSelector {}
impl<S> RelativeSelector for DescendantOf<(), S>
where
    S: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        self.descendant_selector.required_capacity()
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.descendant_selector.compile(compiler);
    }
}

impl<S> relative_selector_seal::Seal for ChildOf<(), S> where S: ComplexSelector {}
impl<S> RelativeSelector for ChildOf<(), S>
where
    S: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        self.child_selector.required_capacity() + 2
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        compiler.compiled.push_str("> ");

        self.child_selector.compile(compiler);
    }
}

impl<S> relative_selector_seal::Seal for NextSiblingOf<(), S> where S: ComplexSelector {}
impl<S> RelativeSelector for NextSiblingOf<(), S>
where
    S: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        self.next_sibling_selector.required_capacity() + 2
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        compiler.compiled.push_str("+ ");

        self.next_sibling_selector.compile(compiler);
    }
}

impl<S> relative_selector_seal::Seal for SubsequentSiblingOf<(), S> where S: ComplexSelector {}
impl<S> RelativeSelector for SubsequentSiblingOf<(), S>
where
    S: ComplexSelector,
{
    fn required_capacity(&self) -> usize {
        self.subsequent_sibling_selector.required_capacity() + 2
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        compiler.compiled.push_str("~ ");

        self.subsequent_sibling_selector.compile(compiler);
    }
}

impl<S0, S1> relative_selector_seal::Seal for Or<S0, S1>
where
    S0: RelativeSelector,
    S1: RelativeSelector,
{
}
impl<S0, S1> RelativeSelector for Or<S0, S1>
where
    S0: RelativeSelector,
    S1: RelativeSelector,
{
    fn required_capacity(&self) -> usize {
        self.s0.required_capacity() + self.s1.required_capacity() + 2
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.s0.compile(compiler);

        compiler.compiled.push_str(", ");

        self.s1.compile(compiler);
    }
}

pub struct Has<S, I> {
    base: S,
    has: I,
}

impl<S, I> CompilableSelector for Has<S, I>
where
    S: CompoundSelector,
    I: RelativeSelector,
{
    fn required_capacity(&self) -> usize {
        self.base.required_capacity() + self.has.required_capacity() + 6
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.base.compile(compiler);

        compiler.compiled.push_str(":has(");

        self.has.compile(compiler);

        compiler.compiled.push_str(")");
    }
}

impl<S, I> compound_selector_seal::Seal for Has<S, I>
where
    S: CompoundSelector,
    I: RelativeSelector,
{
}
impl<S, I> CompoundSelector for Has<S, I>
where
    S: CompoundSelector,
    I: RelativeSelector,
{
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextDirectionality {
    LeftToRight,
    RightToLeft,
}

pub struct Dir<S> {
    selector: S,
    text_directionality: TextDirectionality,
}

impl<S> CompilableSelector for Dir<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        match self.text_directionality {
            TextDirectionality::LeftToRight => compiler.compiled.push_str(":dir(ltr)"),
            TextDirectionality::RightToLeft => compiler.compiled.push_str(":dir(rtl)"),
        }
    }
}

impl<S> compound_selector_seal::Seal for Dir<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Dir<S> where S: CompoundSelector {}

pub struct Lang<'a, S> {
    selector: S,
    language: &'a str,
}

impl<'a, S> CompilableSelector for Lang<'a, S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + self.language.len() + 9
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":lang(");

        compiler.push_quoted_string(self.language);

        compiler.compiled.push_str(")");
    }
}

impl<S> compound_selector_seal::Seal for Lang<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Lang<S> where S: CompoundSelector {}

pub struct AnyLink<S> {
    selector: S,
}

impl<S> CompilableSelector for AnyLink<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 9
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":any-link");
    }
}

impl<S> compound_selector_seal::Seal for AnyLink<S> where S: CompoundSelector {}
impl<S> CompoundSelector for AnyLink<S> where S: CompoundSelector {}

pub struct Link<S> {
    selector: S,
}

impl<S> CompilableSelector for Link<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 5
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":link");
    }
}

impl<S> compound_selector_seal::Seal for Link<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Link<S> where S: CompoundSelector {}

pub struct Visited<S> {
    selector: S,
}

impl<S> CompilableSelector for Visited<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":visited");
    }
}

impl<S> compound_selector_seal::Seal for Visited<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Visited<S> where S: CompoundSelector {}

pub struct LocalLink<S> {
    selector: S,
}

impl<S> CompilableSelector for LocalLink<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 11
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":local-link");
    }
}

impl<S> compound_selector_seal::Seal for LocalLink<S> where S: CompoundSelector {}
impl<S> CompoundSelector for LocalLink<S> where S: CompoundSelector {}

pub struct Target<S> {
    selector: S,
}

impl<S> CompilableSelector for Target<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 7
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":target");
    }
}

impl<S> compound_selector_seal::Seal for Target<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Target<S> where S: CompoundSelector {}

pub struct TargetWithin<S> {
    selector: S,
}

impl<S> CompilableSelector for TargetWithin<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 14
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":target-within");
    }
}

impl<S> compound_selector_seal::Seal for TargetWithin<S> where S: CompoundSelector {}
impl<S> CompoundSelector for TargetWithin<S> where S: CompoundSelector {}

pub struct Scope<S> {
    selector: S,
}

impl<S> CompilableSelector for Scope<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 6
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":scope");
    }
}

impl<S> compound_selector_seal::Seal for Scope<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Scope<S> where S: CompoundSelector {}

pub struct Hover<S> {
    selector: S,
}

impl<S> CompilableSelector for Hover<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 6
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":hover");
    }
}

impl<S> compound_selector_seal::Seal for Hover<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Hover<S> where S: CompoundSelector {}

pub struct Active<S> {
    selector: S,
}

impl<S> CompilableSelector for Active<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 7
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":active");
    }
}

impl<S> compound_selector_seal::Seal for Active<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Active<S> where S: CompoundSelector {}

pub struct Focus<S> {
    selector: S,
}

impl<S> CompilableSelector for Focus<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 6
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":focus");
    }
}

impl<S> compound_selector_seal::Seal for Focus<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Focus<S> where S: CompoundSelector {}

pub struct FocusVisible<S> {
    selector: S,
}

impl<S> CompilableSelector for FocusVisible<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 14
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":focus-visible");
    }
}

impl<S> compound_selector_seal::Seal for FocusVisible<S> where S: CompoundSelector {}
impl<S> CompoundSelector for FocusVisible<S> where S: CompoundSelector {}

pub struct FocusWithin<S> {
    selector: S,
}

impl<S> CompilableSelector for FocusWithin<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 13
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":focus-within");
    }
}

impl<S> compound_selector_seal::Seal for FocusWithin<S> where S: CompoundSelector {}
impl<S> CompoundSelector for FocusWithin<S> where S: CompoundSelector {}

pub struct Current<S> {
    selector: S,
}

impl<S> CompilableSelector for Current<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":current");
    }
}

impl<S> compound_selector_seal::Seal for Current<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Current<S> where S: CompoundSelector {}

pub struct Past<S> {
    selector: S,
}

impl<S> CompilableSelector for Past<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 5
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":past");
    }
}

impl<S> compound_selector_seal::Seal for Past<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Past<S> where S: CompoundSelector {}

pub struct Future<S> {
    selector: S,
}

impl<S> CompilableSelector for Future<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 7
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":future");
    }
}

impl<S> compound_selector_seal::Seal for Future<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Future<S> where S: CompoundSelector {}

pub struct Playing<S> {
    selector: S,
}

impl<S> CompilableSelector for Playing<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":playing");
    }
}

impl<S> compound_selector_seal::Seal for Playing<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Playing<S> where S: CompoundSelector {}

pub struct Paused<S> {
    selector: S,
}

impl<S> CompilableSelector for Paused<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 7
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":paused");
    }
}

impl<S> compound_selector_seal::Seal for Paused<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Paused<S> where S: CompoundSelector {}

pub struct Enabled<S> {
    selector: S,
}

impl<S> CompilableSelector for Enabled<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":enabled");
    }
}

impl<S> compound_selector_seal::Seal for Enabled<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Enabled<S> where S: CompoundSelector {}

pub struct Disabled<S> {
    selector: S,
}

impl<S> CompilableSelector for Disabled<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 9
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":disabled");
    }
}

impl<S> compound_selector_seal::Seal for Disabled<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Disabled<S> where S: CompoundSelector {}

pub struct ReadOnly<S> {
    selector: S,
}

impl<S> CompilableSelector for ReadOnly<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 10
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":read-only");
    }
}

impl<S> compound_selector_seal::Seal for ReadOnly<S> where S: CompoundSelector {}
impl<S> CompoundSelector for ReadOnly<S> where S: CompoundSelector {}

pub struct ReadWrite<S> {
    selector: S,
}

impl<S> CompilableSelector for ReadWrite<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 11
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":read-write");
    }
}

impl<S> compound_selector_seal::Seal for ReadWrite<S> where S: CompoundSelector {}
impl<S> CompoundSelector for ReadWrite<S> where S: CompoundSelector {}

pub struct PlaceholderShown<S> {
    selector: S,
}

impl<S> CompilableSelector for PlaceholderShown<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 18
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":placeholder-shown");
    }
}

impl<S> compound_selector_seal::Seal for PlaceholderShown<S> where S: CompoundSelector {}
impl<S> CompoundSelector for PlaceholderShown<S> where S: CompoundSelector {}

pub struct Default<S> {
    selector: S,
}

impl<S> CompilableSelector for Default<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":default");
    }
}

impl<S> compound_selector_seal::Seal for Default<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Default<S> where S: CompoundSelector {}

pub struct Checked<S> {
    selector: S,
}

impl<S> CompilableSelector for Checked<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":checked");
    }
}

impl<S> compound_selector_seal::Seal for Checked<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Checked<S> where S: CompoundSelector {}

pub struct Indeterminate<S> {
    selector: S,
}

impl<S> CompilableSelector for Indeterminate<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 14
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":indeterminate");
    }
}

impl<S> compound_selector_seal::Seal for Indeterminate<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Indeterminate<S> where S: CompoundSelector {}

pub struct Blank<S> {
    selector: S,
}

impl<S> CompilableSelector for Blank<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 6
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":blank");
    }
}

impl<S> compound_selector_seal::Seal for Blank<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Blank<S> where S: CompoundSelector {}

pub struct Valid<S> {
    selector: S,
}

impl<S> CompilableSelector for Valid<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 6
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":valid");
    }
}

impl<S> compound_selector_seal::Seal for Valid<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Valid<S> where S: CompoundSelector {}

pub struct Invalid<S> {
    selector: S,
}

impl<S> CompilableSelector for Invalid<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 8
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":invalid");
    }
}

impl<S> compound_selector_seal::Seal for Invalid<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Invalid<S> where S: CompoundSelector {}

pub struct InRange<S> {
    selector: S,
}

impl<S> CompilableSelector for InRange<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 9
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":in-range");
    }
}

impl<S> compound_selector_seal::Seal for InRange<S> where S: CompoundSelector {}
impl<S> CompoundSelector for InRange<S> where S: CompoundSelector {}

pub struct OutOfRange<S> {
    selector: S,
}

impl<S> CompilableSelector for OutOfRange<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 13
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":out-of-range");
    }
}

impl<S> compound_selector_seal::Seal for OutOfRange<S> where S: CompoundSelector {}
impl<S> CompoundSelector for OutOfRange<S> where S: CompoundSelector {}

pub struct Required<S> {
    selector: S,
}

impl<S> CompilableSelector for Required<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 9
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":required");
    }
}

impl<S> compound_selector_seal::Seal for Required<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Required<S> where S: CompoundSelector {}

pub struct Optional<S> {
    selector: S,
}

impl<S> CompilableSelector for Optional<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 9
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":optional");
    }
}

impl<S> compound_selector_seal::Seal for Optional<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Optional<S> where S: CompoundSelector {}

pub struct UserInvalid<S> {
    selector: S,
}

impl<S> CompilableSelector for UserInvalid<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 13
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":user-invalid");
    }
}

impl<S> compound_selector_seal::Seal for UserInvalid<S> where S: CompoundSelector {}
impl<S> CompoundSelector for UserInvalid<S> where S: CompoundSelector {}

pub struct Root<S> {
    selector: S,
}

impl<S> CompilableSelector for Root<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 5
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":root");
    }
}

impl<S> compound_selector_seal::Seal for Root<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Root<S> where S: CompoundSelector {}

pub struct Empty<S> {
    selector: S,
}

impl<S> CompilableSelector for Empty<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 6
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":empty");
    }
}

impl<S> compound_selector_seal::Seal for Empty<S> where S: CompoundSelector {}
impl<S> CompoundSelector for Empty<S> where S: CompoundSelector {}

pub enum ANPlusB {
    Even,
    Odd,
    ANPlusB(i32, i32),
}

impl ANPlusB {
    fn required_capacity(&self) -> usize {
        // The max string length for the absolute value of an i32 is 10 digits (2147483648), just
        // assume the max length, the over-allocation isn't going to break the bank
        match *self {
            ANPlusB::Even => 4,
            ANPlusB::Odd => 3,
            ANPlusB::ANPlusB(a, b) => {
                let an_size = if a == 0 {
                    // No `An` part.
                    0
                } else if a == 1 {
                    // `n`
                    1
                } else if a == -1 {
                    // `-n`
                    2
                } else {
                    if a < 0 {
                        // `-An`
                        12
                    } else {
                        // `An`
                        11
                    }
                };

                let b_size = if b == 0 {
                    0
                } else {
                    if a == 0 && b > 0 {
                        // Don't include a sign
                        10
                    } else {
                        // Do include a sign
                        11
                    }
                };

                an_size + b_size
            }
        }
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        match *self {
            ANPlusB::Even => compiler.compiled.push_str("even"),
            ANPlusB::Odd => compiler.compiled.push_str("odd"),
            ANPlusB::ANPlusB(a, b) => {
                if a == 1 {
                    compiler.compiled.push('n');
                } else if a == -1 {
                    compiler.compiled.push_str("-n");
                } else if a < 0 {
                    compiler.compiled.push('-');
                    compiler.push_unsigned_integer(a.unsigned_abs());
                    compiler.compiled.push('n');
                } else if a > 0 {
                    compiler.push_unsigned_integer(a.unsigned_abs());
                    compiler.compiled.push('n');
                }

                if b > 0 {
                    if a != 0 {
                        compiler.compiled.push('+');
                    }

                    compiler.push_unsigned_integer(b.unsigned_abs());
                } else if b < 0 {
                    compiler.compiled.push('-');
                    compiler.push_unsigned_integer(b.unsigned_abs());
                }
            }
        }
    }
}

pub struct NthChild<S, Of> {
    selector: S,
    a_n_plus_b: ANPlusB,
    of: Of,
}

impl<S> CompilableSelector for NthChild<S, ()>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + self.a_n_plus_b.required_capacity() + 12
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":nth-child(");

        self.a_n_plus_b.compile(compiler);

        compiler.push(')');
    }
}

impl<S> compound_selector_seal::Seal for NthChild<S, ()> where S: CompoundSelector {}
impl<S> CompoundSelector for NthChild<S, ()> where S: CompoundSelector {}

impl<S, Of> CompilableSelector for NthChild<S, Of>
where
    S: CompoundSelector,
    Of: CompilableSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity()
            + self.a_n_plus_b.required_capacity()
            + self.of.required_capacity()
            + 16
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":nth-child(");

        self.a_n_plus_b.compile(compiler);

        compiler.compiled.push_str(" of ");

        self.of.compile(compiler);

        compiler.push(')');
    }
}

impl<S, Of> compound_selector_seal::Seal for NthChild<S, Of>
where
    S: CompoundSelector,
    Of: CompilableSelector,
{
}
impl<S, Of> CompoundSelector for NthChild<S, Of>
where
    S: CompoundSelector,
    Of: CompilableSelector,
{
}

pub struct NthLastChild<S, Of> {
    selector: S,
    a_n_plus_b: ANPlusB,
    of: Of,
}

impl<S> CompilableSelector for NthLastChild<S, ()>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + self.a_n_plus_b.required_capacity() + 17
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":nth-last-child(");

        self.a_n_plus_b.compile(compiler);

        compiler.push(')');
    }
}

impl<S> compound_selector_seal::Seal for NthLastChild<S, ()> where S: CompoundSelector {}
impl<S> CompoundSelector for NthLastChild<S, ()> where S: CompoundSelector {}

impl<S, Of> CompilableSelector for NthLastChild<S, Of>
where
    S: CompoundSelector,
    Of: CompilableSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity()
            + self.a_n_plus_b.required_capacity()
            + self.of.required_capacity()
            + 21
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":nth-last-child(");

        self.a_n_plus_b.compile(compiler);

        compiler.compiled.push_str(" of ");

        self.of.compile(compiler);

        compiler.push(')');
    }
}

impl<S, Of> compound_selector_seal::Seal for NthLastChild<S, Of>
where
    S: CompoundSelector,
    Of: CompilableSelector,
{
}
impl<S, Of> CompoundSelector for NthLastChild<S, Of>
where
    S: CompoundSelector,
    Of: CompilableSelector,
{
}

pub struct FirstChild<S> {
    selector: S,
}

impl<S> CompilableSelector for FirstChild<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 12
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":first-child");
    }
}

impl<S> compound_selector_seal::Seal for FirstChild<S> where S: CompoundSelector {}
impl<S> CompoundSelector for FirstChild<S> where S: CompoundSelector {}

pub struct LastChild<S> {
    selector: S,
}

impl<S> CompilableSelector for LastChild<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 11
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":last-child");
    }
}

impl<S> compound_selector_seal::Seal for LastChild<S> where S: CompoundSelector {}
impl<S> CompoundSelector for LastChild<S> where S: CompoundSelector {}

pub struct OnlyChild<S> {
    selector: S,
}

impl<S> CompilableSelector for OnlyChild<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 11
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":only-child");
    }
}

impl<S> compound_selector_seal::Seal for OnlyChild<S> where S: CompoundSelector {}
impl<S> CompoundSelector for OnlyChild<S> where S: CompoundSelector {}

pub struct NthOfType<S> {
    selector: S,
    a_n_plus_b: ANPlusB,
}

impl<S> CompilableSelector for NthOfType<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + self.a_n_plus_b.required_capacity() + 14
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":nth-of-type(");

        self.a_n_plus_b.compile(compiler);

        compiler.push(')');
    }
}

impl<S> compound_selector_seal::Seal for NthOfType<S> where S: CompoundSelector {}
impl<S> CompoundSelector for NthOfType<S> where S: CompoundSelector {}

pub struct NthLastOfType<S> {
    selector: S,
    a_n_plus_b: ANPlusB,
}

impl<S> CompilableSelector for NthLastOfType<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + self.a_n_plus_b.required_capacity() + 19
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":nth-last-of-type(");

        self.a_n_plus_b.compile(compiler);

        compiler.push(')');
    }
}

impl<S> compound_selector_seal::Seal for NthLastOfType<S> where S: CompoundSelector {}
impl<S> CompoundSelector for NthLastOfType<S> where S: CompoundSelector {}

pub struct FirstOfType<S> {
    selector: S,
}

impl<S> CompilableSelector for FirstOfType<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 14
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":first-of-type");
    }
}

impl<S> compound_selector_seal::Seal for FirstOfType<S> where S: CompoundSelector {}
impl<S> CompoundSelector for FirstOfType<S> where S: CompoundSelector {}

pub struct LastOfType<S> {
    selector: S,
}

impl<S> CompilableSelector for LastOfType<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 13
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":last-of-type");
    }
}

impl<S> compound_selector_seal::Seal for LastOfType<S> where S: CompoundSelector {}
impl<S> CompoundSelector for LastOfType<S> where S: CompoundSelector {}

pub struct OnlyOfType<S> {
    selector: S,
}

impl<S> CompilableSelector for OnlyOfType<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + 13
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":only-of-type");
    }
}

impl<S> compound_selector_seal::Seal for OnlyOfType<S> where S: CompoundSelector {}
impl<S> CompoundSelector for OnlyOfType<S> where S: CompoundSelector {}

pub struct NthCol<S> {
    selector: S,
    a_n_plus_b: ANPlusB,
}

impl<S> CompilableSelector for NthCol<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + self.a_n_plus_b.required_capacity() + 10
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":nth-col(");

        self.a_n_plus_b.compile(compiler);

        compiler.push(')');
    }
}

impl<S> compound_selector_seal::Seal for NthCol<S> where S: CompoundSelector {}
impl<S> CompoundSelector for NthCol<S> where S: CompoundSelector {}

pub struct NthLastCol<S> {
    selector: S,
    a_n_plus_b: ANPlusB,
}

impl<S> CompilableSelector for NthLastCol<S>
where
    S: CompoundSelector,
{
    fn required_capacity(&self) -> usize {
        self.selector.required_capacity() + self.a_n_plus_b.required_capacity() + 15
    }

    fn compile(&self, compiler: &mut SelectorCompiler) {
        self.selector.compile(compiler);

        compiler.compiled.push_str(":nth-last-col(");

        self.a_n_plus_b.compile(compiler);

        compiler.push(')');
    }
}

impl<S> compound_selector_seal::Seal for NthLastCol<S> where S: CompoundSelector {}
impl<S> CompoundSelector for NthLastCol<S> where S: CompoundSelector {}
