use in_place::sponge;
use std::io::Result;
use std::path::Path;
fn aggregate_parquets(_original: &Path, _incoming: &Path, _output: &Path) -> Result<()> {
    Ok(())
}
fn run() -> Result<()> {
    {
        let __sponge_tmp_2: ::std::path::PathBuf = {
            let __p: &::std::path::Path = ::std::convert::AsRef::<
                ::std::path::Path,
            >::as_ref("original.parquet");
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
            "original.parquet",
            "incoming.parquet",
            __sponge_tmp_2.as_path(),
        ) {
            Ok(__sponge_ok) => {
                ::std::fs::rename(&__sponge_tmp_2, "original.parquet")?;
                Ok(__sponge_ok)
            }
            Err(__sponge_err) => Err(__sponge_err),
        }
    }
}
fn main() {
    let _ = run;
}
