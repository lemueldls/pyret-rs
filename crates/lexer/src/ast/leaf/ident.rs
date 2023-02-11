use crate::prelude::*;

/// <https://www.pyret.org/docs/latest/s_literals.html#(part._.Names)>
#[derive(Leaf, Debug, PartialEq, Eq, Hash)]
#[regex(r"[_[:alpha:]][[:word:]]*(-+[[:word:]]+)*")]
pub struct IdentifierExpression {
    span: (usize, usize),
    pub name: Box<str>,
}

impl TokenParser for IdentifierExpression {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        Ok(Self {
            span: state.spanned(input.len()),
            name: input,
        })
    }
}
