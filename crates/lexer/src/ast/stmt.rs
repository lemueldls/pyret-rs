use super::TypeAnnotation;
use crate::{
    ast::{
        DeclarationStatement, EqualSymbol, ExpressionStatement, IdentifierExpression,
        ImportStatement, LetDeclaration, LetDeclarationKind, ProvideStatement, SymbolStatement,
    },
    prelude::*,
};

#[common]
#[derive(Node)]
#[transform(transform)]
pub enum Statement {
    Symbol(SymbolStatement),
    Declaration(DeclarationStatement),
    Import(ImportStatement),
    Provide(ProvideStatement),
    Expression(ExpressionStatement),
}

impl Statement {
    /// # Errors
    ///
    /// Will return an [`PyretErrorKind`] if the statement is not valid.
    #[inline]
    pub fn transform(self, state: &mut LexerState) -> PyretResult<Self> {
        // match self {
        //     Self::Symbol(SymbolStatement::Rec(_)) => {
        //         let ident = state.try_lex::<Identifier>()?;

        //     }
        // }

        Ok(match self {
            Self::Symbol(SymbolStatement::Rec(rec)) => {
                state.current_position = rec.end();

                if let Some(Self::Declaration(DeclarationStatement::Let(mut variable))) =
                    state.lex()?
                {
                    variable.kind = LetDeclarationKind::RecursiveLet;

                    Self::Declaration(DeclarationStatement::Let(variable)).transform(state)?
                } else {
                    todo!("rec without let")
                }
            }
            Self::Symbol(SymbolStatement::Var(var)) => {
                state.current_position = var.end();

                if let Some(Self::Declaration(DeclarationStatement::Let(mut variable))) =
                    state.lex()?
                {
                    variable.kind = LetDeclarationKind::Variable;

                    Self::Declaration(DeclarationStatement::Let(variable)).transform(state)?
                } else {
                    todo!("var without let")
                }
            }
            Self::Expression(ExpressionStatement::Identifier(ident)) => {
                if let Some(equal) = state.lex::<EqualSymbol>()? {
                    state.current_position = equal.end();

                    let init = state.try_lex::<ExpressionStatement>()?;

                    Self::Declaration(DeclarationStatement::Let(LetDeclaration::new(
                        LetDeclarationKind::Let,
                        ident,
                        None,
                        Some(init),
                        state,
                    )?))
                    .transform(state)?
                } else if let Some(ann) = state.lex::<TypeAnnotation>()? {
                    state.current_position = ann.end();

                    let init = if let Some(equal) = state.lex::<EqualSymbol>()? {
                        state.current_position = equal.end();

                        Some(state.try_lex::<ExpressionStatement>()?)
                    } else {
                        None
                    };

                    Self::Declaration(DeclarationStatement::Let(LetDeclaration::new(
                        LetDeclarationKind::Let,
                        ident,
                        Some(ann),
                        init,
                        state,
                    )?))
                } else {
                    Self::Expression(ExpressionStatement::Identifier(ident))
                }
            }
            _ => self,
        })
    }
}
