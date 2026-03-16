use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;
use syn::Expr;

use crate::sponge_functionlike::parse::{Arg, SpongeCall};

pub fn codegen(call: SpongeCall) -> TokenStream {
    quote! {}
}
