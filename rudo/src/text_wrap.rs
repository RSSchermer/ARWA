#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextWrap {
    Hard,
    Soft,
}

impl Default for TextWrap {
    fn default() -> Self {
        TextWrap::Soft
    }
}
