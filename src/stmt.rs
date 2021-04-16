use crate::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}
