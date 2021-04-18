use crate::literal::Literal;
use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
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
    Variable(Token),
}
