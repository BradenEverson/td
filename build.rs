use std::{env, fs, io::Write, path::PathBuf};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let units_dir = PathBuf::from(&manifest_dir).join("units");

    let mut contents = vec![];
    if units_dir.exists() && units_dir.is_dir() {
        for entry in fs::read_dir(units_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                let content = fs::read_to_string(path).unwrap();
                contents.push(content);
            }
        }
    } else {
        panic!("The 'units' directory does not exist.");
    }

    let out_file_path = PathBuf::from(&manifest_dir).join("src/game/card_gen/cards.rs");
    let mut out_file = fs::File::create(out_file_path).unwrap();

    writeln!(out_file, "/// THIS FILE IS AUTOGENERATED BY BUILD.RS").unwrap();
    writeln!(
        out_file,
        "/// TO ADD NEW UNITS, ADD A NEW FILE TO THE UNITS DIRECTORY"
    )
    .unwrap();

    writeln!(out_file, "pub static CARDS: &[&str] = &[").unwrap();
    for content in &contents {
        writeln!(out_file, "\tr#\"{}\"#,", content.trim()).unwrap();
    }
    writeln!(out_file, "];").unwrap();

    println!("cargo:rerun-if-changed=units");
    println!("cargo:rerun-if-changed=build.rs");
}