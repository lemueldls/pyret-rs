use crate::{
    ast::{ExpressionStatement, IdentifierExpression, NumericLiteral, Statement},
    prelude::*,
};

/// <https://www.pyret.org/docs/latest/Expressions.html#(part._s~3adot-expr)>
#[common]
#[derive(Leaf)]
#[regex(r"\.")]
pub struct DotExpression {
    span: (usize, usize),
    object: Box<ExpressionStatement>,
    property: IdentifierExpression,
}

impl TokenParser for DotExpression {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.current_position;

        // Skip "."
        state.skip(1);

        let property = match state.try_lex::<IdentifierExpression>() {
            Ok(ident) => ident,
            Err(error) => {
                state.current_position = start_position + 1;

                return Err(match state.lex::<NumericLiteral>()? {
                    Some(number) => PyretErrorKind::InvalidNumber {
                        number: (start_position..number.end()).into(),
                    },
                    None => error,
                });
            }
        };

        let object = Box::new(match state.pop()? {
            Statement::Expression(expr) => expr,
            stmt => {
                return Err(PyretErrorKind::ExpectedObject {
                    left: stmt.serialize(),
                });
            }
        });

        Ok(Self {
            span: (object.start(), property.end()),
            object,
            property,
        })
    }
}
