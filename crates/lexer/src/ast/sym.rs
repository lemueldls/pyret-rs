use crate::prelude::*;

#[common]
#[derive(Node, Eq)]
pub enum SymbolStatement {
    #[regex(r"as")]
    As(AsSymbol),
    #[regex(r"from")]
    From(FromSymbol),
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
