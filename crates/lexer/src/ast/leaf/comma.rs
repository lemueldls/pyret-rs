use crate::prelude::*;

#[derive(Leaf, Debug, PartialEq, Eq)]
#[regex(r",")]
pub struct CommaKeyword {
    span: (usize, usize),
}

impl TokenParser for CommaKeyword {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        Ok(Self {
            span: (state.next_position, state.next_position + 1),
        })
    }
}
