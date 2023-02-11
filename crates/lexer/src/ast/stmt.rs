use crate::ast::{DeclarationStatement, ExpressionStatement, VariableDeclaration};

#[node(transform)]
pub enum Statement {
    Expression(ExpressionStatement),
    Declaration(DeclarationStatement),
}

impl Statement {
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
