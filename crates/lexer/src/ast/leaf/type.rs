use std::collections::HashMap;

use crate::{ast::IdentifierExpression, prelude::*};

#[common]

pub enum IdentifierAnnotation {
    Name(IdentifierExpression),
    Dot(Vec<IdentifierExpression>),
}

/// <https://www.pyret.org/docs/latest/s_annotations.html>
#[common]

pub enum AnnotationType {
    /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3aname-ann)>
    NameAnnotation {
        name: IdentifierAnnotation,
        /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3aapp-ann)>
        parameters: Vec<IdentifierExpression>,
        /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3apred-ann)>
        predicate: Option<IdentifierExpression>,
    },
    /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3aarrow-ann)>
    ArrowAnnotation {
        generics: Option<Vec<IdentifierExpression>>,
        arguments: Vec<AnnotationType>,
        return_annotation: Box<AnnotationType>,
    },
    /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3atuple-ann)>
    TupleAnnotation(Vec<AnnotationType>),
    /// <https://www.pyret.org/docs/latest/s_annotations.html#(part._s~3arecord-ann)>
    RecordAnnotation(HashMap<Box<str>, AnnotationType>),
}

#[common]
#[derive(Leaf)]
#[regex("::")]
pub struct TypeAnnotation {
    span: (usize, usize),
    pub value: AnnotationType,
}

impl TokenParser for TypeAnnotation {
    #[inline]
    fn parse_token(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        state.skip(2);

        let start_position = state.next_position;

        let value = match state.lex::<IdentifierExpression>()? {
            Some(ident) => AnnotationType::NameAnnotation {
                name: IdentifierAnnotation::Name(ident),
                parameters: vec![],
                predicate: None,
            },
            None => {
                todo!()
            }
        };

        Ok(Self {
            span: (start_position, state.current_position),
            value,
        })
    }
}
