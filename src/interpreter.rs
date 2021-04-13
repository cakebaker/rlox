use crate::expr::Expr;
use crate::expr::Literal;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct RuntimeError {}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(expr: Expr) {
        if let Ok(value) = Self::evaluate(expr) {
            println!("The value is: {:?}", value);
        } else {
            // TODO
            println!("Runtime error!");
        }
    }

    fn evaluate(expr: Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal(literal) => Ok(literal),
            Expr::Grouping { expression: expr } => Self::evaluate(*expr),
            Expr::Unary { operator, right } => Self::evaluate_unary(&operator, *right),
            _ => Err(RuntimeError {}),
        }
    }

    fn evaluate_unary(operator: &Token, right: Expr) -> Result<Literal, RuntimeError> {
        let result = Self::evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => match result {
                Literal::Number(number) => Ok(Literal::Number(-number)),
                _ => Err(RuntimeError {}),
            },
            TokenType::Bang => {
                // false and nil are treated as falsey, everything else is truthy
                match result {
                    Literal::Bool(bool) => Ok(Literal::Bool(!bool)),
                    Literal::Nil => Ok(Literal::Bool(true)),
                    _ => Ok(Literal::Bool(false)),
                }
            }
            _ => Err(RuntimeError {}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use crate::expr::Expr;
    use crate::expr::Literal;
    use crate::token::Token;
    use crate::token_type::TokenType;

    #[test]
    fn evaluate_literals() {
        let literals = vec![
            Literal::Bool(true),
            Literal::Bool(false),
            Literal::Nil,
            Literal::Number(1.0),
            Literal::String("str".to_string()),
        ];

        for literal in literals {
            let expr = Expr::Literal(literal.clone());
            if let Ok(result) = Interpreter::evaluate(expr) {
                assert_eq!(literal, result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }

    #[test]
    fn evaluate_grouping() {
        let literal = Literal::Bool(true);
        let expr = Expr::Grouping {
            expression: Box::new(Expr::Literal(literal.clone())),
        };

        if let Ok(result) = Interpreter::evaluate(expr) {
            assert_eq!(literal, result);
        } else {
            panic!("Interpreter::evaluate() returned unexpected Err");
        }
    }

    #[test]
    fn evaluate_negation() {
        let expr = Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_string(), 1),
            right: Box::new(Expr::Literal(Literal::Number(1.0))),
        };

        if let Ok(result) = Interpreter::evaluate(expr) {
            assert_eq!(Literal::Number(-1.0), result);
        } else {
            panic!("Interpreter::evaluate() returned unexpected Err");
        }
    }

    #[test]
    fn evaluate_logical_not() {
        let literals = vec![
            (Literal::Nil, Literal::Bool(true)),
            (Literal::Bool(false), Literal::Bool(true)),
            (Literal::Bool(true), Literal::Bool(false)),
            (Literal::Number(1.0), Literal::Bool(false)),
            (Literal::String("str".to_string()), Literal::Bool(false)),
        ];

        for (literal, expected) in literals {
            let expr = Expr::Unary {
                operator: Token::new(TokenType::Bang, "!".to_string(), 1),
                right: Box::new(Expr::Literal(literal)),
            };

            if let Ok(result) = Interpreter::evaluate(expr) {
                assert_eq!(expected, result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }
}
