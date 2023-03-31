use crate::{
    ast::{BooleanLiteral, NumericLiteral, StringLiteral},
    prelude::*,
};

#[common]
#[derive(Node)]
pub enum LiteralExpression {
    String(StringLiteral),
    Number(NumericLiteral),
    Boolean(BooleanLiteral),
}
