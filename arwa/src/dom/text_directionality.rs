#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TextDirectionality {
    Auto,
    LeftToRight,
    RightToLeft,
}

impl Default for TextDirectionality {
    fn default() -> Self {
        TextDirectionality::Auto
    }
}
