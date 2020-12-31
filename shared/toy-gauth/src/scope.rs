#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Scope {
    LoggingAdmin,
    LoggingRead,
    LoggingWrite,
}

impl Scope {
    pub fn uri(&self) -> &str {
        match self {
            Scope::LoggingAdmin => "https://www.googleapis.com/auth/logging.admin",
            Scope::LoggingRead => "https://www.googleapis.com/auth/logging.read",
            Scope::LoggingWrite => "https://www.googleapis.com/auth/logging.write",
        }
    }
}
