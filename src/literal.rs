use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(string) => write!(f, "{}", string),
            Self::Number(number) => write!(f, "{}", number),
            Self::Bool(bool) => write!(f, "{}", bool),
            Self::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Literal;

    #[test]
    fn display() {
        assert_eq!("test", format!("{}", Literal::String("test".to_string())));
        assert_eq!("1.23", format!("{}", Literal::Number(1.23)));
        assert_eq!("true", format!("{}", Literal::Bool(true)));
        assert_eq!("false", format!("{}", Literal::Bool(false)));
        assert_eq!("nil", format!("{}", Literal::Nil));
    }
}
