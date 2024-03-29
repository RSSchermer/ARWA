#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Duration {
    Infinity,
    Milliseconds(u32),
}
