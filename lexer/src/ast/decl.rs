use super::{Expr, Function};

#[derive(Debug)]
pub enum Decl {
    Fn(FnDecl),
    Var(VarDecl),
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: String,
    pub init: Expr,
    pub mutable: bool,
    pub recursive: bool,
    pub type_ann: Option<String>,
}

#[derive(Debug)]
pub struct FnDecl {
    pub name: String,
    pub function: Function,
}
