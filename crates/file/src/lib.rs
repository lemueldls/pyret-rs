pub mod graph;

use std::fmt;

pub use miette;
use miette::{MietteError, MietteSpanContents, SourceCode, SourceSpan, SpanContents};

pub struct PyretFile {
    pub name: Box<str>,
    pub source: Box<str>,
}

impl PyretFile {
    #[must_use]
    pub const fn new(name: Box<str>, source: Box<str>) -> Self {
        Self { name, source }
    }
}

impl fmt::Debug for PyretFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PyretFile")
            .field("name", &self.name)
            .field("source", &"<redacted>");

        Ok(())
    }
}

impl SourceCode for PyretFile {
    fn read_span<'span>(
        &'span self,
        span: &SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn SpanContents<'span> + 'span>, MietteError> {
        let contents = self
            .source
            .read_span(span, context_lines_before, context_lines_after)?;

        Ok(Box::new(MietteSpanContents::new_named(
            self.name.to_string(),
            contents.data(),
            *contents.span(),
            contents.line(),
            contents.column(),
            contents.line_count(),
        )))
    }
}
