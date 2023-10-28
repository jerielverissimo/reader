use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum ReadError {
    EndOfInput,
    Unexpected(char),
    Missing(String),
    IoError(String),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ReadError::*;
        match self {
            EndOfInput => write!(f, "end of input"),
            Unexpected(ch) => write!(f, "unexpected character '{}'", ch),
            Missing(msg) => write!(f, "missing {}", msg),
            IoError(msg) => msg.fmt(f),
        }
    }
}
