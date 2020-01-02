#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectionDirection {
    Forward,
    Backward,
    None,
}

impl Default for SelectionDirection {
    fn default() -> Self {
        SelectionDirection::None
    }
}
