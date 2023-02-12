use super::ParenthesisExpression;
use crate::{
    ast::{
        ApplicationExpression, BinaryOperatorExpression, BlockExpression, DotExpression,
        IdentifierExpression, LiteralExpression, Statement,
    },
    prelude::*,
};

#[derive(Node, Debug, PartialEq)]
#[transform(transform)]
pub enum ExpressionStatement {
    Application(ApplicationExpression),
    Block(BlockExpression),
    Literal(LiteralExpression),
    Identifier(IdentifierExpression),
    BinaryOperator(BinaryOperatorExpression),
    Dot(DotExpression),
    Parenthesis(ParenthesisExpression),
}

impl ExpressionStatement {
    /// # Errors
    ///
    /// Will return an [`Error`] if the expression is not valid.
    #[inline]
    pub fn transform(self, state: &mut LexerState) -> PyretResult<Self> {
        match self {
            Self::Identifier(ident)
                if {
                    let end = ident.end();

                    &state.source[end..=end] == "("
                } =>
            {
                let application = ApplicationExpression::new(ident, state)?;

                return Ok(Self::Application(application));
            }

            _ => {
                state.consume(Statement::Expression(self));
            }
        }

        let token = if let Some(binary_op) = state.lex::<BinaryOperatorExpression>()? {
            Self::BinaryOperator(binary_op).transform(state)?
        } else if let Some(dot) = state.lex::<DotExpression>()? {
            Self::Dot(dot).transform(state)?.transform(state)?
        } else if let Statement::Expression(expr) = state.pop()? {
            expr
        } else {
            unreachable!()
        };

        Ok(token)
    }
}
