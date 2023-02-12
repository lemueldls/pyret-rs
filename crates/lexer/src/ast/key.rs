use crate::{
    ast::{CommaKeyword, EndKeyword},
    prelude::*,
};

#[derive(Node, Debug, PartialEq, Eq)]
pub enum KeywordStatement {
    End(EndKeyword),
    Comma(CommaKeyword),
}
