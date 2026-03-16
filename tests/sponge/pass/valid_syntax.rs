use in_place::sponge;
use std::io::Result;
use std::path::Path;

fn aggregate_parquets(_original: &Path, _incoming: &Path, _output: &Path) -> Result<()> {
    Ok(())
}

fn run() -> Result<()> {
    sponge!(aggregate_parquets("original.parquet", "incoming.parquet", overwrites "original.parquet"))
}

fn main() {
    let _ = run;
}
