// SPDX-FileCopyrightText: 2025-2026 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! RA020: Flatten nested control flow

use crate::errors::raprogramm::{BestPractice, LocalizedText, PracticeCategory};

pub static ENTRY: BestPractice = BestPractice {
    code:         "RA020",
    title:        LocalizedText::new(
        "Flatten nested control flow",
        "Уменьшайте вложенность управляющих структур",
        "중첩된 제어 흐름 평탄화"
    ),
    category:     PracticeCategory::Idiomatic,
    explanation:  LocalizedText::new(
        "\
Deep nesting (>3-4 levels) makes code hard to read and reason about.
Each level of indentation adds cognitive load.

Techniques to flatten:
- Early returns with guard clauses
- Extract nested logic into separate functions
- Use `match` with pattern guards instead of nested if/else
- Use `?` operator for error propagation
- Use iterator combinators instead of nested loops",
        "\
Глубокая вложенность (>3-4 уровней) делает код трудным для чтения.
Каждый уровень отступа добавляет когнитивную нагрузку.

Техники для уменьшения вложенности:
- Ранний возврат с guard clauses
- Выделение вложенной логики в отдельные функции
- Использование `match` с pattern guards вместо вложенных if/else
- Оператор `?` для propagation ошибок",
        "\
깊은 중첩(3-4 레벨 이상)은 코드를 읽고 이해하기 어렵게 만듭니다.
각 들여쓰기 수준은 인지 부하를 추가합니다."
    ),
    good_example: r#"fn process(data: Option<Data>) -> Result<Output, Error> {
    let data = data.ok_or(Error::NoData)?;

    if !data.is_valid() {
        return Err(Error::Invalid);
    }

    let result = transform(data)?;
    Ok(result)
}"#,
    bad_example:  r#"fn process(data: Option<Data>) -> Result<Output, Error> {
    if let Some(data) = data {
        if data.is_valid() {
            if let Ok(intermediate) = step1(data) {
                if let Ok(result) = step2(intermediate) {
                    return Ok(result);
                }
            }
        }
    }
    Err(Error::Failed)
}"#,
    source:       "https://github.com/RAprogramm/RustManifest#code-structure"
};
