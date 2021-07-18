use crate::Algorithm;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Header {
    pub alg: Algorithm,
    pub kid: Option<String>,
}
