use crate::value::Value;

#[derive(Debug)]
pub enum RuntimeError {
    InvalidOperator,
    InvalidType,
    Return(Value),
    UndefinedVariable(String),
    ValueNotCallable(Value),
}
