use crate::prelude::*;

#[derive(Leaf, Debug, PartialEq, Eq)]
#[regex(r"end")]
pub struct EndKeyword {
    span: (usize, usize),
}

impl TokenParser for EndKeyword {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        Ok(Self {
            span: (state.next_position, state.next_position + 3),
        })
    }
}
