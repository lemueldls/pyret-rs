use crate::prelude::*;

/// <https://www.pyret.org/docs/latest/s_literals.html#(part._.Boolean_.Literals)>
#[derive(Leaf, Debug, PartialEq, Eq)]
#[regex(r"true|false")]
pub struct BooleanLiteral {
    span: (usize, usize),
    pub value: bool,
}

impl TokenParser for BooleanLiteral {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        Ok(Self {
            span: state.spanned(input.len()),
            value: &*input == "true",
        })
    }
}