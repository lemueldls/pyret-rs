use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use proc_macro_error::ResultExt;
use quote::quote;
use syn::{parse::Parse, Ident};

pub fn parse<T: Parse>(input: proc_macro::TokenStream) -> T {
    syn::parse(input).unwrap_or_abort()
}

pub fn node_name(ident: &Ident) -> Box<str> {
    format!("[{}]", ident.to_string().to_case(Case::Title)).into_boxed_str()
}

pub fn empty_lex() -> TokenStream {
    quote! {
        fn lex_token(_state: &mut LexerState) -> PyretResult<::std::option::Option<Self>> {
            unimplemented!()
        }
    }
}
