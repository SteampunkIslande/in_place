use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, ResultExt};

mod auto_rename_attribute;
mod sponge_functionlike;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn auto_rename(attr: TokenStream, item: TokenStream) -> TokenStream {
    use crate::auto_rename_attribute::{codegen, parse};
    codegen(parse(attr.into(), item.into()).unwrap_or_abort()).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn sponge(input: TokenStream) -> TokenStream {
    use crate::sponge_functionlike::{codegen, parse};
    codegen(parse(input.into()).unwrap_or_abort()).into()
}
