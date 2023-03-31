use crate::prelude::*;

/// <https://www.pyret.org/docs/latest/s_literals.html#(part._.Names)>
#[common]
#[derive(Leaf, Eq)]
#[regex(r"[_[:alpha:]][[:word:]]*(-+[[:word:]]+)*")]
pub struct IdentifierExpression {
    span: (usize, usize),
    pub name: Box<str>,
}

impl TokenParser for IdentifierExpression {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        state.current_position = start_position + input.len();

        Ok(Self {
            span: (start_position, state.current_position),
            name: input,
        })
    }
}
