use std::any::TypeId;

use crate::{ast, prelude::*};

#[derive(Debug)]
pub struct LexerState<'input> {
    pub source: &'input str,
    pub current_position: usize,
    pub next_position: usize,
    pub tokens: Vec<ast::Statement>,
    pub late_errors: Vec<PyretErrorKind>,
}

impl<'input> LexerState<'input> {
    #[must_use]
    pub const fn new(source: &'input str) -> Self {
        Self {
            source,
            current_position: 0,
            next_position: 0,
            tokens: Vec::new(),
            late_errors: Vec::new(),
        }
    }

    pub fn consume(&mut self, stmt: ast::Statement) {
        self.current_position = stmt.end();

        self.tokens.push(stmt);
    }

    /// Lexes a token, skipping whitespace.
    /// Returns `None` if there is no token to lex.
    ///
    /// # Errors
    ///
    /// Will return an [`PyretErrorKind`] if there was an error parsing the
    /// token.
    pub fn lex<T: TokenLexer + 'static>(&mut self) -> PyretResult<Option<T>> {
        if let Some(trimmed_start) =
            self.source[self.current_position..].find(|c: char| !c.is_ascii_whitespace())
        {
            self.next_position = self.current_position + trimmed_start;

            T::lex(self)
        } else {
            self.next_position = self.current_position;

            if TypeId::of::<T>() != TypeId::of::<ast::CommentSymbol>() {
                if let Some(comment) = self.lex::<ast::CommentSymbol>()? {
                    self.current_position = comment.end();

                    return self.lex();
                }
            }

            Ok(None)
        }
    }

    /// Lexes a token, skipping whitespace.
    /// Returns an error if there is no token to lex.
    ///
    /// # Errors
    ///
    /// Will return an [`PyretErrorKind`] if there was an error parsing the
    /// token, or if the token was not of the expected type.
    pub fn try_lex<T: TokenLexer + 'static>(&mut self) -> PyretResult<T> {
        if let Some(token) = self.lex()? {
            Ok(token)
        } else {
            let error = if let Some(token) = self.lex::<ast::Statement>()? {
                PyretErrorKind::Unexpected {
                    expected: Box::from(T::NODE_NAME),
                    found: token.serialize(),
                }
            } else {
                let position = self.current_position;

                if self.source[self.next_position..].is_empty() {
                    PyretErrorKind::EarlyEnd { position }
                } else {
                    PyretErrorKind::DidNotUnderstand { position }
                }
            };

            Err(error)
        }
    }

    pub fn throw_late(&mut self, error: PyretErrorKind) {
        self.late_errors.push(error);
    }

    #[must_use]
    pub const fn spanned(&self, position: usize) -> (usize, usize) {
        (self.next_position, self.next_position + position)
    }

    pub fn skip(&mut self, amount: usize) {
        self.current_position = self.next_position + amount;
    }

    /// Removes and returns the previous token.
    ///
    /// # Errors
    ///
    /// Will return an [`PyretErrorKind`] if there was no token to pop.
    pub fn pop(&mut self) -> PyretResult<ast::Statement> {
        match self.tokens.pop() {
            Some(token) => {
                self.next_position = token.start();

                Ok(token)
            }
            None => Err(PyretErrorKind::SomethingBefore {
                position: self.next_position,
            }),
        }
    }
}
