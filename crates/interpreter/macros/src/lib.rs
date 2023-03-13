use std::{iter, rc::Rc};

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro_error::{abort, proc_macro_error, ResultExt};
use quote::{quote, ToTokens};
use syn::{
    parse, parse::Parse, punctuated::Punctuated, Ident, ImplItem, ItemFn, ItemImpl, Pat, Path,
    Token, Type, TypeReference,
};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl: ItemImpl = parse(item).unwrap_or_abort();

    let ident = &item_impl.self_ty;

    let tokens = item_impl.items.iter().map(|item| match item {
        ImplItem::Method(method) => {
            let ident = &method.sig.ident;
            let name = ident.to_string().to_case(Case::Kebab);

            let mut types = Vec::new();

            let args = method
                .sig
                .inputs
                .iter()
                .enumerate()
                .map(|(i, input)| match input {
                    syn::FnArg::Receiver(_) => abort!(input, "Unsupported type"),
                    syn::FnArg::Typed(typed) => {
                        let ident = match &*typed.pat {
                            Pat::Ident(pat_ident) => &pat_ident.ident,
                            _ => abort!(typed.pat, "Unsupported type"),
                        };
                        let (ty, is_ref) = match &*typed.ty {
                            ty @ Type::Path(..) => (ty, false),
                            Type::Reference(reference) => (&*reference.elem, true),
                            _ => abort!(typed.ty, "Unsupported type"),
                        };

                        if ident == "context" {
                            quote!(context)
                        } else {
                            types.push(ty.clone());

                            let ampersand = is_ref.then(|| quote!(&));

                            quote!(#ampersand #ty(Rc::clone(&args[#i])))
                        }
                    }
                })
                .collect::<Vec<_>>();

            let (return_type, is_result) = match &method.sig.output {
                syn::ReturnType::Default => abort!(method.sig, "Unsupported type"),
                syn::ReturnType::Type(_, ty) => match &**ty {
                    Type::Path(ty_path) => {
                        let segment = ty_path.path.segments.first().unwrap();

                        if segment.ident == "PyretResult" {
                            match segment.arguments {
                                syn::PathArguments::AngleBracketed(ref data) => {
                                    (data.args.first().unwrap().into_token_stream(), true)
                                }
                                _ => abort!(ty, "Unsupported type"),
                            }
                        } else {
                            (segment.into_token_stream(), false)
                        }
                    }
                    _ => abort!(ty, "Unsupported type"),
                },
            };

            let question = is_result.then(|| quote!(?));

            quote! {
                registrar.register_function(
                    Box::from(#name),
                    Box::from_iter([#(#types::predicate()),*]),
                    #return_type::predicate(),
                    Rc::new(|args, context| Ok(Self::#ident(#(#args),*)#question.0)),
                )?;
            }
        }
        _ => abort!(item, "Unsupported type"),
    });

    let expanded = quote! {
        #item_impl

        impl crate::trove::Trove for #ident {
            fn register(registrar: &mut Registrar) -> PyretResult<()> {
                #(#tokens)*

                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
