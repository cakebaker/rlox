use crate::expr::Expr;
use crate::literal::Literal;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct RuntimeError {}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(statements: Vec<Stmt>) {
        for statement in statements {
            match statement {
                Stmt::Print(expr) => {
                    if let Ok(result) = Self::evaluate(expr) {
                        println!("{}", result);
                    }
                }
                Stmt::Expr(expr) => {
                    Self::evaluate(expr);
                }
                Stmt::Var(name, initializer) => {} // TODO implement
            }
        }
    }

    fn evaluate(expr: Expr) -> Result<Literal, RuntimeError> {
        match expr {
            Expr::Literal(literal) => Ok(literal),
            Expr::Grouping { expression: expr } => Self::evaluate(*expr),
            Expr::Unary { operator, right } => Self::evaluate_unary(&operator, *right),
            Expr::Binary {
                left,
                operator,
                right,
            } => Self::evaluate_binary(*left, &operator, *right),
            Expr::Variable(name) => Err(RuntimeError {}), // TODO implement
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

    fn evaluate_binary(left: Expr, operator: &Token, right: Expr) -> Result<Literal, RuntimeError> {
        match (Self::evaluate(left)?, Self::evaluate(right)?) {
            (Literal::Number(l), Literal::Number(r)) => match operator.token_type {
                TokenType::Plus => Ok(Literal::Number(l + r)),
                TokenType::Minus => Ok(Literal::Number(l - r)),
                TokenType::Star => Ok(Literal::Number(l * r)),
                TokenType::Slash => Ok(Literal::Number(l / r)),
                TokenType::Greater => Ok(Literal::Bool(l > r)),
                TokenType::GreaterEqual => Ok(Literal::Bool(l >= r)),
                TokenType::Less => Ok(Literal::Bool(l < r)),
                TokenType::LessEqual => Ok(Literal::Bool(l <= r)),
                TokenType::EqualEqual => Ok(Literal::Bool(l == r)),
                TokenType::BangEqual => Ok(Literal::Bool(l != r)),
                _ => Err(RuntimeError {}),
            },
            (Literal::String(l), Literal::String(r)) => match operator.token_type {
                TokenType::Plus => Ok(Literal::String(format!("{}{}", l, r))),
                TokenType::EqualEqual => Ok(Literal::Bool(l == r)),
                TokenType::BangEqual => Ok(Literal::Bool(l != r)),
                _ => Err(RuntimeError {}),
            },
            (Literal::Bool(l), Literal::Bool(r)) => match operator.token_type {
                TokenType::EqualEqual => Ok(Literal::Bool(l == r)),
                TokenType::BangEqual => Ok(Literal::Bool(l != r)),
                _ => Err(RuntimeError {}),
            },
            (Literal::Nil, Literal::Nil) => match operator.token_type {
                TokenType::EqualEqual => Ok(Literal::Bool(true)),
                TokenType::BangEqual => Ok(Literal::Bool(false)),
                _ => Err(RuntimeError {}),
            },
            _ => match operator.token_type {
                TokenType::EqualEqual => Ok(Literal::Bool(false)),
                TokenType::BangEqual => Ok(Literal::Bool(true)),
                _ => Err(RuntimeError {}),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use crate::expr::Expr;
    use crate::literal::Literal;
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
            operator: token(TokenType::Minus),
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
                operator: token(TokenType::Bang),
                right: Box::new(Expr::Literal(literal)),
            };

            if let Ok(result) = Interpreter::evaluate(expr) {
                assert_eq!(expected, result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }

    #[test]
    fn evaluate_arithmetic_operators() {
        const LEFT: Literal = Literal::Number(3.0);
        const RIGHT: Literal = Literal::Number(2.0);

        let operators_and_expectations = vec![
            (TokenType::Minus, Literal::Number(1.0)),
            (TokenType::Plus, Literal::Number(5.0)),
            (TokenType::Star, Literal::Number(6.0)),
            (TokenType::Slash, Literal::Number(1.5)),
        ];

        for (operator, expected) in operators_and_expectations {
            let expr = Expr::Binary {
                left: Box::new(Expr::Literal(LEFT)),
                operator: token(operator),
                right: Box::new(Expr::Literal(RIGHT)),
            };

            if let Ok(result) = Interpreter::evaluate(expr) {
                assert_eq!(expected, result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }

    #[test]
    fn evaluate_addition_of_strings() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Literal(Literal::String("aa".to_string()))),
            operator: token(TokenType::Plus),
            right: Box::new(Expr::Literal(Literal::String("bb".to_string()))),
        };

        if let Ok(result) = Interpreter::evaluate(expr) {
            assert_eq!(Literal::String("aabb".to_string()), result);
        } else {
            panic!("Interpreter::evaluate() returned unexpected Err");
        }
    }

    #[test]
    fn evaluate_comparison_operators() {
        const THREE: Literal = Literal::Number(3.0);
        const TWO: Literal = Literal::Number(2.0);

        let setup = vec![
            (TWO, TokenType::Greater, THREE, false),
            (THREE, TokenType::Greater, TWO, true),
            (TWO, TokenType::Greater, TWO, false),
            (TWO, TokenType::GreaterEqual, THREE, false),
            (THREE, TokenType::GreaterEqual, TWO, true),
            (TWO, TokenType::GreaterEqual, TWO, true),
            (TWO, TokenType::Less, THREE, true),
            (THREE, TokenType::Less, TWO, false),
            (TWO, TokenType::Less, TWO, false),
            (TWO, TokenType::LessEqual, THREE, true),
            (THREE, TokenType::LessEqual, TWO, false),
            (TWO, TokenType::LessEqual, TWO, true),
        ];

        for (left, operator, right, expected) in setup {
            let expr = Expr::Binary {
                left: Box::new(Expr::Literal(left)),
                operator: token(operator),
                right: Box::new(Expr::Literal(right)),
            };

            if let Ok(result) = Interpreter::evaluate(expr) {
                assert_eq!(Literal::Bool(expected), result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }

    #[test]
    fn evaluate_equality_operators() {
        const NIL: Literal = Literal::Nil;
        const ZERO: Literal = Literal::Number(0.0);
        const THREE: Literal = Literal::Number(3.0);
        const TRUE: Literal = Literal::Bool(true);
        const FALSE: Literal = Literal::Bool(false);
        let empty: Literal = Literal::String("".to_string());
        let bb: Literal = Literal::String("bb".to_string());

        let setup = vec![
            (NIL, NIL, true),
            (ZERO, ZERO, true),
            (ZERO, THREE, false),
            (TRUE, TRUE, true),
            (TRUE, FALSE, false),
            (empty.clone(), empty.clone(), true),
            (empty.clone(), bb, false),
            (NIL, ZERO, false),
            (NIL, FALSE, false),
            (NIL, empty.clone(), false),
        ];

        for (left, right, expected) in setup {
            let operators = vec![
                (token(TokenType::EqualEqual), expected),
                (token(TokenType::BangEqual), !expected),
            ];
            for (operator, expected) in operators {
                let expr = Expr::Binary {
                    left: Box::new(Expr::Literal(left.clone())),
                    operator: operator,
                    right: Box::new(Expr::Literal(right.clone())),
                };

                if let Ok(result) = Interpreter::evaluate(expr) {
                    assert_eq!(Literal::Bool(expected), result);
                } else {
                    panic!("Interpreter::evaluate() returned unexpected Err");
                }
            }
        }
    }

    fn token(token_type: TokenType) -> Token {
        Token::new(token_type, "".to_string(), 1)
    }
}
