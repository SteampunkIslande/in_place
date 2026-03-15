use proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Result, Token};

// ---------------------------------------------------------------------------
// Types de données
// ---------------------------------------------------------------------------

/// Un argument passé à la fonction cible : soit une expression Rust normale,
/// soit un marqueur `$<name>i` (input) ou `$<name>o` (inter/output temporaire).
pub enum Arg {
    /// Expression Rust normale (ex: `"incoming.parquet"`, `42`, etc.)
    Expr(Expr),
    /// Référence à l'input d'un fichier : `$<name>i`
    FileInput(Ident),
    /// Référence à l'inter (chemin temporaire) d'un fichier : `$<name>o`
    FileInter(Ident),
}

/// Déclaration d'un fichier à modifier sur place : `name = <expr>`.
pub struct FileSpec {
    pub name: Ident,
    pub path: Expr,
}

/// AST complet de l'invocation `in_place!(…)`.
pub struct Ast {
    pub files: Vec<FileSpec>,
    pub function: Expr,
    pub args: Vec<Arg>,
}

// ---------------------------------------------------------------------------
// Constante de préfixe pour les sentinelles
// ---------------------------------------------------------------------------

const SENTINEL_PREFIX: &str = "__ipm_";
const SENTINEL_SUFFIX: &str = "__";

// ---------------------------------------------------------------------------
// Implémentation de Parse
// ---------------------------------------------------------------------------

impl Parse for Ast {
    fn parse(stream: ParseStream) -> Result<Self> {
        let mut files = Vec::new();

        // Lire les déclarations `name = <expr> ,`  tant qu'il y en a
        while stream.peek(Ident) && stream.peek2(Token![=]) {
            let name: Ident = stream.parse()?;
            stream.parse::<Token![=]>()?;
            let path: Expr = stream.parse()?;
            files.push(FileSpec { name, path });

            if stream.peek(Token![,]) {
                stream.parse::<Token![,]>()?;
                if stream.is_empty() {
                    return Err(stream.error(
                        "in_place! : expression d'appel de fonction manquante après les déclarations de fichiers",
                    ));
                }
            }
        }

        if stream.is_empty() {
            return Err(stream.error("in_place! : une expression d'appel de fonction est requise"));
        }

        // Parser l'appel de fonction (les `$ident` ont été réécrits en sentinelles)
        let call_expr: Expr = stream.parse()?;

        let (function, args) = match call_expr {
            Expr::Call(call) => {
                let func = *call.func;
                let mut parsed_args = Vec::new();

                for arg_expr in call.args {
                    let arg = match &arg_expr {
                        // Un identifiant sentinelle `__ipm_<name>__`
                        Expr::Path(p) if p.path.segments.len() == 1 => {
                            let seg = &p.path.segments[0];
                            let s = seg.ident.to_string();
                            if s.starts_with(SENTINEL_PREFIX) && s.ends_with(SENTINEL_SUFFIX) {
                                // Extraire la partie centrale : `<name><suffix>`
                                let inner = &s[SENTINEL_PREFIX.len()..s.len() - SENTINEL_SUFFIX.len()];
                                // Le dernier caractère est le suffixe 'i' ou 'o'
                                if inner.is_empty() {
                                    return Err(syn::Error::new(seg.ident.span(), "marqueur de fichier vide"));
                                }
                                let (base, suffix) = inner.split_at(inner.len() - 1);
                                let base_ident = Ident::new(base, seg.ident.span());
                                match suffix {
                                    "i" => Arg::FileInput(base_ident),
                                    "o" => Arg::FileInter(base_ident),
                                    _ => return Err(syn::Error::new(
                                        seg.ident.span(),
                                        format!(
                                            "marqueur de fichier invalide `${}{}` : le suffixe doit être `i` (input) ou `o` (output/inter)",
                                            base, suffix
                                        ),
                                    )),
                                }
                            } else {
                                Arg::Expr(arg_expr)
                            }
                        }
                        _ => Arg::Expr(arg_expr),
                    };
                    parsed_args.push(arg);
                }

                (func, parsed_args)
            }
            other => {
                return Err(syn::Error::new_spanned(
                    other,
                    "in_place! : l'expression doit être un appel de fonction, ex: `my_func($f1i, \"arg\", $f1o)`",
                ))
            }
        };

        Ok(Ast {
            files,
            function,
            args,
        })
    }
}

// ---------------------------------------------------------------------------
// Point d'entrée public
// ---------------------------------------------------------------------------

/// Parse un [`TokenStream`] brut en [`Ast`].
///
/// Avant de déléguer à syn, on réécrit chaque séquence `$ <ident>` en un
/// identifiant sentinelle `__ipm_<ident>__` afin que syn accepte l'expression
/// (car `$ident` n'est pas une expression Rust valide).
pub fn parse(ts: TokenStream) -> Result<Ast> {
    let rewritten = rewrite_markers(ts);
    syn::parse2::<Ast>(rewritten)
}

/// Remplace chaque séquence `$ <ident>` par `__ipm_<ident>__`.
///
/// On parcourt le token stream de façon récursive pour traiter les groupes
/// (notamment les parenthèses de l'appel de fonction).
fn rewrite_markers(ts: TokenStream) -> TokenStream {
    use proc_macro2::TokenTree;
    let mut out: Vec<TokenTree> = Vec::new();
    let mut iter = ts.into_iter().peekable();

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == '$' => {
                match iter.peek() {
                    Some(TokenTree::Ident(_)) => {
                        // Consommer l'ident
                        if let Some(TokenTree::Ident(ident)) = iter.next() {
                            let sentinel =
                                format!("{}{}{}", SENTINEL_PREFIX, ident, SENTINEL_SUFFIX);
                            let new_ident = proc_macro2::Ident::new(&sentinel, ident.span());
                            out.push(TokenTree::Ident(new_ident));
                        }
                    }
                    _ => {
                        // Pas suivi d'un ident : on laisse le `$` tel quel
                        out.push(tt);
                    }
                }
            }
            TokenTree::Group(g) => {
                // Récurser dans les groupes (parenthèses, accolades, crochets)
                let inner = rewrite_markers(g.stream());
                let mut new_group = proc_macro2::Group::new(g.delimiter(), inner);
                new_group.set_span(g.span());
                out.push(TokenTree::Group(new_group));
            }
            other => {
                out.push(other);
            }
        }
    }

    out.into_iter().collect()
}
