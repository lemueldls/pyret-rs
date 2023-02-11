use std::ops::Range;

#[derive(Debug, Clone)]
pub struct SerializedToken {
    pub name: Box<str>,
    pub span: Range<usize>,
}
