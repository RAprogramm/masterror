// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA002: Use ? operator for error propagation

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA002",
    title:        LocalizedText::new(
        "Use ? operator for error propagation",
        "Используйте оператор ? для распространения ошибок",
        "오류 전파에 ? 연산자 사용"
    ),
    category:     PracticeCategory::ErrorHandling,
    explanation:  LocalizedText::new(
        "\
The ? operator is the idiomatic way to handle errors in Rust.
It automatically converts errors and propagates them up the call stack.

Use ok_or() or ok_or_else() to convert Option to Result with meaningful messages.",
        "\
Оператор ? — идиоматический способ обработки ошибок в Rust.
Он автоматически конвертирует ошибки и распространяет их вверх по стеку вызовов.",
        "\
? 연산자는 Rust에서 오류를 처리하는 관용적인 방법입니다."
    ),
    good_example: r#"let value = some_option.ok_or("Expected a value")?;
let data = fetch_data().map_err(|e| AppError::Network(e))?;"#,
    bad_example:  r#"let value = some_option.unwrap();
let data = fetch_data().expect("fetch failed");"#,
    source:       "https://github.com/RAprogramm/RustManifest#5-best-practices"
};
