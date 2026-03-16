use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::sponge_functionlike::parse::SpongeCall;

pub fn codegen(call: SpongeCall) -> TokenStream {
    let SpongeCall {
        func_path,
        func,
        args,
        overwrites,
    } = call;

    let mut pre_stmts = Vec::new();
    let mut post_stmts = Vec::new();
    let mut call_args: Vec<TokenStream> = Vec::new();

    // First, create let-bindings for all named arguments so temporaries live long enough
    for (i, arg) in args.iter().enumerate() {
        let expr = &arg.expr;
        let binding = format_ident!("__sponge_arg_{}", i);
        pre_stmts.push(quote! {
            let #binding = #expr;
        });
    }

    for (i, arg) in args.iter().enumerate() {
        let binding = format_ident!("__sponge_arg_{}", i);

        // Check if this argument is an "output" in any overwrite clause
        let is_overwrite = overwrites.iter().any(|ow| ow.output == arg.name);

        if is_overwrite {
            let tmp_ident = format_ident!("__sponge_tmp_{}", i);
            // Create a temp path next to the original file
            pre_stmts.push(quote! {
                let #tmp_ident: ::std::path::PathBuf = {
                    let __p: &::std::path::Path = ::std::convert::AsRef::<::std::path::Path>::as_ref(&#binding);
                    let __stem = __p.file_stem().unwrap_or_default().to_string_lossy();
                    let __ext = __p.extension().unwrap_or_default().to_string_lossy();
                    let __dir = __p.parent().unwrap_or_else(|| ::std::path::Path::new("."));
                    __dir.join(::std::format!("{}.tmp.{}", __stem, __ext))
                };
            });

            // Find the original that this output overwrites
            let original_clause = overwrites.iter().find(|ow| ow.output == arg.name).unwrap();

            // Find the index of the original argument
            let original_idx = args
                .iter()
                .position(|a| a.name == original_clause.original)
                .unwrap();
            let original_binding = format_ident!("__sponge_arg_{}", original_idx);

            post_stmts.push(quote! {
                ::std::fs::rename(&#tmp_ident, ::std::convert::AsRef::<::std::path::Path>::as_ref(&#original_binding))?;
            });

            call_args.push(quote! { #tmp_ident.as_path() });
        } else {
            call_args.push(quote! { #binding });
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
