use crate::{
    ast::{BooleanLiteral, NumericLiteral, StringLiteral},
    prelude::*,
};

#[derive(Node, Debug, PartialEq)]
pub enum LiteralExpression {
    String(StringLiteral),
    Number(NumericLiteral),
    Boolean(BooleanLiteral),
}
