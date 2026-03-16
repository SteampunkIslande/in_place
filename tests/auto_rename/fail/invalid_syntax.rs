use in_place::auto_rename;

#[auto_rename(output input)]
pub fn file_edit_inplace(input: &str, output: &str) -> std::io::Result<()> {
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
    file_edit_inplace("A.csv", "A.csv")
}

fn main() {
    let _ = run;
}
