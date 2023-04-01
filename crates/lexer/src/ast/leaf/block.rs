use crate::{
    ast::{Statement, SymbolStatement},
    prelude::*,
};

/// <https://www.pyret.org/docs/latest/Blocks.html>
#[common]
#[derive(Leaf)]
#[regex(r"block:")]
pub struct BlockExpression {
    span: (usize, usize),
    pub body: Vec<Statement>,
}

impl TokenParser for BlockExpression {
    #[inline]
    fn parse_token(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        state.current_position = start_position + 6;

        let mut body = Vec::new();

        let mut end_position = None;

        while let Some(stmt) = state.lex::<Statement>()? {
            state.current_position = stmt.end();

            if let Statement::Symbol(SymbolStatement::End(end_symbol)) = &stmt {
                end_position = Some(end_symbol.end());

                break;
            }

            body.push(stmt);
        }

        if let Some(end_position) = end_position {
            Ok(Self {
                span: (start_position, end_position),
                body,
            })
        } else {
            todo!("Handle unclosed block expressions")
        }
    }
}
