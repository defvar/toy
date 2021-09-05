use toy_pack::{Schema, Unpack};

#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct FunctionConfig {
    pub code: String,
}
