mod token;

use line_col::LineColLookup;
use miette::{Diagnostic, Report, SourceSpan};
pub use pyret_file::{graph::PyretGraph, miette, PyretFile};
use thiserror::Error;
pub use token::SerializedToken;

pub type PyretResult<T> = Result<T, PyretErrorKind>;

#[derive(Debug)]
pub struct PyretError {
    kind: PyretErrorKind,
    file_id: usize,
}

impl PyretError {
    #[must_use]
    pub const fn new(kind: PyretErrorKind, file_id: usize) -> Self {
        Self { kind, file_id }
    }

    pub fn into_report(self, files: &dyn PyretGraph) -> Report {
        let file = files.get(self.file_id);

        let name = if let Some(mut labels) = self.kind.labels() {
            if let Some(label) = labels.next() {
                let lookup = LineColLookup::new(&file.source);

                let (start_line, start_column) = lookup.get(label.offset());
                let (end_line, end_column) = lookup.get(label.offset() + label.len());

                format!(
                    "{}:{start_line}:{start_column}-{end_line}:{end_column}",
                    file.name
                )
                .into_boxed_str()
            } else {
                file.name.clone()
            }
        } else {
            file.name.clone()
        };

        let source_code = PyretFile::new(name, file.source.clone());

        Report::new(self.kind).with_source_code(source_code)
    }
}

#[non_exhaustive]
#[derive(Error, Diagnostic, Debug)]
pub enum PyretErrorKind {
    #[error("Pyret didn't understand your program")]
    #[diagnostic(help(
        "you may need to add or remove some text to fix your program\nis there something there that shouldn't be?"
    ))]
    DidNotUnderstand {
        #[label("look carefully around here")]
        position: usize,
    },

    #[error("Pyret attempted to divide by zero")]
    DivideByZero {
        #[label("the denominator is zero")]
        denominator: SourceSpan,
    },

    #[error("Pyret didn't understand the very end of your program")]
    EarlyEnd {
        #[label]
        position: usize,
    },

    #[error("Pyret found an empty block")]
    EmptyBlock {
        #[label]
        ident: SourceSpan,
    },

    #[error("Pyret failed to evaluate the object lookup")]
    ExpectedObject {
        #[label]
        left: SerializedToken,
    },

    #[error("Pyret found evaluating this function application expression errored")]
    InvalidFunctionApplication {
        #[label("the left side was not a function value")]
        span: SourceSpan,
    },

    #[error("Pyret thinks your program has an invalid number")]
    InvalidNumber {
        #[label("number literals in Pyret require at least one digit before the decimal point")]
        number: SourceSpan,
    },

    #[error("Pyret found an invalid string")]
    InvalidString {
        #[label]
        string: SourceSpan,
    },

    #[error("Pyret must have whitespace separating operators it from its operands")]
    OperatorWhitespace {
        #[label]
        operator: SourceSpan,
    },

    #[error("Pyret found a Roughnum overflow")]
    RoughNumberOverflow {
        #[label]
        number: SourceSpan,
    },

    #[error("Pyret thinks you're missing something before here")]
    SomethingBefore {
        #[label]
        position: usize,
    },

    #[error("Pyret expects each expression within a block to have its own line")]
    SameLineNextExpression {
        #[label]
        left: SerializedToken,
        #[label]
        right: SerializedToken,
    },

    #[error("Pyret found the identifier {ident} is unbound")]
    UnboundIdentifier {
        ident: Box<str>,
        #[label("it is used but not previously defined")]
        span: SourceSpan,
    },

    #[error("Pyret found an unexpected {}", found.name)]
    Unexpected {
        expected: Box<str>,
        #[label]
        found: SerializedToken,
    },

    #[error("Pyret thinks your program has an incomplete string literal")]
    UnfinishedString {
        #[label]
        from: SerializedToken,
        multiline: bool,
    },

    #[error("Pyret thinks your program is missing an opening block comment")]
    UnmatchedClosingComment {
        #[label]
        position: SourceSpan,
    },

    #[error("Pyret thinks your program is missing a closing block comment")]
    UnmatchedOpeningComment {
        #[label]
        position: SourceSpan,
    },

    #[error("Pyret didn't expect your program to end as soon as it did")]
    UnclosedParenthesis {
        #[label]
        position: SourceSpan,
    },

    #[error("{0}")]
    RaiseRuntime(Box<str>),
}
