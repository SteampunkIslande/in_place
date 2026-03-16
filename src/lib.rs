//! In-place file editing helpers (procedural macros)
//!
//! This crate provides two complementary macros to safely perform in-place
//! modifications of files while avoiding data loss caused by truncation:
//!
//! - `#[auto_rename(...)]` — an attribute macro applied to functions that take
//!   separate input and output file paths. The macro rewrites the function so
//!   that it first writes output to a temporary file and then atomically renames
//!   that temporary file over the original input path. This preserves the
//!   original file until a full, successful write has completed.
//!
//! - `sponge!(expr, options)` — a function-like macro that wraps an expression
//!   which performs a write into an output path. The macro evaluates the
//!   expression with the output path redirected to a temporary file, and on
//!   success renames the temporary file over the input path. This is useful for
//!   quick call-site in-place edits without declaring a separate function.
//!
//! Both macros are intended for workflows where a function or expression would
//! otherwise open the same filesystem path for reading and writing, which would
//! ordinarily truncate the file and lead to data corruption. These utilities
//! implement the well-known "write-to-temp-and-rename" pattern to provide an
//! atomic replace semantics on platforms that support `std::fs::rename`.

use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, ResultExt};

mod auto_rename_attribute;
mod sponge_functionlike;

/// Attribute macro that transforms a function to perform safe in-place edits.
///
/// Usage
/// -----
///
/// ```rust,ignore
/// # use std::path::Path;
/// # use in_place_macro::auto_rename;
/// #[auto_rename(output overwrites input)]
/// pub fn edit_file(input: &Path, output: &Path) -> std::io::Result<()> {
///     // write changes to `output` (which will be a temporary file when the macro
///     // is applied). On success the temp file will replace `input`.
/// }
/// ```
///
/// Semantics
/// ---------
/// - The macro expects the attributed function to accept separate input and
///   output file paths (commonly `&Path`) and to return `Result<..., E>` where
///   `E` is compatible with `std::io::Error` (the macro does not enforce the
///   exact error type but preserves the function's return type).
/// - At call-time when the same path is provided for both `input` and `output`,
///   the expanded code will create a temporary file next to the original file,
///   invoke the original function body writing into that temporary file, and on
///   success rename the temporary file over the original. This avoids the
///   truncate-then-read corruption described in the tests.
/// - If the underlying write fails, the temporary file is left for inspection
///   or automatically cleaned up by the OS / temporary-file helper; the original
///   file is preserved.
///
/// Options
/// -------
/// The attribute accepts a short, declarative option clause describing how the
/// output relates to the input. Current tests exercise the form
/// `output overwrites input`. The macro parses the attribute tokens to decide
/// which parameter is the input and which is the output — this allows clear
/// call-site intent and explicitness in the function signature.
///
/// Safety and portability
/// ----------------------
/// - The atomicity of the final replace depends on the platform's `rename`
///   semantics. On POSIX systems a `rename` over an existing path is atomic; on
///   some filesystems or platforms behaviour may differ. Use in contexts where
///   `std::fs::rename` provides the desired semantics.
/// - The macro does not attempt cross-filesystem moves: the temporary file is
///   created next to the original file to increase the likelihood that `rename`
///   will succeed without requiring a copy.
///
/// Example (based on tests)
/// ------------------------
/// ```ignore
/// // Without the macro, calling a function that opens the same path for
/// // File::create (truncate) and File::open (read) will produce an empty
/// // result. With the macro the file is edited correctly.
/// #[auto_rename(output overwrites input)]
/// pub fn file_edit_inplace(input: &Path, output: &Path) -> std::io::Result<()> {
///     // implementation unchanged — macro rewrites the call site behaviour
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn auto_rename(attr: TokenStream, item: TokenStream) -> TokenStream {
    use crate::auto_rename_attribute::{codegen, parse};
    codegen(parse(attr.into(), item.into()).unwrap_or_abort()).into()
}

/// Function-like macro for in-place editing of a path by wrapping an
/// expression that writes to an output path.
///
/// Usage
/// -----
/// ```rust,ignore
/// # use in_place_macro::sponge;
/// # use std::path::Path;
/// // Call-site form: the first argument is an expression that performs the
/// // write using two `Path`-like values. The second clause describes which
/// // path overwrites which (e.g. `output overwrites input`).
/// sponge!(file_edit(input, output), output overwrites input)?;
/// ```
///
/// Semantics
/// ---------
/// - The macro evaluates to the result of the wrapped expression, but ensures
///   that when the expression is intended to overwrite the input path it is
///   executed using a temporary output file that is renamed into place on
///   success. This makes it convenient to call existing functions without
///   writing a separate attribute-on-function wrapper.
/// - Errors produced by the wrapped expression are propagated unchanged.
///
/// Example (based on tests)
/// ------------------------
/// ```ignore
/// sponge!(file_edit(input, output), output overwrites input)
///     .expect("file_edit_inplace should not return an error");
/// ```
#[proc_macro]
#[proc_macro_error]
pub fn sponge(input: TokenStream) -> TokenStream {
    use crate::sponge_functionlike::{codegen, parse};
    codegen(parse(input.into()).unwrap_or_abort()).into()
}
