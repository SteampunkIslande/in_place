use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::in_place::parse::{Arg, Ast};

pub fn codegen(ast: Ast) -> TokenStream {
    let Ast {
        files,
        function,
        args,
    } = ast;

    // ------------------------------------------------------------------
    // 1. Déclarations des variables input et inter pour chaque fichier
    //
    //  let <name>_input = AsRef::<Path>::as_ref(<path_expr>);
    //  let <name>_inter = {
    //      let __p: &Path = AsRef::<Path>::as_ref(<path_expr>);
    //      let __stem = __p.file_stem().unwrap_or_default().to_string_lossy();
    //      let __ext  = __p.extension().unwrap_or_default().to_string_lossy();
    //      let __dir  = __p.parent().unwrap_or_else(|| Path::new("."));
    //      __dir.join(format!("{}.tmp.{}", __stem, __ext))
    //  };
    // ------------------------------------------------------------------
    let file_declarations: Vec<TokenStream> = files
        .iter()
        .map(|fs| {
            let name_input = Ident::new(&format!("{}_input", fs.name), fs.name.span());
            let name_inter = Ident::new(&format!("{}_inter", fs.name), fs.name.span());
            let path_expr = &fs.path;
            quote! {
                let #name_input: &::std::path::Path =
                    ::std::convert::AsRef::<::std::path::Path>::as_ref(#path_expr);
                let #name_inter: ::std::path::PathBuf = {
                    let __p: &::std::path::Path =
                        ::std::convert::AsRef::<::std::path::Path>::as_ref(#path_expr);
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

    // ------------------------------------------------------------------
    // 2. Construction de la liste des arguments de l'appel de fonction
    // ------------------------------------------------------------------
    let call_args: Vec<TokenStream> = args
        .iter()
        .map(|arg| match arg {
            Arg::Expr(e) => quote! { #e },
            Arg::FileInput(name) => {
                let name_input = Ident::new(&format!("{}_input", name), name.span());
                quote! { #name_input }
            }
            Arg::FileInter(name) => {
                let name_inter = Ident::new(&format!("{}_inter", name), name.span());
                quote! { &#name_inter }
            }
        })
        .collect();

    // ------------------------------------------------------------------
    // 3. Renommages : `fs::rename(inter, input)` pour chaque fichier
    // ------------------------------------------------------------------
    let renames: Vec<TokenStream> = files
        .iter()
        .map(|fs| {
            let name_input = Ident::new(&format!("{}_input", fs.name), fs.name.span());
            let name_inter = Ident::new(&format!("{}_inter", fs.name), fs.name.span());
            quote! {
                ::std::fs::rename(&#name_inter, #name_input)
                    .expect("in_place!: impossible de renommer le fichier temporaire");
            }
        })
        .collect();

    // ------------------------------------------------------------------
    // 4. Bloc final
    // ------------------------------------------------------------------
    quote! {
        {
            #(#file_declarations)*
            #function(#(#call_args),*)?;
            #(#renames)*
            Ok(())
        }
    }
}
