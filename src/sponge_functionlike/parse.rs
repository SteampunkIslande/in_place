use proc_macro2::TokenStream;
use proc_macro_error::{abort, OptionExt};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Path, Result, Token,
};

pub struct SpongeCall {
    pub func_path: Option<Path>,
    pub func: Ident,
    pub args: Vec<Arg>,
}

pub enum Arg {
    Simple(Expr),
    Overwrites(Expr),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.parse::<Ident>()?.to_string() != "overwrites" {
            Ok(Arg::Simple(input.parse()?))
        } else {
            Ok(Arg::Overwrites(input.parse()?))
        }
    }
}

impl Parse for SpongeCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_path: Path = input.parse()?;
        let func = func_path
            .segments
            .last()
            .expect_or_abort("Doesn't make sense, a path should have at least one segment...")
            .ident
            .clone();

        let content;
        syn::parenthesized!(content in input);

        let args: Vec<Arg> = Punctuated::<Arg, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(SpongeCall {
            func_path: (if func_path.get_ident().is_some() {
                None
            } else {
                Some(func_path)
            }),
            func,
            args,
        })
    }
}

pub fn parse(input: TokenStream) -> Result<SpongeCall> {
    syn::parse2::<SpongeCall>(input)
}
