use serde::{Deserialize, Serialize};
use toy_pack::Schema;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct FixedSizeConfig {
    pub(crate) size: usize,
}

impl FixedSizeConfig {
    pub fn with(size: usize) -> FixedSizeConfig {
        FixedSizeConfig { size }
    }
}
