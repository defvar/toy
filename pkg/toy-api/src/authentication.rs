use toy_pack::{Pack, Unpack};

#[derive(Debug, Eq, PartialEq, Clone, Pack, Unpack)]
pub struct Claims {
    sub: String,
}

impl Claims {
    pub fn sub(&self) -> &str {
        &self.sub
    }
}
