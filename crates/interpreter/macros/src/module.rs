use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro_error::{abort, ResultExt};
use quote::{quote, ToTokens};
use syn::{parse, ImplItem, ItemImpl, Pat, Type};

pub fn expand(item: TokenStream) -> TokenStream {
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
                .map(|input| match input {
                    syn::FnArg::Receiver(_) => abort!(input, "Unsupported type"),
                    syn::FnArg::Typed(pat_typed) => {
                        let ident = match &*pat_typed.pat {
                            Pat::Ident(pat_ident) => &pat_ident.ident,
                            _ => abort!(pat_typed.pat, "Unsupported type"),
                        };
                        let (ty, is_ref) = match &*pat_typed.ty {
                            ty @ Type::Path(..) => (ty, false),
                            Type::Reference(reference) => (&*reference.elem, true),
                            _ => abort!(pat_typed.ty, "Unsupported type"),
                        };

                        if ident == "context" {
                            quote!(context)
                        } else {
                            types.push(ty.clone());

                            let ampersand = is_ref.then(|| quote!(&));

                            quote!(#ampersand #ty(args.next().unwrap()))
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
                context.register_function(
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
            fn register(context: Rc<RefCell<crate::Context>>) -> PyretResult<()> {
                use crate::Register;

                #(#tokens)*

                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}
