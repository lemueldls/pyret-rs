use super::{Decl, Expr};

#[derive(Debug)]
pub enum Stmt {
    Decl(Decl),
    Expr(Expr),
}
