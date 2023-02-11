use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::item::LexerItem;

pub fn return_nodes(nodes: &[LexerItem], use_offset: bool, i_slice: bool) -> TokenStream {
    let input = if i_slice {
        let range = if use_offset { quote!(..=) } else { quote!(..) };

        quote!(&input[#range i])
    } else {
        quote!(&input[..=last_match.1])
    };

    let mut last_step = quote!((Box::from(#input), state)?);

    for LexerItem {
        ident,
        variant,
        transform,
    } in nodes
    {
        let (ident, variant) = (format_ident!("{ident}"), format_ident!("{variant}"));

        last_step = quote!(crate::ast::#ident::#variant #last_step);

        if let Some(transform) = transform {
            let transform = format_ident!("{transform}");

            last_step.extend(quote!(.#transform(state)?));
        };

        last_step = quote!((#last_step));
    }

    quote!(Ok(Some #last_step ))
}
