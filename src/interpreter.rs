use crate::clock::Clock;
use crate::environment::Environment;
use crate::expr::Expr;
use crate::literal::Literal;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;

#[derive(Debug)]
pub struct RuntimeError {}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        env.define("clock".to_string(), Value::Function(Box::new(Clock::new())));

        Self { environment: env }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(&statement);
        }
    }

    fn execute(&mut self, statement: &Stmt) {
        match statement {
            Stmt::Block(statements) => {
                self.environment = Environment::new_with_parent(self.environment.clone());

                for statement in statements {
                    self.execute(statement);
                }

                if let Some(parent) = self.environment.take_parent() {
                    self.environment = parent;
                }
            }
            Stmt::Expr(expr) => {
                self.evaluate(&*expr);
            }
            Stmt::If(condition, then_branch, else_branch) => {
                if let Ok(literal) = self.evaluate(&*condition) {
                    if literal.is_truthy() {
                        self.execute(&*then_branch);
                    } else if *else_branch != None {
                        self.execute(&*else_branch.as_ref().unwrap());
                    }
                }
            }
            Stmt::Function(name, params, body) => {} // TODO implement
            Stmt::Print(expr) => {
                if let Ok(result) = self.evaluate(&*expr) {
                    println!("{}", result);
                }
            }
            Stmt::Var(name, None) => self.environment.define(name.lexeme.clone(), Value::Nil),
            Stmt::Var(name, Some(initializer)) => {
                if let Ok(value) = self.evaluate(&*initializer) {
                    self.environment.define(name.lexeme.clone(), value);
                }
            }
            Stmt::While(condition, body) => {
                self.execute_while(condition, body);
            }
        }
    }

    fn execute_while(&mut self, condition: &Expr, body: &Stmt) -> Result<(), RuntimeError> {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body);
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Assign { name, value } => {
                let v = self.evaluate(&*value)?;
                self.environment.assign(name.lexeme.clone(), v.clone());
                Ok(v)
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee = self.evaluate(callee)?;

                match callee {
                    Value::Function(callable) => {
                        let mut args = Vec::with_capacity(arguments.len());

                        for argument in arguments {
                            args.push(self.evaluate(argument)?);
                        }

                        Ok(callable.call(self, args))
                    }
                    _ => Err(RuntimeError {}),
                }
            }
            Expr::Grouping { expression: expr } => self.evaluate(&*expr),
            Expr::Literal(Literal::Bool(bool)) => Ok(Value::Bool(*bool)),
            Expr::Literal(Literal::Nil) => Ok(Value::Nil),
            Expr::Literal(Literal::Number(number)) => Ok(Value::Number(*number)),
            Expr::Literal(Literal::String(string)) => Ok(Value::String(string.clone())),
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_result = self.evaluate(&*left)?;

                // short-circuit, if possible
                if operator.token_type == TokenType::Or {
                    if left_result.is_truthy() {
                        return Ok(left_result);
                    }
                } else if !left_result.is_truthy() {
                    return Ok(left_result);
                }

                Ok(self.evaluate(&*right)?)
            }
            Expr::Unary { operator, right } => self.evaluate_unary(operator, &*right),
            Expr::Variable(name) => self.environment.get(name.lexeme.clone()),
        }
    }

    fn evaluate_unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let result = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => match result {
                Value::Number(number) => Ok(Value::Number(-number)),
                _ => Err(RuntimeError {}),
            },
            TokenType::Bang => Ok(Value::Bool(!&result.is_truthy())),
            _ => Err(RuntimeError {}),
        }
    }

    #[allow(clippy::float_cmp)]
    fn evaluate_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        match (self.evaluate(left)?, self.evaluate(right)?) {
            (Value::Number(l), Value::Number(r)) => match operator.token_type {
                TokenType::Plus => Ok(Value::Number(l + r)),
                TokenType::Minus => Ok(Value::Number(l - r)),
                TokenType::Star => Ok(Value::Number(l * r)),
                TokenType::Slash => Ok(Value::Number(l / r)),
                TokenType::Greater => Ok(Value::Bool(l > r)),
                TokenType::GreaterEqual => Ok(Value::Bool(l >= r)),
                TokenType::Less => Ok(Value::Bool(l < r)),
                TokenType::LessEqual => Ok(Value::Bool(l <= r)),
                TokenType::EqualEqual => Ok(Value::Bool(l == r)),
                TokenType::BangEqual => Ok(Value::Bool(l != r)),
                _ => Err(RuntimeError {}),
            },
            (Value::String(l), Value::String(r)) => match operator.token_type {
                TokenType::Plus => Ok(Value::String(format!("{}{}", l, r))),
                TokenType::EqualEqual => Ok(Value::Bool(l == r)),
                TokenType::BangEqual => Ok(Value::Bool(l != r)),
                _ => Err(RuntimeError {}),
            },
            (Value::Bool(l), Value::Bool(r)) => match operator.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(l == r)),
                TokenType::BangEqual => Ok(Value::Bool(l != r)),
                _ => Err(RuntimeError {}),
            },
            (Value::Nil, Value::Nil) => match operator.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(true)),
                TokenType::BangEqual => Ok(Value::Bool(false)),
                _ => Err(RuntimeError {}),
            },
            _ => match operator.token_type {
                TokenType::EqualEqual => Ok(Value::Bool(false)),
                TokenType::BangEqual => Ok(Value::Bool(true)),
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
    use crate::stmt::Stmt;
    use crate::token::Token;
    use crate::token_type::TokenType;
    use crate::value::Value;

    #[test]
    fn evaluate_literals() {
        let literals_and_expectations = vec![
            (Literal::Bool(true), Value::Bool(true)),
            (Literal::Bool(false), Value::Bool(false)),
            (Literal::Nil, Value::Nil),
            (Literal::Number(1.0), Value::Number(1.0)),
            (
                Literal::String("str".to_string()),
                Value::String("str".to_string()),
            ),
        ];

        for (literal, expected) in literals_and_expectations {
            let expr = Expr::Literal(literal);
            if let Ok(result) = Interpreter::new().evaluate(&expr) {
                assert_eq!(expected, result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }

    #[test]
    fn evaluate_grouping() {
        let expr = Expr::Grouping {
            expression: Box::new(Expr::Literal(Literal::Bool(true))),
        };

        if let Ok(result) = Interpreter::new().evaluate(&expr) {
            assert_eq!(Value::Bool(true), result);
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

        if let Ok(result) = Interpreter::new().evaluate(&expr) {
            assert_eq!(Value::Number(-1.0), result);
        } else {
            panic!("Interpreter::evaluate() returned unexpected Err");
        }
    }

    #[test]
    fn evaluate_logical_not() {
        let literals = vec![
            (Literal::Nil, Value::Bool(true)),
            (Literal::Bool(false), Value::Bool(true)),
            (Literal::Bool(true), Value::Bool(false)),
            (Literal::Number(1.0), Value::Bool(false)),
            (Literal::String("str".to_string()), Value::Bool(false)),
        ];

        for (literal, expected) in literals {
            let expr = Expr::Unary {
                operator: token(TokenType::Bang),
                right: Box::new(Expr::Literal(literal)),
            };

            if let Ok(result) = Interpreter::new().evaluate(&expr) {
                assert_eq!(expected, result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }

    #[test]
    fn evaluate_logical_and() {
        let left_right_and_expectations = vec![
            (false, false, false),
            (false, true, false),
            (true, false, false),
            (true, true, true),
        ];

        for (left, right, expected) in left_right_and_expectations {
            let expr = Expr::Logical {
                left: Box::new(Expr::Literal(Literal::Bool(left))),
                operator: token(TokenType::And),
                right: Box::new(Expr::Literal(Literal::Bool(right))),
            };

            if let Ok(result) = Interpreter::new().evaluate(&expr) {
                assert_eq!(Value::Bool(expected), result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }

    #[test]
    fn evaluate_logical_or() {
        let left_right_and_expectations = vec![
            (false, false, false),
            (false, true, true),
            (true, false, true),
            (true, true, true),
        ];

        for (left, right, expected) in left_right_and_expectations {
            let expr = Expr::Logical {
                left: Box::new(Expr::Literal(Literal::Bool(left))),
                operator: token(TokenType::Or),
                right: Box::new(Expr::Literal(Literal::Bool(right))),
            };

            if let Ok(result) = Interpreter::new().evaluate(&expr) {
                assert_eq!(Value::Bool(expected), result);
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
            (TokenType::Minus, Value::Number(1.0)),
            (TokenType::Plus, Value::Number(5.0)),
            (TokenType::Star, Value::Number(6.0)),
            (TokenType::Slash, Value::Number(1.5)),
        ];

        for (operator, expected) in operators_and_expectations {
            let expr = Expr::Binary {
                left: Box::new(Expr::Literal(LEFT)),
                operator: token(operator),
                right: Box::new(Expr::Literal(RIGHT)),
            };

            if let Ok(result) = Interpreter::new().evaluate(&expr) {
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

        if let Ok(result) = Interpreter::new().evaluate(&expr) {
            assert_eq!(Value::String("aabb".to_string()), result);
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

            if let Ok(result) = Interpreter::new().evaluate(&expr) {
                assert_eq!(Value::Bool(expected), result);
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

                if let Ok(result) = Interpreter::new().evaluate(&expr) {
                    assert_eq!(Value::Bool(expected), result);
                } else {
                    panic!("Interpreter::evaluate() returned unexpected Err");
                }
            }
        }
    }

    #[test]
    fn evaluate_variable() {
        let mut interpreter = Interpreter::new();
        let stmt = Stmt::Var(
            token(TokenType::String("test".to_string())),
            Some(Expr::Literal(Literal::String("value".to_string()))),
        );
        interpreter.interpret(vec![stmt]);

        let expr = Expr::Variable(token(TokenType::String("test".to_string())));

        if let Ok(result) = interpreter.evaluate(&expr) {
            assert_eq!(Value::String("value".to_string()), result);
        } else {
            panic!("Interpreter::evaluate() returned unexpected Err");
        }
    }

    #[test]
    fn evaluate_undefined_variable() {
        let expr = Expr::Variable(token(TokenType::String("test".to_string())));

        if let Err(e) = Interpreter::new().evaluate(&expr) {
            assert!(true);
        } else {
            panic!("Interpreter::evaluate() didn't return an Err");
        }
    }

    #[test]
    fn evaluate_variable_assignment() {
        let mut interpreter = Interpreter::new();
        let def_stmt = Stmt::Var(
            token(TokenType::String("test".to_string())),
            Some(Expr::Literal(Literal::String("value".to_string()))),
        );
        let assign_stmt = Stmt::Expr(Expr::Assign {
            name: token(TokenType::String("test".to_string())),
            value: Box::new(Expr::Literal(Literal::String("updated".to_string()))),
        });
        interpreter.interpret(vec![def_stmt, assign_stmt]);

        let expr = Expr::Variable(token(TokenType::String("test".to_string())));

        if let Ok(result) = interpreter.evaluate(&expr) {
            assert_eq!(Value::String("updated".to_string()), result);
        } else {
            panic!("Interpreter::evaluate() returned unexpected Err");
        }
    }

    fn token(token_type: TokenType) -> Token {
        match token_type {
            TokenType::String(ref s) => Token::new(token_type.clone(), s.to_string(), 1),
            _ => Token::new(token_type, "".to_string(), 1),
        }
    }
}
