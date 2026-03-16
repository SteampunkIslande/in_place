use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Expr, Ident, Path, Result, Token,
};

pub enum Arg {
    Expr(Expr),
    Input(Option<Ident>, Expr),
    Output(Option<Ident>, Expr),
}

pub struct SpongeCall {
    pub func: Ident,
    pub args: Vec<Arg>,
}

struct RawArg {
    kind: ArgKind,
    label: Option<Ident>,
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
            let label = if input.peek(Ident) {
                Some(input.parse::<Ident>()?)
            } else {
                None
            };
            if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
            }
            let expr: Expr = input.parse()?;
            Ok(Self {
                kind: ArgKind::Input,
                label,
                expr,
            })
        } else if input.peek(Token![>]) {
            input.parse::<Token![>]>()?;
            let label = if input.peek(Ident) {
                Some(input.parse::<Ident>()?)
            } else {
                None
            };
            if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
            }
            let expr: Expr = input.parse()?;
            Ok(Self {
                kind: ArgKind::Output,
                label,
                expr,
            })
        } else {
            let expr: Expr = input.parse()?;
            Ok(Self {
                kind: ArgKind::Normal,
                label: None,
                expr,
            })
        }
    }
}

impl Parse for SpongeCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let func_path: Path = input.parse()?;
        let func = func_path
            .get_ident()
            .ok_or_else(|| syn::Error::new(func_path.span(), "Expected a simple function name"))?
            .clone();

        let content;
        syn::parenthesized!(content in input);

        let raw_args: Punctuated<RawArg, Token![,]> =
            Punctuated::<RawArg, Token![,]>::parse_terminated(&content)?;

        let args = raw_args
            .into_iter()
            .map(|a| match a.kind {
                ArgKind::Normal => Arg::Expr(a.expr),
                ArgKind::Input => Arg::Input(a.label, a.expr),
                ArgKind::Output => Arg::Output(a.label, a.expr),
            })
            .collect();

        Ok(SpongeCall { func, args })
    }
}

pub fn parse(input: TokenStream) -> Result<SpongeCall> {
    syn::parse2::<SpongeCall>(input)
}
