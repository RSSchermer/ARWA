use crate::cssom::CssStyleSheet;

pub(crate) mod link_style_seal {
    pub trait Seal {}
}

pub trait LinkStyle {
    fn sheet(&self) -> Option<CssStyleSheet>;
}
