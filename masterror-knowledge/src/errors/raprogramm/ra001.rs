// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA001: Never use unwrap() in production code

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA001",
    title:        LocalizedText::new(
        "Never use unwrap() in production code",
        "Никогда не используйте unwrap() в продакшене",
        "프로덕션 코드에서 unwrap() 사용 금지"
    ),
    category:     PracticeCategory::ErrorHandling,
    explanation:  LocalizedText::new(
        "\
unwrap() calls panic!() if the value is None or Err.
In production this crashes the entire service.
Use ? operator to propagate errors or handle them explicitly with match/map_err.",
        "\
unwrap() вызывает panic!() если значение None или Err.
В продакшене это приводит к падению всего сервиса.
Используйте оператор ? для пробрасывания ошибок или обрабатывайте явно через match/map_err.",
        "\
unwrap()은 값이 None이거나 Err이면 panic!()을 호출합니다.
프로덕션에서는 전체 서비스가 중단됩니다.
? 연산자로 에러를 전파하거나 match/map_err로 명시적으로 처리하세요."
    ),
    good_example: r#"let config = Config::from_file("config.toml")
    .map_err(|e| format!("Failed to load config: {}", e))?;"#,
    bad_example:  r#"let config = Config::from_file("config.toml").unwrap();"#,
    source:       "https://github.com/RAprogramm/RustManifest#6-panic-avoidance-in-production"
};
