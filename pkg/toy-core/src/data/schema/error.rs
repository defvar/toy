use failure::Fail;
use std::fmt::Display;
use toy_pack::schema;

#[derive(Debug, Fail)]
pub enum SchemaScanError {
    #[fail(display = "error: {:?}", inner)]
    Error { inner: String },
}

impl SchemaScanError {
    pub fn error<T>(msg: T) -> SchemaScanError
    where
        T: Display,
    {
        SchemaScanError::Error {
            inner: msg.to_string(),
        }
    }
}

impl schema::Error for SchemaScanError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        SchemaScanError::error(msg)
    }
}
