use crate::{
    ast::{DeclarationStatement, ExpressionStatement, VariableDeclaration},
    prelude::*,
};

#[derive(Node, Debug, PartialEq)]
#[transform(transform)]
pub enum Statement {
    Expression(ExpressionStatement),
    Declaration(DeclarationStatement),
}

impl Statement {
    /// # Errors
    ///
    /// Will return an [`PyretErrorKind`] if the statement is not valid.
    #[inline]
    pub fn transform(self, state: &mut LexerState) -> PyretResult<Self> {
        state.consume(self);

        let token = if let Some(var) = state.lex::<VariableDeclaration>()? {
            Self::Declaration(DeclarationStatement::Variable(var)).transform(state)?
        } else {
            state.pop()?
        };

        Ok(token)
    }
}
