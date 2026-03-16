use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

use in_place_macro::auto_rename;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate an example file
    Generate {
        /// File to create
        file: String,
    },

    /// Run the line-numbering program
    Run {
        /// Use the inplace-safe implementation (intended for macro)
        #[arg(long, short)]
        inplace: bool,

        /// File to operate on
        file: String,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { file } => {
            generate_example(&file)?;
            println!("Example file generated: {}", file);
        }

        Commands::Run { inplace, file } => {
            if inplace {
                numbered_copy_inplace(&file, &file)?;
            } else {
                numbered_copy_plain(&file, &file)?;
            }
        }
    }

    Ok(())
}

fn generate_example(path: &str) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    for i in 0..10 {
        writeln!(f, "This is example line {}", i)?;
    }
    Ok(())
}

pub fn numbered_copy_plain(input: &str, output: &str) -> std::io::Result<()> {
    let infile = File::open(input)?;
    let mut outfile = File::create(output)?;

    let reader = BufReader::new(infile);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        writeln!(outfile, "{}: {}", i + 1, line)?;
    }

    Ok(())
}

#[auto_rename(output to input)]
pub fn numbered_copy_inplace(input: &str, output: &str) -> std::io::Result<()> {
    let infile = File::open(input)?;

    eprintln!("Using {:?}", output);
    let mut outfile = File::create(&output)?;

    let reader = BufReader::new(infile);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        writeln!(outfile, "{}: {}", i + 1, line)?;
    }
}
