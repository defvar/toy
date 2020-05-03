#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    Number,
    String,
    True,
    False,
    Null,
    ValueSeparator,
    NameSeparator,
    Unexpected(u8),
}
