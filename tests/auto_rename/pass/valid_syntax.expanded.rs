use in_place::auto_rename;
use std::path::Path;
pub fn file_edit_inplace(input: &Path, output: &Path) -> std::io::Result<()> {
    let output: ::std::path::PathBuf = {
        let __p: &::std::path::Path = ::std::convert::AsRef::<
            ::std::path::Path,
        >::as_ref(output);
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
    {
        use std::fs::File;
        use std::io::{BufRead, BufReader, Write};
        let infile = File::open(input)?;
        let mut outfile = File::create(&output)?;
        let reader = BufReader::new(infile);
        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            outfile.write_fmt(format_args!("{0}: {1}\n", i + 1, line))?;
        }
    }
    ::std::fs::rename(&output, input)?;
    Ok(())
}
fn run() -> std::io::Result<()> {
    file_edit_inplace(std::path::Path::new("A.csv"), std::path::Path::new("A.csv"))
}
fn main() {
    let _ = run;
}
