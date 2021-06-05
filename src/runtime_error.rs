use crate::value::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidOperator,
    InvalidType,
    Return(Value),
    UndefinedVariable(String),
    ValueNotCallable(Value),
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperator => write!(f, "Invalid operator"),
            Self::InvalidType => write!(f, "Invalid type"),
            Self::Return(value) => write!(f, "{}", value),
            Self::UndefinedVariable(var) => write!(f, "Undefined variable: '{}'", var),
            Self::ValueNotCallable(value) => write!(f, "Value not callable: '{}'", value),
        }
    }
}
