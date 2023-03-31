mod module;
mod trove;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

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
