#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum State {
    StartRecord = 0,
    StartField = 1,
    InField = 2,
    InQuotedField = 3,
    InEscapedQuote = 4,
    InDoubleEscapedQuote = 5,
    InComment = 6,
    EndFieldDelimiter = 7,
    EndRecord = 8,
    CRLF = 9,

    EndFieldTerminator = 200,
    InRecordTerminator = 201,
    End = 202,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Action {
    ToOutput,
    Discard,
    None,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReadResult {
    InputEmpty,
    OutputFull,
    OutputEdgeFull,
    Record,
    End,
}

impl ReadResult {
    pub fn is_record(&self) -> bool {
        *self == ReadResult::Record
    }
}
