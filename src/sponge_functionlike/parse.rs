use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Path, Result, Token,
};

/// Represents a single argument in the function call.
/// It can be either:
/// - `ident = expr` (keyword style)
/// - `expr` (positional style, where the ident is inferred if the expr is a simple ident)
#[derive(Clone)]
pub struct Arg {
    pub name: Ident,
    pub expr: Expr,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        // Try to parse `ident = expr` first
        if input.peek(Ident) && input.peek2(Token![=]) && !input.peek2(Token![==]) {
            let name: Ident = input.parse()?;
            let _eq: Token![=] = input.parse()?;
            let expr: Expr = input.parse()?;
            Ok(Arg { name, expr })
        } else {
            // Parse as a simple expression
            let expr: Expr = input.parse()?;
            // Try to extract an ident from the expression
            let name = if let Expr::Path(ref ep) = expr {
                if ep.path.segments.len() == 1 {
                    ep.path.segments[0].ident.clone()
                } else {
                    return Err(syn::Error::new_spanned(
                        &expr,
                        "positional arguments must be simple identifiers or use `name = expr` syntax",
                    ));
                }
            } else {
                return Err(syn::Error::new_spanned(
                    &expr,
                    "positional arguments must be simple identifiers or use `name = expr` syntax",
                ));
            };
            Ok(Arg { name, expr })
        }
    }
}

/// An overwrite clause: `output overwrites original`
pub struct OverwriteClause {
    pub output: Ident,
    pub original: Ident,
}

impl Parse for OverwriteClause {
    fn parse(input: ParseStream) -> Result<Self> {
        let output: Ident = input.parse()?;
        let kw: Ident = input.parse()?;
        if kw != "overwrites" {
            return Err(syn::Error::new_spanned(kw, "expected `overwrites` keyword"));
        }
        let original: Ident = input.parse()?;
        Ok(OverwriteClause { output, original })
    }
}

/// The full parsed representation of a `sponge!` invocation.
pub struct SpongeCall {
    /// The path prefix of the function (e.g. `module::submodule`), if any.
    pub func_path: Option<Path>,
    /// The function name itself.
    pub func: Ident,
    /// The parsed arguments.
    pub args: Vec<Arg>,
    /// The overwrite clauses.
    pub overwrites: Vec<OverwriteClause>,
}

impl Parse for SpongeCall {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the function path (e.g. `module::func` or just `func`)
        let func_path_full: Path = input.parse()?;

        // Split into path prefix and function ident
        let (func_path, func) = {
            let mut segments = func_path_full.segments.clone();
            let last = segments
                .pop()
                .expect("a path should have at least one segment")
                .into_value();
            let func = last.ident;
            if segments.is_empty() {
                (None, func)
            } else {
                let mut path = func_path_full.clone();
                // Remove the last segment (the function name)
                path.segments = segments;
                (Some(path), func)
            }
        };

        // Parse the parenthesized arguments
        let content;
        syn::parenthesized!(content in input);

        let args: Vec<Arg> = Punctuated::<Arg, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        // Parse overwrite clauses after a comma
        let mut overwrites = Vec::new();
        if input.peek(Token![,]) {
            let _comma: Token![,] = input.parse()?;
            let overwrite_clauses =
                Punctuated::<OverwriteClause, Token![,]>::parse_terminated(input)?;
            overwrites = overwrite_clauses.into_iter().collect();
        }

        Ok(SpongeCall {
            func_path,
            func,
            args,
            overwrites,
        })
    }
}

pub fn parse(input: TokenStream) -> Result<SpongeCall> {
    syn::parse2::<SpongeCall>(input)
}
