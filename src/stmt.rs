use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}
