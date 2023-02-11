use crate::{
    ast::{ExpressionStatement, IdentifierExpression, NumericLiteral, Statement},
    prelude::*,
};

/// <https://www.pyret.org/docs/latest/Expressions.html#(part._s~3adot-expr)>
#[derive(Leaf, Debug, PartialEq)]
#[regex(r"\.")]
pub struct DotExpression {
    span: (usize, usize),
    object: Box<ExpressionStatement>,
    property: IdentifierExpression,
}

impl TokenParser for DotExpression {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        // Skip "."
        state.skip(1);

        let property = match state.try_lex::<IdentifierExpression>() {
            Ok(ident) => ident,
            Err(error) => {
                return match state.lex::<NumericLiteral>()? {
                    Some(number) => Err(PyretErrorKind::InvalidNumber {
                        number: number.start() - 1..number.end(),
                    }),
                    None => Err(error),
                }
            }
        };

        let end = property.end();

        let object = Box::new(match state.pop()? {
            Statement::Expression(expr) => expr,
            stmt => {
                return Err(PyretErrorKind::ExpectedObject {
                    left: stmt.serialize(),
                })
            }
        });

        Ok(Self {
            span: (object.start(), end),
            object,
            property,
        })
    }
}
