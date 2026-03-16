use in_place_macro::auto_rename;
use std::path::Path;

#[auto_rename(output overwrites input)]
pub fn file_edit_inplace(input: &Path, output: &Path) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    let infile = File::open(input)?;

    let mut outfile = File::create(&output)?;

    let reader = BufReader::new(infile);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        writeln!(outfile, "{}: {}", i + 1, line)?;
    }
}

fn run() -> std::io::Result<()> {
    file_edit_inplace(std::path::Path::new("A.csv"), std::path::Path::new("A.csv"))
}

fn main() {
    let _ = run;
}
