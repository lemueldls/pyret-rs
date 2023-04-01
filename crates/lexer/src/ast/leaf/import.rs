use super::{IdentifierExpression, StringLiteral};
use crate::{
    ast::{ExpressionStatement, LiteralExpression, Statement, SymbolStatement},
    prelude::*,
};

#[common]
pub enum Import {
    As {
        source: ImportSource,
        name: IdentifierExpression,
    },
    From {
        names: Box<[IdentifierExpression]>,
        source: ImportSource,
    },
}

#[common]
pub enum ImportSource {
    Special {
        name: IdentifierExpression,
        values: Box<[StringLiteral]>,
    },
    Name {
        value: IdentifierExpression,
    },
    String {
        value: StringLiteral,
    },
}

/// <https://www.pyret.org/docs/latest/Import_Statements.html>
#[common]
#[derive(Leaf)]
#[regex(r"import")]
pub struct ImportStatement {
    span: (usize, usize),
    pub value: Import,
}

impl TokenParser for ImportStatement {
    #[inline]
    fn parse(_input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
        let start_position = state.current_position;

        state.current_position = state.next_position + 6;

        let value = parse_import(Vec::new(), state)?;

        Ok(Self {
            span: (start_position, state.current_position),
            value,
        })
    }
}

fn parse_import(mut sources: Vec<ImportSource>, state: &mut LexerState) -> PyretResult<Import> {
    // dbg!(&state.source[state.current_position..]);

    let stmt = state.try_lex::<Statement>()?;

    state.current_position = stmt.end();

    if let Statement::Symbol(SymbolStatement::Comma(..)) = stmt {
        return parse_import(sources, state);
    }

    if let Statement::Symbol(SymbolStatement::As(..)) = stmt {
        let name = state.try_lex()?;

        return Ok(Import::As {
            source: sources.pop().unwrap(),
            name,
        });
    }
    if let Statement::Symbol(SymbolStatement::From(..)) = stmt {
        let names = sources
            .into_iter()
            .map(|source| match source {
                ImportSource::Name { value } => value,
                _ => todo!("expected name"),
            })
            .collect::<Vec<_>>();

        let source = parse_source(state.try_lex()?)?;

        return Ok(Import::From {
            names: Box::from_iter(names),
            source,
        });
    }

    sources.push(parse_source(stmt)?);

    parse_import(sources, state)
}

fn parse_source(stmt: Statement) -> PyretResult<ImportSource> {
    Ok(match stmt {
        Statement::Expression(ExpressionStatement::Application(app)) => {
            let values = app
                .args
                .into_iter()
                .map(|arg| match arg {
                    ExpressionStatement::Literal(LiteralExpression::String(value)) => Ok(value),
                    _ => todo!("expected string literal"),
                })
                .collect::<PyretResult<Vec<_>>>()?;

            ImportSource::Special {
                name: app.ident,
                values: Box::from_iter(values),
            }
        }
        Statement::Expression(ExpressionStatement::Identifier(ident)) => {
            ImportSource::Name { value: ident }
        }
        Statement::Expression(ExpressionStatement::Literal(LiteralExpression::String(value))) => {
            ImportSource::String { value }
        }
        _ => todo!("expected import source"),
    })
}
