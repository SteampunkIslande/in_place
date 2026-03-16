use in_place::sponge;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
fn aggregate_parquets(_original: &Path, _incoming: &Path, _output: &Path) -> Result<()> {
    Ok(())
}
fn run() -> Result<()> {
    let original_path = PathBuf::from("original.parquet");
    let incoming_path = PathBuf::from("incoming.parquet");
    let output_path = PathBuf::from("original.parquet");
    {
        let __sponge_arg_0 = original_path.as_path();
        let __sponge_arg_1 = incoming_path.as_path();
        let __sponge_arg_2 = output_path.as_path();
        let __sponge_tmp_2: ::std::path::PathBuf = {
            let __p: &::std::path::Path = ::std::convert::AsRef::<
                ::std::path::Path,
            >::as_ref(&__sponge_arg_2);
            let __stem = __p.file_stem().unwrap_or_default().to_string_lossy();
            let __ext = __p.extension().unwrap_or_default().to_string_lossy();
            let __dir = __p.parent().unwrap_or_else(|| ::std::path::Path::new("."));
            __dir
                .join(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}.tmp.{1}", __stem, __ext))
                    }),
                )
        };
        match aggregate_parquets(
            __sponge_arg_0,
            __sponge_arg_1,
            __sponge_tmp_2.as_path(),
        ) {
            Ok(__sponge_ok) => {
                ::std::fs::rename(
                    &__sponge_tmp_2,
                    ::std::convert::AsRef::<::std::path::Path>::as_ref(&__sponge_arg_0),
                )?;
                Ok(__sponge_ok)
            }
            Err(__sponge_err) => Err(__sponge_err),
        }
    }?;
    let original = original_path.as_path();
    let incoming = incoming_path.as_path();
    let output = output_path.as_path();
    {
        let __sponge_arg_0 = original;
        let __sponge_arg_1 = incoming;
        let __sponge_arg_2 = output;
        let __sponge_tmp_2: ::std::path::PathBuf = {
            let __p: &::std::path::Path = ::std::convert::AsRef::<
                ::std::path::Path,
            >::as_ref(&__sponge_arg_2);
            let __stem = __p.file_stem().unwrap_or_default().to_string_lossy();
            let __ext = __p.extension().unwrap_or_default().to_string_lossy();
            let __dir = __p.parent().unwrap_or_else(|| ::std::path::Path::new("."));
            __dir
                .join(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0}.tmp.{1}", __stem, __ext))
                    }),
                )
        };
        match aggregate_parquets(
            __sponge_arg_0,
            __sponge_arg_1,
            __sponge_tmp_2.as_path(),
        ) {
            Ok(__sponge_ok) => {
                ::std::fs::rename(
                    &__sponge_tmp_2,
                    ::std::convert::AsRef::<::std::path::Path>::as_ref(&__sponge_arg_0),
                )?;
                Ok(__sponge_ok)
            }
            Err(__sponge_err) => Err(__sponge_err),
        }
    }?;
    Ok(())
}
fn main() {
    let _ = run;
}
