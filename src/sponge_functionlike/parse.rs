use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Result, Token,
};

pub enum Arg {
    Expr(Expr),
    Input(Expr),
    Output(Expr),
}

pub struct SpongeCall {
    pub func: Expr,
    pub args: Vec<Arg>,
}

struct RawArg {
    kind: ArgKind,
    expr: Expr,
}

enum ArgKind {
    Normal,
    Input,
    Output,
}

impl Parse for RawArg {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            let expr: Expr = input.parse()?;
            Ok(Self {
                kind: ArgKind::Input,
                expr,
            })
        } else if input.peek(Token![>]) {
            input.parse::<Token![>]>()?;
            let expr: Expr = input.parse()?;
            Ok(Self {
                kind: ArgKind::Output,
                expr,
            })
        } else {
            let expr: Expr = input.parse()?;
            Ok(Self {
                kind: ArgKind::Normal,
                expr,
            })
        }
    }
}

impl Parse for SpongeCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let func: Expr = input.parse()?;

        let content;
        syn::parenthesized!(content in input);

        let raw_args: Punctuated<RawArg, Token![,]> =
            content.parse_terminated(RawArg::parse, Token![,])?;

        let args = raw_args
            .into_iter()
            .map(|a| match a.kind {
                ArgKind::Normal => Arg::Expr(a.expr),
                ArgKind::Input => Arg::Input(a.expr),
                ArgKind::Output => Arg::Output(a.expr),
            })
            .collect();

        Ok(SpongeCall { func, args })
    }
}

pub fn parse(attr: TokenStream, item: TokenStream) -> Result<SpongeCall> {
    let ParsedFunctionParts { body, signature } = syn::parse2::<ParsedFunctionParts>(item)?;
    let attr = syn::parse2::<Renames>(attr)?;
    Ok(IR {
        attr,
        body,
        signature,
    })
}
