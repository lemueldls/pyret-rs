use crate::{
    ast::{CloseParenSymbol, ExpressionStatement},
    prelude::*,
};

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

        state.current_position = expr.end();

        let end = if let Some(closing) = state.lex::<CloseParenSymbol>()? {
            closing.end()
        } else {
            return Err(PyretErrorKind::UnclosedParenthesis {
                position: (start_position, start_position + 1).into(),
            });
        };

        Ok(Self {
            span: (start_position, end),
            expr,
        })
    }
}
