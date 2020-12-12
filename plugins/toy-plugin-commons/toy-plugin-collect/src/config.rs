use toy_pack::{Schema, Unpack};

#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct FirstConfig {}

#[derive(Debug, Clone, Default, Unpack, Schema)]
pub struct LastConfig {}
