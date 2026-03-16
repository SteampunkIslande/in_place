use in_place::sponge;
use std::path::Path;

#[derive(Debug)]
struct PipelineError {}

fn aggregate_parquets(
    _original: &Path,
    _incoming: &Path,
    _output: &Path,
) -> Result<(), PipelineError> {
    Ok(())
}

fn run() -> Result<(), PipelineError> {
    sponge!(aggregate_parquets(< "original.parquet", "incoming.parquet".as_ref(), > "output.parquet"))
}

fn main() {
    let _ = run;
}
