use crate::cssom::CssStyleDeclaration;

pub(crate) mod inline_style_context_seal {
    pub trait Seal {}
}

pub trait InlineStyleContext: inline_style_context_seal::Seal {
    fn style(&self) -> CssStyleDeclaration;
}
