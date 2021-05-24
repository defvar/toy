use crate::Algorithm;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Validation {
    pub aud: Option<HashSet<String>>,
    pub iss: Option<String>,
    pub sub: Option<String>,
    pub algorithms: Vec<Algorithm>,
    pub kid: Option<String>,
}

impl Validation {
    pub fn new(alg: Algorithm) -> Self {
        Self {
            aud: None,
            iss: None,
            sub: None,
            algorithms: vec![alg],
            kid: None,
        }
    }

    pub(crate) fn convert(self) -> jsonwebtoken::Validation {
        let mut r = jsonwebtoken::Validation::default();
        r.aud = self.aud;
        r.iss = self.iss;
        r.sub = self.sub;
        r.algorithms = self.algorithms.iter().map(|x| x.convert()).collect();
        r
    }
}
