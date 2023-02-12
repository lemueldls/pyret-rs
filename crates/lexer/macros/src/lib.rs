mod leaf;
mod node;
mod regex;
mod utils;

use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use utils::parse;

static REGULAR_EXPRESSIONS: Lazy<Mutex<regex::RegexMap>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[proc_macro_error]
#[proc_macro_derive(Node, attributes(transform))]
pub fn node(input: TokenStream) -> TokenStream {
    node::expand(&parse(input))
}

#[proc_macro_error]
#[proc_macro_derive(Leaf, attributes(regex))]
pub fn leaf(input: TokenStream) -> TokenStream {
    leaf::expand(&parse(input))
}
