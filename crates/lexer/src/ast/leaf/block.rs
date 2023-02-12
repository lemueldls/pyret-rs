use crate::{
    ast::{KeywordStatement, Statement},
    prelude::*,
};

/// <https://www.pyret.org/docs/latest/Blocks.html>
#[derive(Leaf, Debug, PartialEq)]
#[regex(r"block:")]
pub struct BlockExpression {
    span: (usize, usize),
    pub body: Vec<Statement>,
}

impl TokenParser for BlockExpression {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        state.current_position = start_position + 6;

        let mut body = Vec::new();

        while let Some(stmt) = state.lex::<Statement>()? {
            if matches!(stmt, Statement::Keyword(KeywordStatement::End(_))) {
                break;
            }

            body.push(stmt);
        }

        Ok(Self {
            span: (start_position, state.current_position),
            body,
        })
    }
}
