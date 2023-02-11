use super::ParenthesisExpression;
use crate::{
    ast::{
        ApplicationExpression, BinaryOperatorExpression, DotExpression, IdentifierExpression,
        LiteralExpression, Statement,
    },
    prelude::*,
};

#[node(transform)]
pub enum ExpressionStatement {
    Application(ApplicationExpression),
    // Block(BlockExpression),
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
            Self::Identifier(ident) if &state.source[ident.end()..=ident.end()] == "(" => {
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
            // if !state.source[expr.end()..state.current_position]
            //     .as_bytes()
            //     .iter()
            //     .any(|b| b == &b'\n')
            // {
            //     state.throw_late(PyretErrorKind::SameLineNextExpression {
            //         left: expr.serialize(),
            //         right: state.tokens.last().unwrap().serialize(),
            //     });
            // }

            expr
        } else {
            unreachable!()
        };

        Ok(token)
    }
}

// #[derive(Debug)]
// pub struct BlockExpression {
//     pub stmts: Vec<Statement>,
// }

// #[derive(Debug)]
// pub struct CallExpression {
//     pub callee: Box<str>,
//     pub arguments: Vec<ExpressionStatement>,
// }
