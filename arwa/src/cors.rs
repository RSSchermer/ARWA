#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CORS {
    Anonymous,
    UseCredentials,
}

impl Default for CORS {
    fn default() -> Self {
        CORS::Anonymous
    }
}
