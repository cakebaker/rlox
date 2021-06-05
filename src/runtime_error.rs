use crate::value::Value;
use std::error::Error;
use std::fmt;

type Line = usize;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidOperator,
    NumberExpectedAfterMinus(Line),
    Return(Value),
    UndefinedVariable(String),
    ValueNotCallable(Value),
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperator => write!(f, "Invalid operator"),
            Self::NumberExpectedAfterMinus(line) => {
                write!(f, "Number expected after '-' on line {}", line)
            }
            Self::Return(value) => write!(f, "{}", value),
            Self::UndefinedVariable(var) => write!(f, "Undefined variable: '{}'", var),
            Self::ValueNotCallable(value) => write!(f, "Value not callable: '{}'", value),
        }
    }
}
