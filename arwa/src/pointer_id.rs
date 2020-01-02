#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PointerId {
    inner: i32,
}

impl PointerId {
    pub(crate) fn new(inner: i32) -> Self {
        PointerId { inner }
    }
}

impl Into<i32> for PointerId {
    fn into(self) -> i32 {
        self.inner
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InvalidPointerId(pub(crate) PointerId);
