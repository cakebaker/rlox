use crate::expr::Expr;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
}
