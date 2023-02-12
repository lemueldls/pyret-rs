use crate::{
    ast::{ExpressionStatement, Statement},
    prelude::*,
};

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divide,
}

/// <https://www.pyret.org/docs/latest/Expressions.html#(part._s~3abinop-expr)>
#[derive(Leaf, Debug, PartialEq)]
#[regex(r"[\-+*/]")]
pub struct BinaryOperatorExpression {
    span: (usize, usize),
    pub left: Box<ExpressionStatement>,
    pub operator: BinaryOperation,
    pub right: Box<ExpressionStatement>,
}

impl TokenParser for BinaryOperatorExpression {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        let no_whitespace = start_position > 1
            && !state.source[start_position - 1..start_position]
                .chars()
                .next()
                .unwrap()
                .is_ascii_whitespace();

        if no_whitespace {
            return Err(PyretErrorKind::OperatorWhitespace {
                operator: start_position,
            });
        }

        let operator = match &*input {
            "+" => BinaryOperation::Plus,
            "-" => BinaryOperation::Minus,
            "*" => BinaryOperation::Times,
            "/" => BinaryOperation::Divide,
            _ => unreachable!(),
        };

        state.skip(input.len());

        let right = state.try_lex::<ExpressionStatement>()?;

        let left = match state.pop()? {
            Statement::Expression(expr) => match expr {
                ExpressionStatement::BinaryOperator(ref binary_op) => {
                    if binary_op.operator == operator {
                        expr
                    } else {
                        todo!("error about groups levels")
                    }
                }
                _ => expr,
            },
            _ => todo!("expr pls"),
        };

        Ok(Self {
            span: (state.next_position, right.end()),
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}
