use super::VariableDeclaration;
use crate::prelude::*;

#[derive(Node, Debug, PartialEq)]
pub enum DeclarationStatement {
    // Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}
