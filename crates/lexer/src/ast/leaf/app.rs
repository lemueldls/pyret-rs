use crate::{
    ast::{ExpressionStatement, IdentifierExpression, KeywordStatement, Statement},
    prelude::*,
};

/// <https://www.pyret.org/docs/latest/Expressions.html#(part._s~3aapp-expr)>
#[derive(Leaf, Debug, PartialEq)]
pub struct ApplicationExpression {
    span: (usize, usize),
    pub ident: IdentifierExpression,
    pub args: Vec<ExpressionStatement>,
}

impl ApplicationExpression {
    /// # Errors
    ///
    /// Will return an [`PyretErrorKind`] if the expression is not valid.
    pub fn new(ident: IdentifierExpression, state: &mut LexerState) -> PyretResult<Self> {
        let mut args = Vec::new();

        state.current_position = ident.end() + 1;

        while let Some(expr) = state.lex::<ExpressionStatement>()? {
            args.push(expr);

            if let Some(Statement::Keyword(KeywordStatement::Comma(_))) = state.lex()? {
                continue;
            }
        }

        Ok(Self {
            span: (ident.start(), state.next_position + 1),
            ident,
            args,
        })
    }
}
