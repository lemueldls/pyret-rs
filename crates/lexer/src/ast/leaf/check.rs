use super::StringLiteral;
use crate::{
    ast::{ColonSymbol, Statement, SymbolStatement},
    prelude::*,
};

/// <https://www.pyret.org/docs/latest/testing.html>
#[common]
#[derive(Leaf)]
#[regex(r"check")]
pub struct CheckDeclaration {
    span: (usize, usize),
    pub label: Option<Box<str>>,
    pub body: Vec<Statement>,
}

impl TokenParser for CheckDeclaration {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        state.current_position = start_position + 5;

        let label = state.lex::<StringLiteral>()?.map(|lit| {
            state.current_position = lit.end();

            lit.value
        });
        state.current_position = state.try_lex::<ColonSymbol>()?.end();

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
                label,
                body,
            })
        } else {
            todo!("Handle unclosed block expressions")
        }
    }
}
