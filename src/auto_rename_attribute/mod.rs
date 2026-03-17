pub mod parse;
pub use parse::parse;

pub mod codegen;
pub use codegen::codegen;

#[cfg(test)]
mod tests {
    use proc_macro2;
    use proc_macro_error2::ResultExt;
    use runtime_macros;
    use runtime_macros::emulate_attributelike_macro_expansion;
    use std::fs;

    fn auto_rename_proc_macro2(
        attr: proc_macro2::TokenStream,
        item: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        use crate::auto_rename_attribute::{codegen, parse};
        codegen(parse(attr.into(), item.into()).unwrap_or_abort()).into()
    }

    #[test]
    fn code_coverage() {
        let file = fs::File::open("tests/auto_rename.rs").unwrap();
        emulate_attributelike_macro_expansion(file, &[("auto_rename", auto_rename_proc_macro2)])
            .unwrap();
        let file = fs::File::open("tests/auto_rename/pass/valid_syntax.rs").unwrap();
        emulate_attributelike_macro_expansion(file, &[("auto_rename", auto_rename_proc_macro2)])
            .unwrap();
    }

    #[should_panic]
    #[test]
    fn code_coverage_panic() {
        let file = fs::File::open("tests/auto_rename/fail/invalid_syntax.rs").unwrap();
        emulate_attributelike_macro_expansion(file, &[("auto_rename", auto_rename_proc_macro2)])
            .unwrap();
    }
}
