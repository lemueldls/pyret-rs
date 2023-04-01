use crate::{
    ast::{ExpressionStatement, IdentifierExpression, TypeAnnotation},
    prelude::*,
};

#[common]
#[derive(PartialEq, Eq)]
pub enum LetDeclarationKind {
    Let,
    Variable,
    RecursiveLet,
}

#[common]
#[derive(Leaf)]
pub struct LetDeclaration {
    span: (usize, usize),
    pub kind: LetDeclarationKind,
    pub ident: IdentifierExpression,
    pub annotation: Option<TypeAnnotation>,
    pub init: Option<ExpressionStatement>,
}

impl LetDeclaration {
    pub fn new(
        kind: LetDeclarationKind,
        ident: IdentifierExpression,
        annotation: Option<TypeAnnotation>,
        init: Option<ExpressionStatement>,
        state: &mut LexerState,
    ) -> PyretResult<Self> {
        let start_position = state.next_position;

        let end = init.as_ref().map_or_else(
            || annotation.as_ref().map_or_else(|| ident.end(), Token::end),
            Token::end,
        );

        Ok(Self {
            span: (start_position, end),
            kind,
            ident,
            annotation,
            init,
        })
    }
}
