use super::TypeAnnotation;
use crate::prelude::*;

/// <https://www.pyret.org/docs/latest/s_literals.html#(part._.Names)>
#[derive(Leaf, Debug, PartialEq)]
#[regex(r"[_[:alpha:]][[:word:]]*(-+[[:word:]]+)*")]
pub struct IdentifierExpression {
    span: (usize, usize),
    pub name: Box<str>,
    pub annotation: Option<Box<TypeAnnotation>>,
}

impl TokenParser for IdentifierExpression {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        state.current_position = start_position + input.len();

        let annotation = state.lex::<TypeAnnotation>()?.map(Box::new);

        Ok(Self {
            span: (start_position, state.current_position),
            name: input,
            annotation,
        })
    }
}
