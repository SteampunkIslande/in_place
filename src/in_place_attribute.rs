use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, ItemFn, Token,
};

pub struct IR {
    attr: Renames,
    body: TokenStream,
    signature: TokenStream,
}

#[derive(Clone)]
pub struct Rename {
    original_ident: Ident,
    overwrites_ident: Ident,
}

pub struct Renames {
    arg_renames: Vec<Rename>,
}

pub struct ParsedFunctionParts {
    pub signature: TokenStream,
    pub body: TokenStream,
}

impl Parse for Rename {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let overwrites_ident: Ident = input.parse()?;
        let _ = input.parse::<Ident>()?; // On met ce qu'on veut entre input_arg et output_arg
        let original_ident: Ident = input.parse()?;
        Ok(Rename {
            original_ident,
            overwrites_ident,
        })
    }
}

impl Parse for Renames {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Renames {
            arg_renames: Punctuated::<Rename, Token![,]>::parse_terminated(input)?
                .into_iter()
                .collect(),
        })
    }
}

impl Parse for ParsedFunctionParts {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item_fn: ItemFn = input.parse()?;

        let vis = &item_fn.vis;
        let sig = &item_fn.sig;
        let block = &item_fn.block;

        let signature = quote! {
            #vis #sig
        };

        let body = quote! {
            #block
        };

        Ok(ParsedFunctionParts { signature, body })
    }
}

pub fn parse(attr: TokenStream, item: TokenStream) -> Result<IR, syn::Error> {
    let ParsedFunctionParts { body, signature } = syn::parse2::<ParsedFunctionParts>(item)?;
    let attr = syn::parse2::<Renames>(attr)?;
    Ok(IR {
        attr,
        body,
        signature,
    })
}

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
                ::std::fs::rename(&#overwrites, #original).unwrap();
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
