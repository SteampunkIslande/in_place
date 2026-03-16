use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::sponge_functionlike::parse::{Arg, SpongeCall};

pub fn codegen(call: SpongeCall) -> TokenStream {
    let SpongeCall {
        func_path,
        func,
        args,
    } = call;

    let mut pre_stmts = Vec::new();
    let mut post_stmts = Vec::new();
    let mut call_args: Vec<TokenStream> = Vec::new();

    for (i, arg) in args.into_iter().enumerate() {
        match arg {
            Arg::Simple(e) => {
                call_args.push(quote! { #e });
            }
            Arg::Overwrites(e) => {
                let tmp_ident = format_ident!("__sponge_tmp_{}", i);
                // create temp path and a string representation to pass to functions expecting &str
                pre_stmts.push(quote! {
                    let #tmp_ident: ::std::path::PathBuf = {
                        let __p: &::std::path::Path = ::std::convert::AsRef::<::std::path::Path>::as_ref(#e);
                        let __stem = __p.file_stem().unwrap_or_default().to_string_lossy();
                        let __ext = __p.extension().unwrap_or_default().to_string_lossy();
                        let __dir = __p.parent().unwrap_or_else(|| ::std::path::Path::new("."));
                        __dir.join(::std::format!("{}.tmp.{}", __stem, __ext))
                    };
                });

                post_stmts.push(quote! {
                    ::std::fs::rename(&#tmp_ident, #e)?;
                });

                call_args.push(quote! { &#tmp_ident });
            }
        }
    }

    let call_path = if let Some(p) = func_path {
        quote! { #p::#func }
    } else {
        quote! { #func }
    };

    quote! {
        {
            #(#pre_stmts)*
            match #call_path(#(#call_args),*) {
                Ok(__sponge_ok) => {
                    #(#post_stmts)*
                    Ok(__sponge_ok)
                }
                Err(__sponge_err) => Err(__sponge_err),
            }
        }
    }
}
