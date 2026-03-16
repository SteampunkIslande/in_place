use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

use crate::sponge_functionlike::parse::{Arg, SpongeCall};

pub fn codegen(call: SpongeCall) -> TokenStream {
    let func = call.func;

    let mut setup = Vec::new();
    let mut args = Vec::new();
    let mut epilog = Vec::new();

    let mut input_count = 0usize;
    let mut output_count = 0usize;
    let mut label_inputs: HashMap<String, proc_macro2::TokenStream> = HashMap::new();

    for arg in call.args {
        match arg {
            Arg::Expr(expr) => {
                args.push(quote! { #expr });
            }

            Arg::Input(label, expr) => {
                let var = format_ident!("__sponge_input_{}", input_count);
                input_count += 1;

                setup.push(quote! {
                    let #var = ::std::fs::File::create(#expr)?;
                });

                if let Some(lbl) = label {
                    label_inputs.insert(lbl.to_string(), quote! { #expr });
                }

                args.push(quote! { #var });
            }

            Arg::Output(label, expr) => {
                let overwrites = format_ident!("__sponge_output_{}", output_count);
                output_count += 1;

                setup.push(quote! {
                    let #overwrites: ::std::path::PathBuf = {
                    let __p: &::std::path::Path =
                        ::std::convert::AsRef::<::std::path::Path>::as_ref(#expr);
                    let __stem = __p
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let __ext = __p
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let __dir = __p
                        .parent()
                        .unwrap_or_else(|| ::std::path::Path::new("."));
                    __dir.join(::std::format!("{}.tmp.{}", __stem, __ext))
                    };

                });

                let original_target = if let Some(lbl) = label {
                    if let Some(orig_expr) = label_inputs.get(&lbl.to_string()) {
                        quote! { #orig_expr }
                    } else {
                        quote! { #expr }
                    }
                } else {
                    quote! { #expr }
                };

                epilog.push(quote! {
                    ::std::fs::rename(&#overwrites, #original_target)?;
                });

                args.push(quote! { #overwrites });
            }
        }
    }

    quote! {
        {
            #(#setup)*

            #func(
                #(#args),*
            )
            #(#epilog)*
        }
    }
}
