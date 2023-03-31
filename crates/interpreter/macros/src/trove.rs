use std::{fs, path::PathBuf, rc::Rc};

use bincode::{config, serde};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use pyret_lexer::{ast, lex};
use quote::quote;
use syn::{
    parse,
    parse::{Parse, ParseBuffer},
    LitStr, Token,
};

struct TroveParser {
    name: Box<str>,
    context: Ident,
}

impl Parse for TroveParser {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        let path = input.parse::<LitStr>()?.value();
        input.parse::<Token![,]>()?;
        let context = input.parse::<Ident>()?;

        Ok(Self {
            name: path.into_boxed_str(),
            context,
        })
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse::<TroveParser>(input).unwrap();

    let name = &*input.name;
    let context = input.context;

    let path = fs::canonicalize(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../src/trove/arr")
            .join(name)
            .with_extension("arr"),
    )
    .unwrap();

    let source = fs::read_to_string(path).unwrap();

    let tokens = lex(&source).unwrap();

    let config = config::standard();
    let bytes = serde::encode_to_vec(tokens, config).unwrap();

    let expanded = quote! {{
        struct TroveGraph {
            files: std::vec::Vec<::pyret_file::PyretFile>,
        }

        impl ::pyret_file::graph::PyretGraph for TroveGraph {
            fn register(&mut self, name: &str) -> usize {
                unimplemented!()
            }

            fn get(&self, file_id: usize) -> &::pyret_file::PyretFile {
                unimplemented!()
            }
        }

        let graph = TroveGraph {
            files: ::std::vec![
                ::pyret_file::PyretFile::new(
                    Box::from(#name),
                    Box::from(#source),
                )
            ]
        };

        let (tokens, _): (::std::vec::Vec<::pyret_lexer::ast::Statement>, _) =
            ::bincode::serde::decode_from_slice(&[#(#bytes),*], ::bincode::config::standard()).unwrap();

        let mut interpreter = crate::Interpreter::new(graph);
        interpreter.interpret_block(tokens)?;

        let provided = interpreter.get_provided();

        #context.borrow_mut().declarations.extend(provided);

        Ok(())
    }};

    TokenStream::from(expanded)
}
