use crate::{
    ast::{
        DeclarationStatement, EqualSymbol, ExpressionStatement, IdentifierExpression,
        ImportStatement, LetDeclaration, LetDeclarationKind, SymbolStatement,
    },
    prelude::*,
};

#[derive(Node, Debug, PartialEq)]
#[transform(transform)]
pub enum Statement {
    Symbol(SymbolStatement),
    Declaration(DeclarationStatement),
    Import(ImportStatement),
    // Provide(ProvideStatement),
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

                if let Some(Self::Declaration(DeclarationStatement::Let(mut variable))) = state.lex()? {
                    variable.kind = LetDeclarationKind::RecursiveLet;

                    Self::Declaration(DeclarationStatement::Let(variable)).transform(state)?
                } else {
                    todo!("rec without let")
                }
            }
            Self::Symbol(SymbolStatement::Var(var)) => {
                state.current_position = var.end();

                if let Some(Self::Declaration(DeclarationStatement::Let(mut variable))) = state.lex()? {
                    variable.kind = LetDeclarationKind::Variable;

                    Self::Declaration(DeclarationStatement::Let(variable)).transform(state)?
                } else {
                    todo!("var without let")
                }
            }
            Self::Expression(ExpressionStatement::Identifier(ident)) if let Some(equal) = state.lex::<EqualSymbol>()? => {
                state.current_position = equal.end();

                Self::Declaration(DeclarationStatement::Let(LetDeclaration::new(
                    LetDeclarationKind::Let,
                    ident,
                    state,
                )?)).transform(state)?
            }
            _ => self,
        })
    }
}
