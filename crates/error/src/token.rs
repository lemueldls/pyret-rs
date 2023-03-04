use std::ops::Range;

use pyret_file::miette::SourceSpan;

#[derive(Debug, Clone)]
pub struct SerializedToken {
    pub name: Box<str>,
    pub span: Range<usize>,
}

impl From<SerializedToken> for SourceSpan {
    fn from(token: SerializedToken) -> Self {
        (token.span.start..token.span.end).into()
    }
}
