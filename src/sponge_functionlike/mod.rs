pub mod parse;
pub use parse::parse;

pub mod codegen;
pub use codegen::codegen;

#[cfg(test)]
mod tests {
    use proc_macro2;
    use proc_macro_error2::ResultExt;
    use runtime_macros;
    use runtime_macros::emulate_functionlike_macro_expansion;
    use std::fs;

    fn sponge_proc_macro2(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        use crate::sponge_functionlike::{codegen, parse};
        codegen(parse(input.into()).unwrap_or_abort()).into()
    }

    #[test]
    fn code_coverage() {
        let file = fs::File::open("tests/sponge.rs").unwrap();
        emulate_functionlike_macro_expansion(file, &[("sponge", sponge_proc_macro2)]).unwrap();
        let file = fs::File::open("tests/sponge/pass/valid_syntax.rs").unwrap();
        emulate_functionlike_macro_expansion(file, &[("sponge", sponge_proc_macro2)]).unwrap();
    }

    #[should_panic]
    #[test]
    fn code_coverage_panic() {
        let file = fs::File::open("tests/sponge/fail/invalid_syntax.rs").unwrap();
        emulate_functionlike_macro_expansion(file, &[("sponge", sponge_proc_macro2)]).unwrap();
    }
}
