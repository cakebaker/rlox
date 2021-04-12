use crate::expr::Expr;
use crate::expr::Literal;

#[derive(Debug)]
pub struct RuntimeError {}

pub struct Interpreter {
}

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
            _ => Err(RuntimeError{}),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;
    use crate::expr::Expr;
    use crate::expr::Literal;

    #[test]
    fn evaluate_literals() {
        let literals = vec![Literal::Bool(true), Literal::Bool(false), Literal::Nil, Literal::Number(1.0), Literal::String("str".to_string())];

        for literal in literals {
           let expr = Expr::Literal(literal.clone());
            if let Ok(result) = Interpreter::evaluate(expr) {
                assert_eq!(literal, result);
            } else {
                panic!("Interpreter::evaluate() returned unexpected Err");
            }
        }
    }
}
