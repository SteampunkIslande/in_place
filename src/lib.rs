use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod in_place;

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
