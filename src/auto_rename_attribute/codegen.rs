use crate::auto_rename_attribute::parse::IR;

use proc_macro2::TokenStream;
use quote::quote;

pub fn codegen(ir: IR) -> TokenStream {
    let IR {
        attr,
        body,
        signature,
    } = ir;
    let args_rename_start: Vec<TokenStream> = attr
        .arg_renames
        .iter()
        .cloned()
        .map(|rename| {
            let overwrites = rename.overwrites_ident;
            quote! {
                let #overwrites: ::std::path::PathBuf = {
                    let __p: &::std::path::Path =
                        ::std::convert::AsRef::<::std::path::Path>::as_ref(#overwrites);
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
            }
        })
        .collect();
    let args_rename_end: Vec<TokenStream> = attr
        .arg_renames
        .iter()
        .cloned()
        .map(|rename| {
            let original = rename.original_ident;
            let overwrites = rename.overwrites_ident;
            quote! {
                ::std::fs::rename(&#overwrites, #original)?;
            }
        })
        .collect();
    quote! {
        #signature {
            #(#args_rename_start)*
            #body
            #(#args_rename_end)*
            Ok(())
        }
    }
}
