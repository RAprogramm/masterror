// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf}
};

fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, files)?;
        } else if path.extension() == Some(OsStr::new("rs")) {
            files.push(path);
        }
    }
    Ok(())
}

#[test]
fn prohibits_direct_result_app_error_usage() {
    let src_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files).expect("collect Rust sources");

    let mut offenders = Vec::new();

    for path in &files {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));

        for (idx, line) in content.lines().enumerate() {
            if line.contains("Result<")
                && line.contains("AppError")
                && !line.contains("AppResult<")
            {
                if path.file_name() == Some(OsStr::new("app_error.rs"))
                    && line.contains("pub type AppResult")
                {
                    continue;
                }
                offenders.push(format!("{}:{}", path.display(), idx + 1));
            }
        }
    }

    if !offenders.is_empty() {
        panic!(
            "Found direct `Result<_, AppError>` usage; replace with `AppResult<_>`: {}",
            offenders.join(", ")
        );
    }
}
