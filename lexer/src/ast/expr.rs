use super::Stmt;

#[derive(Debug)]
pub enum Expr {
    Block(BlockExpr),
    Call(CallExpr),
    Binary(BinaryExpr),
    Ident(String),
    Lit(Lit),
    Paren(Box<Expr>),
}

#[derive(Debug)]
pub struct BlockExpr {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub struct CallExpr {
    pub callee: String,
    pub arguments: Vec<Expr>,
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub operator: String,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum Lit {
    Str(String),
    Num(Number),
    Bool(bool),
}

#[derive(Debug)]
pub enum Number {
    Exact(f64),
    Rough(f64),
}
