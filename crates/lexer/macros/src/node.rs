use std::sync::Arc;

use proc_macro::TokenStream;
use proc_macro_error::{abort, ResultExt};
use quote::quote;
use syn::{Fields, Ident, ItemEnum, Type};

use crate::{regex, utils, REGULAR_EXPRESSIONS};

pub fn expand(input: &ItemEnum) -> TokenStream {
    let enum_ident = &input.ident;
    let enum_name = Arc::<str>::from(enum_ident.to_string());

    let node_name = utils::node_name(enum_ident);

    let mut matches = Vec::new();
    let mut members = Vec::new();

    let mut regular_expressions = REGULAR_EXPRESSIONS
        .lock()
        .expect("Could not acquire regular expressions lock");

    let transforms = get_transforms(input);

    for variant in &input.variants {
        match variant.fields {
            Fields::Unnamed(ref fields) => {
                let variant_ident = &variant.ident;

                let leaf_ident = match fields
                    .unnamed
                    .first()
                    .unwrap_or_else(|| abort!(fields, "Expected an unnamed field"))
                    .ty
                {
                    Type::Path(ref type_path) => {
                        &type_path
                            .path
                            .segments
                            .last()
                            .unwrap_or_else(|| abort!(fields, "Expected a path"))
                            .ident
                    }
                    _ => abort!(variant, "Expected a path"),
                };

                if let Some(regexps) = regular_expressions.get(&Arc::from(leaf_ident.to_string())) {
                    let variant_name = Arc::from(variant_ident.to_string());

                    for (items, exprs) in regexps {
                        let items = items
                            .iter()
                            .cloned()
                            .chain([regex::LexerItem {
                                ident: Arc::clone(&enum_name),
                                variant: Arc::clone(&variant_name),
                                transforms: Arc::clone(&transforms),
                            }])
                            .collect();

                        matches.push((items, exprs.clone()));
                    }
                } else {
                    abort!(leaf_ident, "Field item `{}` is not initialized", leaf_ident);
                }

                members.push(quote!(#enum_ident::#variant_ident(token) => token));
            }
            _ => abort!(variant, "Expected an unnamed field"),
        }
    }

    let lex = if matches.is_empty() {
        utils::empty_lex()
    } else {
        regex::expand(&matches, enum_ident.span())
    };

    regular_expressions.insert(enum_name, matches);

    let expanded = quote! {
        impl TokenLexer for #enum_ident {
            #lex
        }

        impl Token for #enum_ident {
            const NODE_NAME: &'static str = #node_name;

            #[inline]
            fn leaf_name(&self) -> &str {
                match self {
                    #(#members.leaf_name(),)*
                }
            }

            #[inline]
            fn start(&self) -> usize {
                match self {
                    #(#members.start(),)*
                }
            }

            #[inline]
            fn end(&self) -> usize {
                match self {
                    #(#members.end(),)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_transforms(input: &ItemEnum) -> Arc<[Box<str>]> {
    input
        .attrs
        .iter()
        .filter_map(|attr| {
            let ident = attr
                .path
                .get_ident()
                .unwrap_or_else(|| abort!(attr.path, "invalid identifier"))
                .to_string();

            match ident.as_str() {
                "transform" => Some(
                    attr.parse_args::<Ident>()
                        .unwrap_or_abort()
                        .to_string()
                        .into_boxed_str(),
                ),
                _ => None,
            }
        })
        .collect()
}
