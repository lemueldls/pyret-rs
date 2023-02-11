use pyret_number::PyretNumber;

use crate::prelude::*;

/// <https://www.pyret.org/docs/latest/s_literals.html#(part._f~3anumber_literals)>
#[derive(Leaf, Debug, PartialEq)]
#[regex(r"~?[-+]?[[:digit:]]+(/[[:digit:]]+|(\.[[:digit:]]+)?([eE][-+]?[[:digit:]]+)?)?")]
pub struct NumericLiteral {
    span: (usize, usize),
    pub value: PyretNumber,
}

impl TokenParser for NumericLiteral {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        // if let Some(prefix) = value.get(..2) {
        //     let unary_operator = prefix.find('-').or_else(|| prefix.find('+'));

        //     if let Some(position) = unary_operator {
        //         if state.on_same_line() {
        //             return Err(ErrorKind::OperatorWhitespace {
        //                 position: state.next_position + position,
        //             });
        //         }
        //     }
        // };

        let value = match input.parse() {
            Ok(value) => value,
            Err(..) => {
                let division = input.find('/').unwrap();

                state.throw_late(PyretErrorKind::DivideByZero {
                    // denominator: (
                    //     state.next_position + input.find('/').unwrap() + 1,
                    //     // input.len(),
                    //     1,
                    // )
                    //     .into(),
                    denominator: (
                        state.next_position + division + 1,
                        input.len() - division - 1,
                    )
                        .into(),
                });

                PyretNumber::Rough(0_f64)
            }
        };

        Ok(Self {
            span: state.spanned(input.len()),
            value,
        })
    }
}
