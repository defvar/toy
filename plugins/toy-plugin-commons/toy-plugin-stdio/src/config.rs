use toy_pack::{Schema, Unpack};

#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct StdinConfig {}

#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct StdoutConfig {}
