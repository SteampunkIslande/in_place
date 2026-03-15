use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, ItemFn, Token,
};

pub struct IR {
    pub attr: Renames,
    pub body: TokenStream,
    pub signature: TokenStream,
}

#[derive(Clone)]
pub struct Rename {
    pub original_ident: Ident,
    pub overwrites_ident: Ident,
}

pub struct Renames {
    pub arg_renames: Vec<Rename>,
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
