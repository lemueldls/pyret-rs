use pyret_number::{str::PyretNumberParseError, PyretNumber};

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
        let value = match input.parse() {
            Ok(value) => value,
            Err(error) => match error {
                PyretNumberParseError::DivideByZero => {
                    let divider = input.find('/').unwrap();

                    state.throw_late(PyretErrorKind::DivideByZero {
                        denominator: (state.next_position + divider + 1, input.len() - divider - 1)
                            .into(),
                    });

                    PyretNumber::Rough(0.0)
                }
                PyretNumberParseError::InvalidNumber => {
                    state.throw_late(PyretErrorKind::InvalidNumber {
                        number: (state.next_position, input.len()).into(),
                    });

                    PyretNumber::Rough(0.0)
                }
                PyretNumberParseError::ParseFloat(..) => {
                    state.throw_late(PyretErrorKind::RoughNumberOverflow {
                        number: (state.next_position, input.len()).into(),
                    });

                    PyretNumber::Rough(0.0)
                }
                _ => unreachable!(),
            },
        };

        Ok(Self {
            span: state.spanned(input.len()),
            value,
        })
    }
}
