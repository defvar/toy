use toy_pack::{Pack, Unpack};

#[derive(Debug, Pack, Unpack)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

impl ErrorMessage {
    pub fn new<P: Into<String>>(code: u16, message: P) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn code(&self) -> u16 {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
