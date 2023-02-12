use crate::{
    ast::{ExpressionStatement, IdentifierExpression, Statement, TypeAnnotation},
    prelude::*,
};

#[derive(Debug, PartialEq, Eq)]
pub enum VariableDeclarationKind {
    Let,
    Variable,
    RecursiveLet,
}

#[derive(Leaf, Debug, PartialEq)]
#[regex(r"=")]
pub struct VariableDeclaration {
    span: (usize, usize),
    pub kind: VariableDeclarationKind,
    pub ident: IdentifierExpression,
    pub type_annotation: Option<TypeAnnotation>,
    pub init: ExpressionStatement,
}

impl TokenParser for VariableDeclaration {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        let kind = VariableDeclarationKind::Let;

        let Statement::Expression(ExpressionStatement::Identifier(ident)) = state.pop()? else { todo!("ident pls") };

        let type_annotation = None;

        // Reset next position
        state.next_position = start_position;
        state.skip(input.len());

        let init = state.try_lex::<ExpressionStatement>()?;

        Ok(Self {
            span: (start_position, init.end()),
            kind,
            ident,
            type_annotation,
            init,
        })
    }
}
