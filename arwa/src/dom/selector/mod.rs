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
    type Compiled = CompiledSelector;

    fn compiled(&self) -> Self::Compiled {
        todo!()
    }
}

impl AsRef<str> for CompiledSelector {
    fn as_ref(&self) -> &str {
        match &self.internal {
            CompiledSelectorInternal::Static(selector) => selector,
            CompiledSelectorInternal::Dynamic(selector) => selector,
        }
    }
}

pub struct SelectorCompiler {
    compiled: String,
}

pub trait CompilableSelector {
    fn required_capacity(&self) -> usize;

    fn compile(&self, compiler: &mut SelectorCompiler);
}

pub struct CompoundSelector<'a> {
    type_selector: TypeSelector<'a>,
    id_selector: Option<Identifier<'a>>,
    class_selectors: &'a [Identifier<'a>],
    attribute_selectors: &'a [AttributeSelector<'a>],
}

impl CompilableSelector for CompoundSelector {
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

pub struct Identifier<'a>(&'a str);

impl Identifier<'_> {
    fn required_capacity(&self) -> usize {
        self.0.borrow().len()
    }
}

impl Borrow<str> for Identifier<'_> {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

pub struct AttributeSelector<'a> {
    attribute_name: Identifier<'a>,
    kind: AttributeSelectorKind<'a>,
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

pub struct FunctionalPseudoClass<'a> {
    identifier: Identifier<'a>,
}

pub struct DescendantOf<S0, S1> {
    parent_selector: S0,
    descendant_selector: S1,
}

pub struct ChildOf<S0, S1> {
    parent_selector: S0,
    child_selector: S1,
}

pub struct NextSiblingOf<S0, S1> {
    reference_selector: S0,
    next_sibling_selector: S1,
}

pub struct SubsequentSiblingOf<S0, S1> {
    reference_selector: S0,
    subsequent_sibling_selector: S1,
}
