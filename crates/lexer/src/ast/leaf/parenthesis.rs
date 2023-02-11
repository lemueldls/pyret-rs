use crate::{ast::ExpressionStatement, prelude::*};

/// <https://www.pyret.org/docs/latest/Expressions.html#(elem._(bnf-prod._(.Pyret._paren-expr)))>
#[derive(Leaf, Debug, PartialEq)]
#[regex(r"\(")]
pub struct ParenthesisExpression {
    span: (usize, usize),
    pub expr: Box<ExpressionStatement>,
}

impl TokenParser for ParenthesisExpression {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        state.current_position = start_position + 1;

        let expr = Box::new(state.try_lex::<ExpressionStatement>()?);

        let end = if let Some(closing) = state.source[state.current_position..].find(')') {
            closing + 1
        } else {
            return Err(PyretErrorKind::UnclosedParenthesis {
                position: start_position,
            });
        };

        Ok(Self {
            span: (start_position, state.current_position + end),
            expr,
        })
    }
}
