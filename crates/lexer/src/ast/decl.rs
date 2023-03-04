use crate::{
    ast::{CheckDeclaration, LetDeclaration},
    prelude::*,
};

#[derive(Node, Debug, PartialEq)]
pub enum DeclarationStatement {
    Check(CheckDeclaration),
    // Function(FunctionDeclaration),
    Let(LetDeclaration),
}
