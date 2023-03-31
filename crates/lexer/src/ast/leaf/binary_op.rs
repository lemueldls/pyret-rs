use crate::{
    ast::{ExpressionStatement, Statement},
    prelude::*,
};

#[common]
#[derive(Eq)]
pub enum BinaryOperation {
    Plus,
    Minus,
    Times,
    Divide,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Is,
    IsRoughly,
}

impl BinaryOperation {
    #[must_use]
    pub const fn priority(&self) -> u8 {
        match self {
            Self::Plus | Self::Minus => 0,
            Self::Times | Self::Divide => 1,
            Self::LessThan
            | Self::LessThanOrEqual
            | Self::GreaterThan
            | Self::GreaterThanOrEqual
            | Self::Equal
            | Self::NotEqual => 2,
            Self::And | Self::Or => 3,
            Self::Is | Self::IsRoughly => 4,
        }
    }

    #[must_use]
    pub const fn is_testing(&self) -> bool {
        matches!(self, Self::Is | Self::IsRoughly)
    }

    #[must_use]
    pub fn check_grouping(&self, other: &Self) -> bool {
        self == other || self.is_testing() || other.is_testing()
    }
}

/// <https://www.pyret.org/docs/latest/Expressions.html#(part._s~3abinop-expr)>
#[common]
#[derive(Leaf)]
#[regex(r"[+\-*/]|<=?|>=?|==|<>|and|or|is(-roughly)?")]
pub struct BinaryOperatorExpression {
    span: (usize, usize),
    pub left: Box<ExpressionStatement>,
    pub operator: BinaryOperation,
    pub right: Box<ExpressionStatement>,
}

impl BinaryOperatorExpression {
    #[must_use]
    pub fn new(
        left: ExpressionStatement,
        operator: BinaryOperation,
        right: ExpressionStatement,
    ) -> Self {
        match (left, right) {
            (left, ExpressionStatement::BinaryOperator(mut binary_op))
                if operator.priority() < binary_op.operator.priority() =>
            {
                binary_op.left = Box::new(ExpressionStatement::BinaryOperator(Self::new(
                    left,
                    operator,
                    *binary_op.left,
                )));

                binary_op
            }

            (left, right) => Self {
                span: (left.start(), right.end()),
                left: Box::new(left),
                operator,
                right: Box::new(right),
            },
        }
    }
}

impl TokenParser for BinaryOperatorExpression {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.next_position;

        let no_whitespace = state.current_position == start_position;

        let length = input.len();

        if no_whitespace {
            return Err(PyretErrorKind::OperatorWhitespace {
                operator: (start_position, length).into(),
            });
        }

        let operator = match &*input {
            "+" => BinaryOperation::Plus,
            "-" => BinaryOperation::Minus,
            "*" => BinaryOperation::Times,
            "/" => BinaryOperation::Divide,
            "<" => BinaryOperation::LessThan,
            "<=" => BinaryOperation::LessThanOrEqual,
            ">" => BinaryOperation::GreaterThan,
            ">=" => BinaryOperation::GreaterThanOrEqual,
            "==" => BinaryOperation::Equal,
            "<>" => BinaryOperation::NotEqual,
            "and" => BinaryOperation::And,
            "or" => BinaryOperation::Or,
            "is" => BinaryOperation::Is,
            "is-roughly" => BinaryOperation::IsRoughly,
            op => unreachable!("{{{op}}}"),
        };

        state.skip(length);

        let right = match state.try_lex::<ExpressionStatement>()? {
            ExpressionStatement::BinaryOperator(binary_op)
                if !operator.check_grouping(&binary_op.operator) =>
            {
                todo!("error about groups levels")
            }
            expr => expr,
        };

        let left = match state.pop()? {
            Statement::Expression(expr) => match &expr {
                ExpressionStatement::BinaryOperator(binary_op) => {
                    if operator.check_grouping(&binary_op.operator) {
                        expr
                    } else {
                        todo!("error about groups levels")
                    }
                }
                _ => expr,
            },
            _ => todo!("expr pls"),
        };

        Ok(Self::new(left, operator, right))
    }
}
