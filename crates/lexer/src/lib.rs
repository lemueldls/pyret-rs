#![feature(if_let_guard)]

mod comments;
mod macros;
mod state;
mod token;

pub mod ast;
mod prelude;

#[macro_use]
extern crate pyret_lexer_macros;

use comments::remove_comments;
use prelude::*;
pub use pyret_error as error;
pub use token::Token;

/// # Errors
///
/// Will return a vector of [`PyretErrorKind`]s if there are any lexing errors.
#[inline]
pub fn lex(source: &str) -> Result<Vec<ast::Statement>, Vec<PyretErrorKind>> {
    let source = remove_comments(source).map_err(|error| vec![error])?;

    let mut state = LexerState::new(&source);

    let lex_state = lex_state(&mut state);

    let mut errors = state.late_errors;

    match lex_state {
        Ok(..) => {
            if errors.is_empty() {
                Ok(state.tokens)
            } else {
                Err(errors)
            }
        }
        Err(error) => {
            errors.push(error);

            Err(errors)
        }
    }
}

fn lex_state(state: &mut LexerState) -> PyretResult<()> {
    while let Some(stmt) = state.lex()? {
        state.consume(stmt);
    }

    if state.source[state.next_position..]
        .chars()
        .all(|c| c.is_ascii_whitespace())
    {
        Ok(())
    } else {
        Err(PyretErrorKind::DidNotUnderstand {
            position: state.next_position,
        })
    }
}
