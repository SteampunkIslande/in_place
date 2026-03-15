use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, ResultExt};

mod in_place_attribute;

use crate::in_place_attribute::{codegen, parse};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn auto_rename(attr: TokenStream, item: TokenStream) -> TokenStream {
    codegen(parse(attr.into(), item.into()).unwrap_or_abort()).into()
}
