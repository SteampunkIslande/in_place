use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::sponge_functionlike::parse::{Arg, SpongeCall};

pub fn codegen(call: SpongeCall) -> TokenStream {
    let func = call.func;

    let mut setup = Vec::new();
    let mut args = Vec::new();

    let mut in_count = 0usize;
    let mut out_count = 0usize;

    for arg in call.args {
        match arg {
            Arg::Expr(expr) => {
                args.push(quote! { #expr });
            }

            Arg::Input(expr) => {
                let var = format_ident!("__sponge_input_{}", in_count);
                in_count += 1;

                setup.push(quote! {
                    let #var = ::std::fs::File::open(#expr)?;
                });

                args.push(quote! { #var });
            }

            Arg::Output(expr) => {
                let var = format_ident!("__sponge_output_{}", out_count);
                out_count += 1;

                setup.push(quote! {
                    let #var = ::std::fs::File::create(#expr)?;
                });

                args.push(quote! { #var });
            }
        }
    }

    quote! {
        {
            #(#setup)*

            #func(
                #(#args),*
            )
        }
    }
}
