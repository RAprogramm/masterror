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
The Cloudflare November 2025 outage affected 330+ datacenters due to a single
.unwrap(). Configuration change exposed an error case that was never handled.
Result: ChatGPT, X, Canva offline for ~3 hours.

Always use proper error propagation with Result and the ? operator.
Implement detailed error messages with map_err().",
        "\
Сбой Cloudflare в ноябре 2025 затронул 330+ дата-центров из-за одного .unwrap().
Изменение конфигурации обнажило случай ошибки, который не был обработан.
Результат: ChatGPT, X, Canva недоступны ~3 часа.

Всегда используйте правильное распространение ошибок с Result и оператором ?.",
        "\
2025년 11월 Cloudflare 장애는 단일 .unwrap()으로 인해 330개 이상의 데이터센터에
영향을 미쳤습니다. 구성 변경으로 처리되지 않은 오류 케이스가 노출되었습니다."
    ),
    good_example: r#"let config = Config::from_file("config.toml")
    .map_err(|e| format!("Failed to load config: {}", e))?;"#,
    bad_example:  r#"let config = Config::from_file("config.toml").unwrap();"#,
    source:       "https://github.com/RAprogramm/RustManifest#6-panic-avoidance-in-production"
};
