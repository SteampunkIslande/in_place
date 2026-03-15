use in_place::in_place;
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
    in_place!(f1 = "original.parquet", aggregate_parquets($f1i, "incoming.parquet".as_ref(), $f1o))
}

fn main() {
    let _ = run;
}
