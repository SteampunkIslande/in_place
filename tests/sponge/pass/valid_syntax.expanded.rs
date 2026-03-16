use in_place::sponge;
use std::io::Result;
use std::path::Path;
fn aggregate_parquets(_original: &Path, _incoming: &Path, _output: &Path) -> Result<()> {
    Ok(())
}
fn run() -> Result<()> {
    {
        let __sponge_arg_0 = Path::new("original.parquet");
        let __sponge_arg_1 = Path::new("incoming.parquet");
        let __sponge_arg_2 = Path::new("original.parquet");
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
    let original = Path::new("original.parquet");
    let incoming = Path::new("incoming.parquet");
    let output = Path::new("original.parquet");
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
