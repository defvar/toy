use serde::{Deserialize, Serialize};
use toy_pack::Schema;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct StdinConfig {}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct StdoutConfig {}
