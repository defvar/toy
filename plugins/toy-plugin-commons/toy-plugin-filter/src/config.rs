use serde::{Deserialize, Serialize};
use toy_pack::Schema;
use crate::predicate::Predicate;

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct FilterConfig {
    preds: Vec<Predicate>,
}

impl FilterConfig {
    pub fn with(preds: &[Predicate]) -> FilterConfig {
        FilterConfig {
            preds: preds.to_vec()
        }
    }

    pub fn preds(&self) -> &[Predicate] {
        &self.preds
    }
}
