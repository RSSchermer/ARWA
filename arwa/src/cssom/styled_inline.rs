use crate::cssom::CssStyleDeclaration;

pub(crate) mod styled_inline_seal {
    pub trait Seal {}
}

pub trait StyledInline: styled_inline_seal::Seal {
    fn style(&self) -> CssStyleDeclaration;
}
