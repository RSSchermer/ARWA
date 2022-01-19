pub(crate) mod inline_style_seal {
    pub trait Seal {}
}

pub trait InlineStyle: inline_style_seal::Seal {
    fn style(&self) -> CssStyleDeclaration;
}
