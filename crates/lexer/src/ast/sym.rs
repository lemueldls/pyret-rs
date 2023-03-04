use crate::prelude::*;

#[derive(Node, Debug, PartialEq, Eq)]
pub enum SymbolStatement {
    #[regex(r"#.*")]
    Comment(CommentSymbol),
    #[regex(r"=")]
    Equal(EqualSymbol),
    #[regex(r"end")]
    End(EndSymbol),
    #[regex(r"var")]
    Var(VarSymbol),
    #[regex(r"rec")]
    Rec(RecSymbol),
    #[regex(r",")]
    Comma(CommaSymbol),
    #[regex(r":")]
    Colon(ColonSymbol),
    #[regex(r"\)")]
    CloseParen(CloseParenSymbol),
}
