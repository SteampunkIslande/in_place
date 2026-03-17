use in_place_macro::sponge;
use std::io::Result;
use std::path::Path;

fn aggregate_parquets(_original: &Path, _incoming: &Path, _output: &Path) -> Result<()> {
    Ok(())
}

fn run() -> Result<()> {
    // Wrong syntax with keyword arguments style
    sponge!(aggregate_parquets(
        original = Path::new("original.parquet"),
        incoming = Path::new("incoming.parquet"),
        output = Path::new("original.parquet")
    ), output 42 original)?;

    let original = Path::new("original.parquet");
    let incoming = Path::new("incoming.parquet");
    let output = Path::new("original.parquet");

    // Wrong syntax with implicit keywords (only works if you use identifiers as arguments)
    sponge!(aggregate_parquets(original, incoming, Path::new("original.parquet")), output overwrites original)?;

    // Syntax with implicit keywords (only works if you use identifiers as arguments)
    sponge!(aggregate_parquets(original, incoming, output), output blabla original)?;
    Ok(())
}

fn main() {
    let _ = run;
}
