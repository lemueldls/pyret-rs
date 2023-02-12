use std::{collections::HashMap, sync::Arc};

use regex_syntax::hir::Hir;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerItem {
    pub ident: Arc<str>,
    pub variant: Arc<str>,
    pub transforms: Arc<[Box<str>]>,
}

pub type RegexMap = HashMap<Arc<str>, Vec<(Arc<[LexerItem]>, Hir)>>;
