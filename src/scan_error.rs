use std::error::Error;
use std::fmt;

type Line = usize;

#[derive(Debug)]
pub enum ScanError {
    NumberEndsWithDot(Line),
    UnexpectedChar(char, Line),
    UnterminatedString(Line),
}

impl Error for ScanError {}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NumberEndsWithDot(line) => {
                write!(f, "Number ends with '.' on line {}", line)
            }
            Self::UnexpectedChar(c, line) => {
                write!(f, "Unexpected character '{}' on line {}", c, line)
            }
            Self::UnterminatedString(line) => {
                write!(f, "Unterminated string starting on line {}", line)
            }
        }
    }
}
