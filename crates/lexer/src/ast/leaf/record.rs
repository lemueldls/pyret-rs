use super::{IdentifierExpression, StringLiteral};
use crate::{
    ast::{
        AsSymbol, ExpressionStatement, FromSymbol, LiteralExpression, Statement, SymbolStatement,
    },
    prelude::*,
};

/// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3arecord-ann)>
#[common]
#[derive(Leaf)]
#[regex(r"\{")]
pub struct RecordAnnotation {
    span: (usize, usize),
    pub value: HashMap<IdentifierExpression, ExpressionStatement>,
}

impl TokenParser for RecordAnnotation {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.current_position;

        state.current_position = state.next_position + 6;

        let value = parse_import(Vec::new(), state)?;

        Ok(Self {
            span: (start_position, state.current_position),
            value,
        })
    }
}
