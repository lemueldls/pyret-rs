mod item;
mod step;

use std::sync::Arc;

pub use item::{LexerItem, RegexMap};
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use re_set::{parse_program, state::CasePattern, ParsedProgram};
use regex::internal::Compiler;
use regex_syntax::hir::Hir;
use step::return_nodes;
use syn::LitInt;

pub fn expand(exprs: &[(Arc<[LexerItem]>, Hir)], span: Span) -> TokenStream {
    let (matches, exprs): (Vec<_>, Vec<_>) = exprs.iter().cloned().unzip();

    let compiler = Compiler::new().bytes(true).only_utf8(true);

    let program = compiler
        .compile(&exprs)
        .unwrap_or_else(|error| abort!(span, error));

    let ParsedProgram { steps, ends } = parse_program(&program);

    let size = (2_u8 << (steps.len() / 256)) * 4;
    let u_shrink = |n| LitInt::new(&format!("{n}u{size}"), span);

    let step_matches = steps
        .into_iter()
        .map(|(position, step_cases)| {
            let char_matches = step_cases.into_iter().map(|step_case| {
                let start = step_case.char_range.start();
                let end = step_case.char_range.end();

                match step_case.next_case {
                    CasePattern::Step(next_step) => {
                        let u_step = u_shrink(next_step);

                        if ends.contains_key(&next_step) {
                            quote! {
                                #start..=#end => {
                                    last_match = (#u_step, i + 1);
                                    step = #u_step;
                                }
                            }
                        } else {
                            quote! {
                                #start..=#end => step = #u_step
                            }
                        }
                    }
                    CasePattern::Match(match_index) => {
                        let default = return_nodes(&matches[match_index], true, true);

                        quote! {
                            #start..=#end => return #default
                        }
                    }
                }
            });

            let end_match = if let Some(match_index) = ends.get(&position) {
                return_nodes(&matches[*match_index], false, true)
            } else {
                quote!(Ok(None))
            };

            let u_position = u_shrink(position);

            quote! {
                #u_position => match next {
                    #(#char_matches,)*
                    _ => return #end_match
                }
            }
        })
        .collect::<Vec<_>>();

    let end_matches = ends.iter().map(|(step, match_index)| {
        let u_step = u_shrink(*step);
        let pattern = return_nodes(&matches[*match_index], false, false);

        quote! {
            #u_step => #pattern
        }
    });

    quote! {
        #[inline]
        fn lex(state: &mut LexerState) -> PyretResult<::std::option::Option<Self>> {
            let input = &state.source[state.next_position..];

            let mut last_match = (0, 0);
            let mut step = 0;

            for (i, next) in input.as_bytes().iter().enumerate() {
                match step {
                    #(#step_matches,)*
                    _ => unreachable!()
                }
            }

            match last_match.0 {
                #(#end_matches,)*
                _ => Ok(None),
            }
        }
    }
}
