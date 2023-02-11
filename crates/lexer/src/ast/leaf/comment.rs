#[derive(Debug, PartialEq, Eq)]
pub enum CommentKind {
    Line,
    Block,
}

// #[leaf]
pub struct Comment {
    pub kind: CommentKind,
    pub text: Box<str>,
}
