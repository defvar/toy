#[derive(Debug, PartialEq, Hash, Copy, Clone)]
pub enum Algorithm {
    RS256,
}

impl Algorithm {
    pub(crate) fn convert(&self) -> jsonwebtoken::Algorithm {
        match self {
            Algorithm::RS256 => jsonwebtoken::Algorithm::RS256,
        }
    }
}
