#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Terminator {
    CRLF,
    Any(u8),
}

impl Terminator {
    pub fn is_crlf(&self) -> bool {
        match *self {
            Terminator::CRLF => true,
            Terminator::Any(_) => false,
        }
    }

    pub fn equals(&self, other: u8) -> bool {
        match *self {
            Terminator::CRLF => other == b'\r' || other == b'\n',
            Terminator::Any(b) => other == b,
        }
    }
}

impl Default for Terminator {
    fn default() -> Terminator {
        Terminator::CRLF
    }
}
