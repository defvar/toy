#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    /// "{"
    BeginObject,
    /// "}"
    EndObject,
    /// "["
    BeginArray,
    /// "]"
    EndArray,
    /// "-" & "0-9"
    Number,
    /// """
    String,
    /// "t"
    True,
    /// "f"
    False,
    /// "n"
    Null,
    /// ","
    Comma,
    /// ":"
    Colon,
    /// other
    Unexpected(u8),
}
