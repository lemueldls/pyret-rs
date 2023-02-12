use std::sync::Arc;

use proc_macro::TokenStream;
use proc_macro_error::{abort, ResultExt};
use quote::quote;
use syn::{ItemStruct, LitStr};

use crate::{regex, utils, REGULAR_EXPRESSIONS};

pub fn expand(input: &ItemStruct) -> TokenStream {
    let struct_ident = &input.ident;

    let node_name = utils::node_name(struct_ident);

    let variant = Arc::from_iter([regex::LexerItem {
        ident: Arc::from(struct_ident.to_string()),
        variant: Arc::from("parse"),
        transforms: Arc::new([]),
    }]);

    let exprs = input
        .attrs
        .iter()
        .filter_map(|attr| {
            let ident = attr
                .path
                .get_ident()
                .unwrap_or_else(|| abort!(attr.path, "invalid identifier"))
                .to_string();

            let hir = match ident.as_str() {
                "regex" => {
                    let regex: LitStr = attr.parse_args().unwrap_or_abort();

                    Some(
                        regex_syntax::Parser::new()
                            .parse(&regex.value())
                            .unwrap_or_else(|error| abort!(regex, error)),
                    )
                }
                // "transform" => {
                //     let transform: LeafTransform = attr.parse_args().unwrap_or_abort();

                //     None
                // }
                _ => None,
            };

            hir.map(|hir| (Arc::clone(&variant), hir))
        })
        .collect::<Vec<_>>();

    let lex = if exprs.is_empty() {
        utils::empty_lex()
    } else {
        regex::expand(&exprs, struct_ident.span())
    };

    let old_exprs = REGULAR_EXPRESSIONS
        .lock()
        .expect("Could not acquire regular expressions lock")
        .insert(Arc::from(struct_ident.to_string()), exprs);

    if old_exprs.is_some() {
        abort!(
            struct_ident,
            "Duplicate leaf struct identifier: {}.",
            struct_ident
        );
    };

    let expanded = quote! {
        impl TokenLexer for #struct_ident {
            #lex
        }

        impl Token for #struct_ident {
            const NODE_NAME: &'static str = #node_name;

            #[inline]
            fn leaf_name(&self) -> &str {
                Self::NODE_NAME
            }

            #[inline]
            fn start(&self) -> usize {
                self.span.0
            }

            #[inline]
            fn end(&self) -> usize {
                self.span.1
            }
        }
    };

    TokenStream::from(expanded)
}
