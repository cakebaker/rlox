use crate::token::Literal;
use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal(Literal),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
