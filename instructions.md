La macro `in_place!` permet de modifier un fichier sur place, à la manière d'un `sed -i` ou d'un `sponge` (de moreutils).

Exemple de fonction qui s'y prête:

```rust
fn aggregate_parquets<P: AsRef<Path>>(original_aggregate:P, incoming:P, aggregate:P)->Result<(),SomeError>{...}
```

Dans cette situation, la fonction ne se préoccupe pas de savoir si le fichier aggregate en sortie a le même nom que original_aggregate. Comme elle lit original_aggregate, écire directement dans aggregate risquerait de corrompre le fichier en cours de lecture.

Pour y remédier, la macro `in_place!` s'occupe de générer le boilerplate que demanderait une invocation propre de aggregate_parquets, à savoir:

```rust
in_place!(f1 = "original.parquet", aggregate_parquets($f1i, "incoming.parquet", $f1o))
```

qui va générer le code suivant:

```rust
{
    let f1_input = AsRef::<Path>::as_ref("original.parquet");
    let f1_inter = AsRef::<Path>::as_ref("original.tmp.parquet");
    aggregate_parquets(f1_input, "incoming.parquet", f1_inter)?;
    fs::rename(f1_inter, f1_input).expect("Cannot rename file!");
    Ok(())
}
```

Une variante:

```rust
in_place!(f1 = "original.parquet", f2 = "other.csv", some_function($f1i, "incoming.parquet", $f1o, $f2o, $f2i))
```

qui va générer le code suivant:

```rust
{
    let f1_input = AsRef::<Path>::as_ref("original.parquet");
    let f1_inter = AsRef::<Path>::as_ref("original.tmp.parquet");
    let f2_input = AsRef::<Path>::as_ref("other.csv");
    let f2_inter = AsRef::<Path>::as_ref("other.tmp.csv");
    some_function(f1_input, "incoming.parquet", f1_inter, f2_inter, f2_input)?;
    fs::rename(f1_inter, f1_input).expect("Cannot rename file!");
    fs::rename(f2_inter, f2_input).expect("Cannot rename file!");
    Ok(())
}
```