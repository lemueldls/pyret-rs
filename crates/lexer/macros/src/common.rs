use proc_macro::TokenStream;
use quote::quote;

pub fn expand(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let expanded = quote! {
        #[derive(Debug)]
        #[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
        #input
    };

    TokenStream::from(expanded)
}
