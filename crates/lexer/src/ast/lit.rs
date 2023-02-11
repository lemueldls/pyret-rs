use crate::ast::{BooleanLiteral, NumericLiteral, StringLiteral};

#[node]
pub enum LiteralExpression {
    String(StringLiteral),
    Number(NumericLiteral),
    Boolean(BooleanLiteral),
}
