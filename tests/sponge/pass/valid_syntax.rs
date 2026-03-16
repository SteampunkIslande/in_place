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

    // Syntax with keyword arguments style
    sponge!(aggregate_parquets(
        original = original_path.as_path(),
        incoming = incoming_path.as_path(),
        output = output_path.as_path()
    ), output overwrites original)?;

    let original = original_path.as_path();
    let incoming = incoming_path.as_path();
    let output = output_path.as_path();
    // Syntax with implicit keywords (only works if you use identifiers as arguments)
    sponge!(aggregate_parquets(original, incoming, output), output overwrites original)?;
    Ok(())
}

fn main() {
    let _ = run;
}
