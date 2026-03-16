/// Compile-tests for auto_rename attribute macro
#[cfg(test)]
mod sponge_try_build_test {
    /// Ensure the code compiles with valid syntax
    #[test]
    fn sponge_pass() {
        let t = trybuild::TestCases::new();
        t.pass("tests/sponge/pass/valid_syntax.rs");
    }

    /// Ensure the code doesn't compile with invalid syntax
    #[test]
    fn sponge_fail() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/sponge/fail/invalid_syntax.rs");
    }
}

/// Ensure the code expands to expected output
#[cfg(test)]
mod sponge_macrotest_test {

    #[test]
    fn sponge_expands() {
        macrotest::expand("tests/sponge/pass/valid_syntax.rs");
    }
}

#[cfg(test)]
mod sponge_unit_test {

    use in_place::sponge;

    pub fn file_edit(input: &std::path::Path, output: &std::path::Path) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::{BufRead, BufReader, Write};
        let infile = File::open(input)?;

        let mut outfile = File::create(output)?;

        let reader = BufReader::new(infile);

        for (i, line) in reader.lines().enumerate() {
            let line = line?;
            writeln!(outfile, "{}: {}", i + 1, line)?;
        }
        Ok(())
    }

    fn create_temp_file_with_content(content: &str) -> tempfile::NamedTempFile {
        use std::io::Write;
        let mut file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes())
            .expect("Failed to write to temp file");
        file.flush().expect("Failed to flush temp file");
        file
    }

    /// Sans la macro, appeler file_edit avec le même chemin en entrée et en sortie
    /// corrompt le fichier : File::create tronque d'abord le fichier, puis
    /// File::open lit un fichier vide, ce qui produit une sortie vide.
    #[test]
    fn test_same_file_without_macro_corrupts_file() {
        let temp = create_temp_file_with_content(
            (0..10)
                .map(|i| format!("This is line {}", i))
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        );
        let path = temp.path();

        file_edit(path, path).expect("file_edit should not return an error");

        let content = std::fs::read_to_string(path).unwrap();
        assert!(
            content.is_empty(),
            "Without the sponge macro, the file should be corrupted (empty): got {:?}",
            content
        );
    }

    /// Avec la macro, appeler file_edit_inplace avec le même chemin en entrée et
    /// en sortie écrit d'abord dans un fichier temporaire, puis le renomme à la
    /// place de l'original : le contenu est modifié comme attendu.
    #[test]
    fn test_same_file_with_macro_edits_correctly() {
        let temp = create_temp_file_with_content(
            (0..10)
                .map(|i| format!("This is line {}", i))
                .collect::<Vec<String>>()
                .join("\n")
                .as_str(),
        );
        let path = temp.path();

        (|| -> std::io::Result<()> { sponge!(file_edit(path, overwrites path)) })()
            .expect("file_edit_inplace should not return an error");

        let content = std::fs::read_to_string(path).unwrap();
        assert_eq!(
            content,
            format!(
                "{}\n",
                (0..10)
                    .map(|i| format!("{}: This is line {}", i + 1, i))
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_str(),
            ),
            "With macro, the file should be correctly modified"
        );
    }
}
