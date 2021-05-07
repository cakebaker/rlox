use std::fmt;

use crate::lox_callable::LoxCallable;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Function(Box<dyn LoxCallable>),
    Nil,
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(bool) => write!(f, "{}", bool),
            Self::Function(function) => write!(f, "{:?}", function), // XXX what should the output be?
            Self::Nil => write!(f, "nil"),
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}

impl Value {
    pub const fn is_truthy(&self) -> bool {
        !matches!(self, Self::Nil | Self::Bool(false))
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn display() {
        assert_eq!("test", format!("{}", Value::String("test".to_string())));
        assert_eq!("1.23", format!("{}", Value::Number(1.23)));
        assert_eq!("true", format!("{}", Value::Bool(true)));
        assert_eq!("false", format!("{}", Value::Bool(false)));
        assert_eq!("nil", format!("{}", Value::Nil));
    }

    #[test]
    fn is_truthy() {
        assert_eq!(false, Value::Nil.is_truthy());
        assert_eq!(false, Value::Bool(false).is_truthy());
        assert_eq!(true, Value::Bool(true).is_truthy());
        assert_eq!(true, Value::Number(0.0).is_truthy());
        assert_eq!(true, Value::String("".to_string()).is_truthy());
    }
}
