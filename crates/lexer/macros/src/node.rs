use std::sync::Arc;

use proc_macro::TokenStream;
use proc_macro_error::{abort, ResultExt};
use quote::quote;
use syn::{Fields, Ident, ItemEnum, LitStr, Type};

use crate::{leaf::create_leaf, regex, utils, REGULAR_EXPRESSIONS};

pub fn expand(input: &ItemEnum) -> TokenStream {
    let enum_ident = &input.ident;
    let enum_name = Arc::<str>::from(enum_ident.to_string());

    let node_name = utils::node_name(enum_ident);

    let mut matches = Vec::new();
    let mut members = Vec::new();
    let mut leafs = Vec::new();

    let transforms = get_transforms(input);

    for variant in &input.variants {
        let variant_ident = &variant.ident;

        match &variant.fields {
            Fields::Unnamed(fields) => {
                let leaf_ident = match fields
                    .unnamed
                    .first()
                    .unwrap_or_else(|| abort!(fields, "Expected an unnamed field"))
                    .ty
                {
                    Type::Path(ref type_path) => {
                        let i = &type_path
                            .path
                            .segments
                            .last()
                            .unwrap_or_else(|| abort!(fields, "Expected a path"))
                            .ident;
                        i
                    }
                    _ => abort!(variant, "Expected a path"),
                };

                let item = Arc::from_iter([
                    regex::LexerItem {
                        ident: Arc::from(leaf_ident.to_string()),
                        variant: Arc::from("parse_token"),
                        transforms: Arc::new([]),
                    },
                    regex::LexerItem {
                        ident: Arc::clone(&enum_name),
                        variant: Arc::from(variant_ident.to_string()),
                        transforms: Arc::clone(&transforms),
                    },
                ]);

                let exprs = variant.attrs.iter().find_map(|attr| {
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
                        _ => None,
                    };

                    hir.map(|hir| (Arc::clone(&item), hir))
                });

                if let Some(regex) = exprs {
                    let leaf_item = &regex.0[0];
                    let mut leaf = create_leaf(
                        vec![(Arc::from_iter([leaf_item.clone()]), regex.1.clone())],
                        leaf_ident,
                    );

                    leaf.extend(quote! {
                        #[common]
                        pub struct #leaf_ident {
                            span: (usize, usize)
                        }

                        impl TokenParser for #leaf_ident {
                            #[inline]
                            fn parse_token(input: Box<str>, state: &mut LexerState) -> PyretResult<Self> {
                                Ok(Self { span: state.spanned(input.len()) })
                            }
                        }
                    });

                    leafs.push(leaf);
                    matches.extend(vec![regex]);
                } else if let Some(regexps) = REGULAR_EXPRESSIONS
                    .lock()
                    .expect("Could not acquire regular expressions lock")
                    .get(&Arc::from(leaf_ident.to_string()))
                {
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
        regex::expand(matches.clone(), enum_ident.span())
    };

    REGULAR_EXPRESSIONS
        .lock()
        .expect("Could not acquire regular expressions lock")
        .insert(enum_name, matches);

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

        #(#leafs)*
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
