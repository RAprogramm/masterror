use std::{
    env,
    path::{Path, PathBuf},
    process
};

use crate::readme::{sync_readme, verify_readme_relaxed};

#[path = "build/readme.rs"]
mod readme;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=README.template.md");
    println!("cargo:rerun-if-changed=build/readme.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    // Явный флаг, чтобы где угодно ослабить проверку (ремень безопасности для
    // CI/verify)
    if allow_readme_drift() {
        return Ok(());
    }

    // В tarball-е (cargo package --verify) или вообще без .git — проверяем мягко и
    // НЕ валимся.
    if is_packaged_manifest(&manifest_dir) || !has_git_anywhere(&manifest_dir) {
        if let Err(err) = verify_readme_relaxed(&manifest_dir) {
            println!("cargo:warning={err}");
        }
        return Ok(());
    }

    // В нормальном git-рабочем дереве — синхронизируем (жёсткий режим).
    sync_readme(&manifest_dir)?;
    Ok(())
}

// Твоя прежняя эвристика: target/package/... => packaged
fn is_packaged_manifest(manifest_dir: &Path) -> bool {
    let mut seen_target = false;
    for comp in manifest_dir.components() {
        match comp {
            std::path::Component::Normal(name) => {
                if seen_target && name == "package" {
                    return true;
                }
                seen_target = name == "target";
            }
            _ => {
                seen_target = false;
            }
        }
    }
    false
}

// Проверяем .git по цепочке вверх (workspace корень часто выше
// crate-директории)
fn has_git_anywhere(mut dir: &Path) -> bool {
    loop {
        if dir.join(".git").exists() {
            return true;
        }
        match dir.parent() {
            Some(p) => dir = p,
            None => return false
        }
    }
}

fn allow_readme_drift() -> bool {
    has_env("MASTERROR_ALLOW_README_DRIFT") || has_env("MASTERROR_SKIP_README_CHECK")
}

fn has_env(name: &str) -> bool {
    env::var_os(name).map(|v| !v.is_empty()).unwrap_or(false)
}
