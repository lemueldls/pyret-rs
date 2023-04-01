use crate::{
    ast::{CheckDeclaration, LetDeclaration},
    prelude::*,
};

#[common]
#[derive(Node)]
pub enum DeclarationStatement {
    Check(CheckDeclaration),
    // Function(FunctionDeclaration),
    Let(LetDeclaration),
}
