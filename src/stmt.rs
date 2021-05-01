use crate::expr::Expr;
use crate::token::Token;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    Function(Token, Vec<Token>, Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
}
