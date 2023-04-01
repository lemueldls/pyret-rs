mod item;
mod step;

use std::sync::Arc;

pub use item::{LexerItem, RegexMap};
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use re_set::{state::CasePattern, ProgramPatterns};
use regex::internal::Compiler;
use regex_syntax::hir::Hir;
use step::return_nodes;
use syn::LitInt;

pub fn expand(exprs: Vec<(Arc<[LexerItem]>, Hir)>, span: Span) -> TokenStream {
    let (matches, exprs): (Vec<_>, Vec<_>) = exprs.into_iter().unzip();

    let compiler = Compiler::new().bytes(true);

    let program = compiler
        .compile(&exprs)
        .unwrap_or_else(|error| abort!(span, error));

    let patterns = ProgramPatterns::new(&program);

    let step_size = patterns.step_size();
    let u_shrink = |n| LitInt::new(&format!("{n}u{step_size}"), span);

    let u_first = u_shrink(patterns.first_step());

    let step_matches = patterns
        .steps
        .into_iter()
        .map(|(position, step_cases)| {
            let char_matches = step_cases.into_iter().map(|step_case| {
                let start = step_case.byte_range.start();
                let end = step_case.byte_range.end();

                match step_case.next_case {
                    CasePattern::Step(next_step, conditions) => {
                        let u_step = u_shrink(next_step);

                        if patterns.ends.contains_key(&next_step) {
                            quote! {
                                #start..=#end => {
                                    last_match = (#u_step, i);
                                    step = #u_step;
                                }
                            }
                        } else {
                            let conditions = conditions.into_iter().map(|(step, range)| {
                                let start = range.start();
                                let end = range.end();

                                let u_step = u_shrink(step);

                                quote! {
                                    if (#start..=#end).contains(&next) {
                                        last_match = (#u_step, i);
                                    }
                                }
                            });

                            quote! {
                                #start..=#end => {
                                    #(#conditions)*

                                    step = #u_step
                                }
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

            let u_position = u_shrink(position);

            let end_match = if let Some(match_index) = patterns.ends.get(&position) {
                let default = return_nodes(&matches[*match_index], false, true);

                quote!(return #default)
            } else {
                quote!(break)
            };

            quote! {
                #u_position => match next {
                    #(#char_matches,)*
                    _ => #end_match
                }
            }
        })
        .collect::<Vec<_>>();

    let end_matches = patterns.ends.iter().map(|(step, match_index)| {
        let u_step = u_shrink(*step);
        let pattern = return_nodes(&matches[*match_index], true, false);

        quote! {
            #u_step => #pattern
        }
    });

    quote! {
        #[inline]
        fn lex_token(state: &mut LexerState) -> PyretResult<::std::option::Option<Self>> {
            let input = &state.source[state.next_position..];

            let mut last_match = (#u_first, 0);
            let mut step = #u_first;

            for (i, next) in input.as_bytes().iter().enumerate() {
                match step {
                    #(#step_matches,)*
                    _ => unreachable!("{{ i: {}, step: {}, next: {}}}", i, step, next)
                }
            }

            match last_match.0 {
                #(#end_matches,)*
                _ => Ok(None),
            }
        }
    }
}
