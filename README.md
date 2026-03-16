Macros for in-place file manipulations
===========================
![Repo](https://img.shields.io/badge/github-in_place-blue?style=for-the-badge&logo=github&link=https%3A%2F%2Fgithub.com%2FSteampunkIslande%2Fin_place)
![Build status](https://img.shields.io/github/actions/workflow/status/SteampunkIslande/in_place/ci.yaml?style=for-the-badge)

Are you tired of always having to write that same old code, over and over again, everytime you need a file to be edited in-place in a streaming fashion ?

This is the point of the [sponge](https://joeyh.name/code/moreutils/#:~:text=Probably%20the%20most%20general%20purpose%20tool%20in%20moreutils%20so%20far%20is%20sponge%281%29%2C%20which%20lets%20you%20do%20things%20like%20this) command that comes with the [moreutils](https://joeyh.name/code/moreutils/) package.

Using this crate in rust, you now have two ways to write code that edits a file in-place.

# Disclaimer

This crate is **not** production-ready yet, please feel free to open an issue or a PR if you find either a bug, a solution to it, or both.

# Why use this crate

There are three typical scenarios in which you may wish you could edit a file in-place.

- You're writing your own, brand new function, and realize that your input **file** should be overwritten by your output **file**. In this case, don't use this crate! Instead, use [this one](https://crates.io/crates/in-place).
- You've already written tens of functions that all take two different files as input and output, and for some reason, you're only realizing now that you need the output to overwrite the input. That's fine, you can use this crate. And more specifically, I would advise you to pick the [`#[auto_rename]` macro_attribute](#using-the-auto_rename-macro_attribute) which I find more convenient.
- You're using someone else's crate, and their API uses functions that take `Path`s as inputs and as outputs, but there is no other way of safely editing the files in-place, besides the tedious, manual, rename before, rename after, don't mess up the order. For this use case, since you cannot decorate their functions with a macro_attribute (unless you wrap them yourself), you can still use this [convenient function-like macro](#using-the-sponge-function-like-macro).

# Using the `#[auto_rename]` macro_attribute

Say you define a function that takes input and output as files:

```rust
use in_place_macro::auto_rename;

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
    file_edit_inplace("A.csv", "A.csv")
}
```

When you call this function with the same path for input and output (e.g. `file_edit_inplace("A.csv", "A.csv")`), the macro will:

1. Create a temporary file (e.g., `A.tmp.csv`)
2. Write the output to the temporary file
3. Rename the temporary file to replace the original file

This way, you never have a half-written file if something goes wrong during the write.

# Using the `sponge!` function-like macro

The `sponge!` macro is designed for when you have an existing function that you want to use in-place. It works similarly to the `sponge` command from moreutils: it evaluates the expression, writes the result to a temporary file, and then atomically replaces the original file with the temporary one.

## With keyword arguments

```rust
use in_place_macro::sponge;
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

    Ok(())
}
```

## With implicit keywords (identifiers only)

If you use simple identifiers as arguments, you can use a shorter syntax:

```rust
use in_place_macro::sponge;
use std::io::Result;
use std::path::Path;
use std::path::PathBuf;

fn process_file(original: &Path, incoming: &Path, output: &Path) -> Result<()> {
    Ok(())
}

fn run() -> Result<()> {
    let original = PathBuf::from("file.txt");
    let incoming = PathBuf::from("changes.txt");
    let output = PathBuf::from("file.txt");

    // Shorthand syntax using identifiers
    sponge!(process_file(original, incoming, output), output overwrites original)?;

    Ok(())
}
```

In both cases, the `sponge!` macro will:
1. Evaluate your function with a temporary file path as the output argument
2. If the function succeeds, atomically rename the temporary file to replace the original
3. If the function fails, clean up the temporary file and return the error
