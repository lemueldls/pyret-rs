mod module;
mod trove;

use std::{iter, rc::Rc};

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error, ResultExt};
use quote::{quote, ToTokens};
use syn::{
    parse, parse::Parse, punctuated::Punctuated, Ident, ImplItem, ItemFn, ItemImpl, Pat, Path,
    Token, Type, TypeReference,
};

#[proc_macro_error]
#[proc_macro_attribute]
pub fn module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    module::expand(item)
}

#[proc_macro_error]
#[proc_macro]
pub fn trove(item: TokenStream) -> TokenStream {
    trove::expand(item)
}
