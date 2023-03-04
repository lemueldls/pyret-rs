#![feature(if_let_guard)]

mod macros;
mod state;
mod token;

pub mod ast;
mod prelude;

#[macro_use]
extern crate pyret_lexer_macros;

use prelude::*;
pub use pyret_error as error;
pub use token::Token;

/// # Errors
///
/// Will return a vector of [`PyretErrorKind`]s if there are any lexing errors.
#[inline]
pub fn lex(source: &str) -> Result<Vec<ast::Statement>, Vec<PyretErrorKind>> {
    // let mut comment_parser = CommentParser::new(self.input.clone());

    // let start = std::time::Instant::now();

    // let comments = comment_parser.parse();

    // println!("[COMMENTS]: {}\u{3bc}s", start.elapsed().as_micros());

    // match comments {
    //     Ok(input) => {
    let mut state = LexerState::new(source);

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
    //     }
    //     Err(error) => Err(vec![error]),
    // }
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
