// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! E0457: plugin only in rlib format

use crate::errors::{Category, DocLink, ErrorEntry, FixSuggestion, LocalizedText};

pub static ENTRY: ErrorEntry = ErrorEntry {
    code:        "E0457",
    title:       LocalizedText::new(
        "Plugin only found in rlib format, dylib required",
        "Плагин найден только в формате rlib, требуется dylib",
        "플러그인이 rlib 형식으로만 발견됨, dylib 필요"
    ),
    category:    Category::Attributes,
    explanation: LocalizedText::new(
        "\
A plugin crate was compiled to the statically-linked rlib format instead
of the required dynamically-linked dylib format. The Rust compiler's plugin
interface requires plugins to be compiled as dynamically-linked libraries.

Note: This error is no longer emitted by modern compilers as the plugin
system has been deprecated in favor of procedural macros.",
        "\
Крейт плагина скомпилирован в статически связываемый формат rlib вместо
требуемого динамически связываемого формата dylib. Интерфейс плагинов
компилятора Rust требует компиляции плагинов как динамических библиотек.

Примечание: эта ошибка больше не выдаётся, так как система плагинов
устарела в пользу процедурных макросов.",
        "\
플러그인 크레이트가 필요한 동적 링크 dylib 형식 대신 정적 링크
rlib 형식으로 컴파일되었습니다."
    ),
    fixes:       &[FixSuggestion {
        description: LocalizedText::new(
            "Compile the plugin as dylib",
            "Скомпилировать плагин как dylib",
            "플러그인을 dylib로 컴파일"
        ),
        code:        "# In Cargo.toml:\n[lib]\ncrate-type = [\"dylib\"]"
    }],
    links:       &[DocLink {
        title: "Error Code Reference",
        url:   "https://doc.rust-lang.org/error_codes/E0457.html"
    }]
};
