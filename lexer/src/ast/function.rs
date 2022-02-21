use super::Stmt;

#[derive(Debug)]
pub struct Function {
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub type_ann: Option<String>,
}
