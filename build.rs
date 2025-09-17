use std::{env, error::Error, path::PathBuf, process};

#[path = "build/readme.rs"]
mod readme;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=README.template.md");
    println!("cargo:rerun-if-changed=build/readme.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    readme::sync_readme(&manifest_dir)?;
    Ok(())
}
