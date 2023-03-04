use crate::{
    ast::{ExpressionStatement, IdentifierExpression, Statement, SymbolStatement},
    prelude::*,
};

/// <https://www.pyret.org/docs/latest/Expressions.html#(part._s~3aapp-expr)>
#[derive(Leaf, Debug, PartialEq)]
pub struct ApplicationExpression {
    span: (usize, usize),
    pub ident: IdentifierExpression,
    pub args: Vec<ExpressionStatement>,
}

impl ApplicationExpression {
    /// # Errors
    ///
    /// Will return an [`PyretErrorKind`] if the expression is not valid.
    pub fn new(ident: IdentifierExpression, state: &mut LexerState) -> PyretResult<Self> {
        let mut args = Vec::new();

        state.current_position = ident.end() + 1;

        while let Some(stmt) = state.lex::<Statement>()? {
            state.current_position = stmt.end();

            if let Statement::Symbol(SymbolStatement::Comma(..)) = stmt {
                continue;
            }
            if let Statement::Symbol(SymbolStatement::CloseParen(..)) = stmt {
                break;
            }

            if let Statement::Expression(expr) = stmt {
                args.push(expr);
            } else {
                todo!("Handle non-expression statements in application expressions");
            }
        }

        Ok(Self {
            span: (ident.start(), state.current_position),
            ident,
            args,
        })
    }
}
