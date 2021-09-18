use serde::{Deserialize, Serialize};
use toy_pack::Schema;

#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct FirstConfig {}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct LastConfig {}

#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
pub struct CountConfig {}
