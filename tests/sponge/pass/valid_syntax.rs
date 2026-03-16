use in_place::sponge;
use std::io::Result;
use std::path::Path;

fn aggregate_parquets(_original: &Path, _incoming: &Path, _output: &Path) -> Result<()> {
    Ok(())
}

fn run() -> Result<()> {
    // Syntax with keyword arguments style
    sponge!(aggregate_parquets(
        original = Path::new("original.parquet"),
        incoming = Path::new("incoming.parquet"),
        output = Path::new("original.parquet")
    ), output overwrites original)?;

    let original = Path::new("original.parquet");
    let incoming = Path::new("incoming.parquet");
    let output = Path::new("original.parquet");

    // Syntax with implicit keywords (only works if you use identifiers as arguments)
    sponge!(aggregate_parquets(original, incoming, output), output overwrites original)?;
    Ok(())
}

fn main() {
    let _ = run;
}
