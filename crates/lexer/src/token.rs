use std::{fmt::Debug, ops::Range};

use crate::{
    error::{PyretResult, SerializedToken},
    LexerState,
};

pub trait Token: Debug + Sized {
    const NODE_NAME: &'static str;

    fn leaf_name(&self) -> &str;

    fn start(&self) -> usize;

    fn end(&self) -> usize;

    #[inline]
    fn span(&self) -> Range<usize> {
        self.start()..self.end()
    }

    fn serialize(&self) -> SerializedToken {
        SerializedToken {
            name: Box::from(self.leaf_name()),
            span: self.start()..self.end(),
        }
    }
}

pub trait TokenLexer: Token {
    /// # Errors
    ///
    /// Will return an [`Error`] if there was an error parsing the token.
    ///
    /// [`Error`]: crate::Error
    fn lex_token(state: &mut LexerState) -> PyretResult<::std::option::Option<Self>>;
}

pub trait TokenParser: Token {
    /// # Errors
    ///
    /// Will return an [`Error`] if there was an error parsing the token.
    ///
    /// [`Error`]: crate::Error
    fn parse_token(input: Box<str>, state: &mut LexerState) -> PyretResult<Self>;
}
