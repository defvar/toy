use serde::Deserialize;
use toy_pack::Schema;

#[derive(Debug, Clone, Default, Deserialize, Schema)]
pub struct FunctionConfig {
    pub code: String,
}
