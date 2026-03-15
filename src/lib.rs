use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, ResultExt};

mod in_place;
mod in_place_attribute;

use crate::in_place_attribute::{codegen, parse};

use crate::in_place::codegen;
use crate::in_place::parse;

#[proc_macro]
#[proc_macro_error]
pub fn in_place(ts: TokenStream) -> TokenStream {
    match parse::parse(ts.into()) {
        Ok(ast) => codegen::codegen(ast).into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn auto_rename(attr: TokenStream, item: TokenStream) -> TokenStream {
    codegen(parse(attr.into(), item.into()).unwrap_or_abort()).into()
}
