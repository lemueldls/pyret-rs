use crate::{
    ast::{ExpressionStatement, IdentifierExpression, Statement},
    prelude::*,
};

#[derive(Debug, PartialEq, Eq)]
pub enum LetDeclarationKind {
    Let,
    Variable,
    RecursiveLet,
}

#[derive(Leaf, Debug, PartialEq)]
pub struct LetDeclaration {
    span: (usize, usize),
    pub kind: LetDeclarationKind,
    pub ident: IdentifierExpression,
    pub init: ExpressionStatement,
}

impl LetDeclaration {
    pub fn new(
        kind: LetDeclarationKind,
        ident: IdentifierExpression,
        state: &mut LexerState,
    ) -> PyretResult<Self> {
        let start_position = state.next_position;

        let kind = LetDeclarationKind::Let;

        let init = state.try_lex::<ExpressionStatement>()?;

        Ok(Self {
            span: (start_position, init.end()),
            kind,
            ident,
            init,
        })
    }
}
