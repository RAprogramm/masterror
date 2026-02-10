// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0462: found staticlib instead of rlib/dylib

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0462",
    title:       LocalizedText::new(
        "Found staticlib instead of rlib or dylib",
        "Найден staticlib вместо rlib или dylib",
        "rlib 또는 dylib 대신 staticlib이 발견됨"
    ),
    category:    Category::Linking,
    explanation: LocalizedText::new(
        "\
An attempt was made to link a crate compiled as staticlib from another
Rust crate. A staticlib is a static library format intended only for
linking with non-Rust applications (like C programs). It is not a valid
crate type for Rust-to-Rust linking.

Valid Rust crate types for Rust linking: rlib or dylib.",
        "\
Попытка связать крейт, скомпилированный как staticlib, из другого
крейта Rust. Staticlib - формат статической библиотеки, предназначенный
только для связывания с не-Rust приложениями (например, C программами).

Допустимые типы крейтов для связывания Rust: rlib или dylib.",
        "\
다른 Rust 크레이트에서 staticlib로 컴파일된 크레이트를 연결하려고
시도했습니다. staticlib은 비Rust 애플리케이션(C 프로그램 등)과
연결하기 위한 정적 라이브러리 형식입니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Recompile as rlib or dylib",
            "Перекомпилировать как rlib или dylib",
            "rlib 또는 dylib로 재컴파일"
        ),
        code:        "#![crate_type = \"rlib\"]\n// or in Cargo.toml:\n// [lib]\n// crate-type = [\"rlib\"]"
    }],
    links:       &[
        DocLink {
            title: "Linkage",
            url:   "https://doc.rust-lang.org/reference/linkage.html"
        },
        DocLink {
            title: "Error Code Reference",
            url:   "https://doc.rust-lang.org/error_codes/E0462.html"
        }
    ]
};
