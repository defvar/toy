use crate::Algorithm;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Validation {
    pub aud: Option<HashSet<String>>,
    pub iss: Option<HashSet<String>>,
    pub sub: Option<String>,
    algorithms: Vec<Algorithm>,
    pub kid: Option<String>,
    exp: bool,
}

impl Validation {
    pub fn new(alg: Algorithm) -> Self {
        Self {
            aud: None,
            iss: None,
            sub: None,
            algorithms: vec![alg],
            kid: None,
            exp: true,
        }
    }

    pub fn exp(self, v: bool) -> Self {
        Self { exp: v, ..self }
    }

    pub(crate) fn convert(self) -> jsonwebtoken::Validation {
        let mut r = jsonwebtoken::Validation::default();
        r.aud = self.aud;
        r.iss = self.iss;
        r.sub = self.sub;
        r.validate_exp = self.exp;
        r.algorithms = self.algorithms.iter().map(|x| x.convert()).collect();
        r
    }
}
