use std::collections::HashMap;

use super::IdentifierExpression;
use crate::{
    ast::{ExpressionStatement, Statement, SymbolStatement},
    prelude::*,
};

#[common]
#[derive(Eq)]
pub enum ProvideValues {
    Wildcard,
    Identifiers(HashMap<Box<str>, IdentifierExpression>),
}

#[common]
#[derive(Eq)]
pub enum ProvideTypes {
    Wildcard,
}

#[common]
#[derive(Eq)]
pub enum Provide {
    Values(ProvideValues),
    Types(ProvideTypes),
}

/// <https://www.pyret.org/docs/latest/Provide_Statements.html>
#[common]
#[derive(Leaf, Eq)]
#[regex(r"provide(-types)?")]
#[regex(r"(?s)provide(-types)?\s+\*")]
pub struct ProvideStatement {
    span: (usize, usize),
    pub value: Provide,
}

impl TokenParser for ProvideStatement {
    #[inline]
    fn parse(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.current_position;

        let mut end_position = None;

        let value = if input.starts_with("provide-types") {
            end_position = Some(state.next_position + input.len());

            Provide::Types(ProvideTypes::Wildcard)
        } else if input.len() > 7 {
            end_position = Some(state.next_position + input.len());

            Provide::Values(ProvideValues::Wildcard)
        } else {
            state.current_position = start_position + 8;

            let mut idents = HashMap::new();

            while let Some(stmt) = state.lex::<Statement>()? {
                state.current_position = stmt.end();

                if let Statement::Symbol(SymbolStatement::Comma(..)) = stmt {
                    continue;
                }

                if let Statement::Symbol(SymbolStatement::End(..)) = &stmt {
                    end_position = Some(state.current_position);

                    break;
                }

                if let Statement::Expression(ExpressionStatement::Identifier(ident)) = stmt {
                    idents.insert(ident.name.clone(), ident);
                } else {
                    todo!("provide statement")
                }
            }

            Provide::Values(ProvideValues::Identifiers(idents))
        };

        if let Some(end_position) = end_position {
            Ok(Self {
                span: (start_position, end_position),
                value,
            })
        } else {
            todo!("Handle unclosed block expressions")
        }
    }
}
